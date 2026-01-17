#![forbid(unsafe_code)]
#![deny(unused_crate_dependencies)]
#![warn(clippy::std_instead_of_core)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unsafe_derive_deserialize)]

use crate::game::{Chapter, Content, GameLanguage, Level};
use crate::msg::MsgLanguage;
use crate::select::{Item, Number, Randomize, Select, SelectStore};
use core::ops::Deref;
use getrandom as _; // is only used indirectly through rand but is required to activate feature
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use yew::{Html, function_component, html};
use yew_bootstrap::component::form::{FormControl, FormControlType};
use yew_bootstrap::component::{Alert, Button};
use yew_bootstrap::icons::BI;
use yew_bootstrap::util::Color;
use yewdux::{Dispatch, Reducer, Store, use_store};

pub(crate) mod game;
pub(crate) mod msg;
pub(crate) mod select;
mod setup;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, Store)]
#[store(storage = "local", storage_tab_sync)]
pub(crate) struct Settings {
    pub(crate) game_language: GameLanguage,
    pub(crate) msg: MsgLanguage,
    pub(crate) content: HashSet<Content>,
    pub(crate) preset: bool,
    pub(crate) players: Number,
    pub(crate) preset_content: Content,
    pub(crate) preset_chapter: Chapter,
    pub(crate) new_list_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            game_language: GameLanguage::En,
            msg: MsgLanguage::default(),
            content: HashSet::from([Content::Core]),
            preset: true,
            players: Number::Five,
            preset_content: Content::Core,
            preset_chapter: Chapter(1),
            new_list_mode: true,
        }
    }
}

impl Reducer<Settings> for GameLanguage {
    fn apply(self, mut rc_settings: Rc<Settings>) -> Rc<Settings> {
        let settings = Rc::make_mut(&mut rc_settings);
        settings.game_language = self;
        rc_settings
    }
}

impl Reducer<Settings> for MsgLanguage {
    fn apply(self, mut rc_settings: Rc<Settings>) -> Rc<Settings> {
        let settings = Rc::make_mut(&mut rc_settings);
        settings.msg = self;
        rc_settings
    }
}

impl Reducer<Settings> for Content {
    fn apply(self, mut rc_settings: Rc<Settings>) -> Rc<Settings> {
        let settings = Rc::make_mut(&mut rc_settings);
        if settings.content.contains(&self) {
            settings.content.remove(&self);
        } else {
            settings.content.insert(self);
        }
        Dispatch::<SelectStore>::global().reduce_mut(|s| s.adjust_content(settings));
        if settings.preset && !settings.content.contains(&Content::Core) {
            settings.preset = true;
        }
        rc_settings
    }
}

impl Reducer<Settings> for bool {
    fn apply(self, mut rc_settings: Rc<Settings>) -> Rc<Settings> {
        let settings = Rc::make_mut(&mut rc_settings);
        settings.preset = self;
        rc_settings
    }
}

struct Players(Number);
impl Reducer<Settings> for Players {
    fn apply(self, mut rc_settings: Rc<Settings>) -> Rc<Settings> {
        let settings = Rc::make_mut(&mut rc_settings);
        settings.players = self.0;
        rc_settings
    }
}

pub(crate) struct ToggleListType;
impl Reducer<Settings> for ToggleListType {
    fn apply(self, mut rc_settings: Rc<Settings>) -> Rc<Settings> {
        let settings = Rc::make_mut(&mut rc_settings);
        settings.new_list_mode = !settings.new_list_mode;
        rc_settings
    }
}

#[function_component]
fn App() -> Html {
    let (settings, dispatch) = use_store::<Settings>();
    let (select, select_dispatch) = use_store::<SelectStore>();

    let game_languages = GameLanguage::iter()
        .zip(GameLanguage::names())
        .map(|(l, n)| {
            let id = format!("gameLanguage:{n}");
            let onchange = dispatch.apply_callback(move |_| l);
            html! {
                <>
                         <input
                             type="radio"
                             class="btn-check"
                             name="gameLanguage"
                             id={id.clone()}
                             autocomplete="off"
                             checked={settings.game_language == l}
                             onchange={onchange}
                         />
                         <label class="btn btn-outline-primary" for={id}>{n}</label>
                </>
            }
        });

    let msg_languages = MsgLanguage::iter().zip(MsgLanguage::names()).map(|(l, n)| {
        let id = format!("msgLanguage:{n}");
        let onchange = dispatch.apply_callback(move |_| l);
        html! {
            <>
                     <input
                         type="radio"
                         class="btn-check"
                         name="msgLanguage"
                         id={id.clone()}
                         autocomplete="off"
                         checked={settings.msg == l}
                         onchange={onchange}
                     />
                     <label class="btn btn-outline-primary" for={id}>{n}</label>
            </>
        }
    });

    let mut contents = Content::iter()
        .map(|content| (content, content.name(settings.game_language)))
        .collect::<Vec<_>>();
    contents.sort_by_key(|(c, _n)| c.order_name(settings.game_language));
    let has_unknown = contents.iter().any(|(_, n)| n.ends_with('*'));
    let contents = contents.into_iter().map(|(content, name)| {
        let id = format!("content:{}", content.into());
        let onchange = dispatch.apply_callback(move |_| content);
        html! {
            <div class="form-check">
              <input
                type="checkbox"
                class="form-check-input"
                value=""
                id={id.clone()}
                checked={settings.content.contains(&content)}
                onchange={onchange}
              />
              <label class="form-check-label" for={id}>
                {name}
              </label>
            </div>
        }
    });

    let list = if settings.new_list_mode {
        render_list_new(&settings, select.output.borrow())
    } else {
        select
            .output
            .borrow()
            .iter()
            .map(|item| render_list_old(&settings, item))
            .collect::<Vec<_>>()
    };
    let randomize = select_dispatch.apply_callback(|_| Randomize);
    let toggle_list_type = dispatch.apply_callback(|_| ToggleListType);

    let click_use_preset = dispatch.apply_callback(|_| true);
    let click_dont_use_preset = dispatch.apply_callback(|_| false);

    let players = Number::iter().map(|p| {
        let id = format!("numPlayers:{}", p.as_str());
        let onchange = dispatch.apply_callback(move |_| Players(p));
        html! {
            <>
                     <input
                         type="radio"
                         class="btn-check"
                         name="numPlayers"
                         id={id.clone()}
                         autocomplete="off"
                         checked={settings.players == p}
                         onchange={onchange}
                     />
                     <label class="btn btn-outline-primary" for={id}>{p.as_str()}</label>
            </>
        }
    });

    html! {
          <div class="app-wrap">
            <nav class="navbar sticky-top bg-body-tertiary">
              <div class="container-fluid">
                <a class="navbar-brand" href="#">
                  {"Unofficial Drunagor Randomizer"}
                </a>

                <div class="navbar-nav">
                  <ul class="nav nav-pills">
                    <div>
                        {"Game language: "}
                        <div class="btn-group" role="group">
                            {for game_languages}
                        </div>
                        {" UI language: "}
                        <div class="btn-group" role="group">
                            {for msg_languages}
                        </div>
                    </div>
                  </ul>
                </div>
              </div>
            </nav>

              <main class="py-4">
                <div class="container">
                  <div class="accordion" id="accordionExample">
                    <div class="accordion-item">
                      <h2 class="accordion-header">
                        <button class="accordion-button" type="button" data-bs-toggle="collapse" data-bs-target="#collapseOne" aria-expanded="true" aria-controls="collapseOne">
                          {"Settings"}
                        </button>
                      </h2>
                      <div id="collapseOne" class={"accordion-collapse collapse show"} data-bs-parent="#accordionExample">
                        <div class="accordion-body">
                          {for contents}
                          if has_unknown {
                            <div>
                              {"* = not translated"}
                            </div>
                          }
        <hr/>
        {"Number of players:"}<br/>
        {for players}
        if settings.content.contains(&Content::Core) {
            <hr/>
            <FormControl
                id={"click_use_preset"}
                ctype={FormControlType::Radio}
                name={"use_preset"}
                checked={settings.preset}
                onclick={click_use_preset}
                label={"Use preset monster setups"}
            />
            <FormControl
                id={"click_dont_use_preset"}
                ctype={FormControlType::Radio}
                name={"dont_use_preset"}
                checked={!settings.preset}
                onclick={click_dont_use_preset}
                label={"Use custom monster setups"}
            />
        }
                        </div>
                      </div>
                    </div>
        if !settings.content.is_empty() {
                    <div class="accordion-item">
                      <h2 class="accordion-header">
                        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#collapseTwo" aria-expanded="false" aria-controls="collapseTwo">
                          {"Selection"}
                        </button>
                      </h2>
                      <div id="collapseTwo" class="accordion-collapse collapse" data-bs-parent="#accordionExample">
                        <div class="accordion-body">
                          <Select/>
        if  false {
            <button class="btn btn-primary" type="button" data-bs-toggle="collapse" data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
      {"Button with data-bs-target"}
    </button>
            }
                        </div>
                      </div>
                    </div>
        }
        if !select.output.borrow().is_empty() || select.setup.is_some() {
                    <div class="accordion-item">
                      <h2 class="accordion-header">
                        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                          {"Monster Setup"}
                        </button>
                      </h2>
                      <div id="collapseThree" class={"accordion-collapse collapse"} data-bs-parent="#accordionExample">
                        <div class="accordion-body">
                            if let Some((content, chapter, name)) = &select.setup {
                                <Alert style={Color::Light}>
                                    {content.name(settings.game_language)}{" - "}
                                    {chapter.0}{" - "}
                                    {name}
                                </Alert>
                            }
                            if select.output.borrow().is_empty() {
                                <Alert style={Color::Secondary}>{"This setup has no monsters"}</Alert>
                            }else{
                                <table class="table">
                                    <tbody>
                                        {for list}
                                    </tbody>
                                </table>
                            }
                            <div>
                            <Button style={Color::Primary} outline={true} onclick={randomize}>{BI::ARROW_COUNTERCLOCKWISE}</Button>
                            <div class="form-check">
                              <input
                                type="checkbox"
                                class="form-check-input"
                                value=""
                                id="toggle-list-type"
                                checked={settings.new_list_mode}
                                onchange={toggle_list_type}
                              />
                              <label class="form-check-label" for="toggle-list-type">
                                {"New list type"}
                              </label>
                            </div>

                            </div>
            if select.output.borrow().iter().any(|item| item.preset) {
                            <div>
                {"* = Preset monsters"}
                            </div>
            }
                        </div>
                      </div>
                    </div>
            }else{
                    <div class="accordion-item" style="display: none">
                      <h2 class="accordion-header">
                        <button class="accordion-button collapsed" type="button" data-bs-toggle="collapse" data-bs-target="#collapseThree" aria-expanded="false" aria-controls="collapseThree">
                          {"Monster Setup"}
                        </button>
                      </h2>
                      <div id="collapseThree" class={"accordion-collapse collapse"} data-bs-parent="#accordionExample">
                        <div class="accordion-body">
                        </div>
                      </div>
                    </div>
            }
                  </div>
                </div>
              </main>

              <nav class="navbar sticky-bottom bg-body-tertiary">
                <div class="container-fluid">
                  <h5 class="mb-0">
                    {"Written by Alex."}
                  </h5>

                  <div class="ms-auto">
                    <a target="_blank" href="https://alexkazik.github.io/boardgames.html">{"My other board game tools"}</a>
                  </div>

                  <div class="ms-auto">
                    {"Favicon by "}<a target="_blank" href="https://boardgameheaven.eu">{"Board Game Heaven"}</a>
                  </div>

                  <div class="ms-auto">
                    {"Version: "}{env!("CARGO_PKG_VERSION")}
                    <a
                      href="https://github.com/alexkazik/spawning-drunagor"
                      target="_blank"
                      class="btn btn-dark btn-sm ms-4"
                    >
                      <i class="bi bi-github"></i>
                      {"Source"}
                    </a>
                  </div>
                </div>
              </nav>
            </div>
      }
}

fn render_list_old(settings: &Rc<Settings>, item: &Item) -> Html {
    if let Some(m) = item.monster {
        let mut fade = "";
        if let Some(n) = item.number
            && n > settings.players
        {
            fade = "opacity: 0.5";
        }
        if item.color == Some(game::Color::Commander) {
            html! {
                <tr>
                    <td style={fade}>
                        {BI::PERSON_WALKING}{item.number.map_or("*",|x|x.as_str())}{" "}
                        {item.color.map_or("",|c|c.short(settings.game_language))}{" - "}
                        {m.name(settings.game_language)}
                        {if item.preset {"*"}else{""}}
                    </td>
                </tr>
            }
        } else {
            html! {
                <tr>
                    <td style={fade}>
                        {BI::PERSON_WALKING}{item.number.map_or("*",|x|x.as_str())}{" "}
                        if let Some(c) = item.color {
                            {c.short(settings.game_language)}{" - "}
                        }
                        {m.name(settings.game_language)}{if item.preset {"*"}else{""}}{" - "}{item.level.name(settings.game_language)}
                    </td>
                </tr>
            }
        }
    } else {
        html! {}
    }
}

fn render_list_new(settings: &Rc<Settings>, output: impl Deref<Target = Vec<Item>>) -> Vec<Html> {
    let mut result = Vec::new();

    let mut todo = output
        .iter()
        .filter(|item| item.monster.is_some())
        .filter(|item| item.number.is_none_or(|n| n <= settings.players))
        .collect::<VecDeque<_>>();
    while let Some(item) = todo.pop_front() {
        let mut items = Vec::new();
        items.push(item);

        if item.number.is_some() {
            // number and monster are set!
            let mut pos = 0;
            while pos < todo.len() {
                let other = todo[pos];
                if item.color == other.color
                    && item.level == other.level
                    && item.monster == other.monster
                    && item.preset == other.preset
                {
                    items.push(other);
                    todo.remove(pos);
                } else {
                    pos += 1;
                }
            }

            let mut is_special = None;

            let icons = items
                .iter()
                .map(|item| {
                    let mut color = item.color;
                    if let Level::Special(Some(special)) = item.level {
                        color = special.color();
                    }
                    if (!item.preset || matches!(item.level, Level::Special(_)))
                        && let Some(color) = color
                    {
                        html! {
                            <div class={format!("box_{}_{}", color.prefix_lower(), item.level.id_lower())}>{color.prefix(settings.game_language)}{item.number.unwrap().as_str()}</div>
                        }
                    } else {
                        is_special = item.monster;
                        html! {
                            <div class={format!("box_{}", item.level.id_lower())}>{BI::PERSON_WALKING}{item.number.unwrap().as_str()}</div>
                        }
                    }
                })
                .collect::<Vec<_>>();

            if item
                .monster
                .unwrap()
                .name(settings.game_language)
                .is_empty()
            {
                result.push(html! {
                    <tr>
                        <td>
                            if item.level != Level::Special(None) {
                                {item.level.name(settings.game_language)}
                            }
                            <br/>
                            {icons}
                        </td>
                    </tr>
                });
            } else {
                let extra = if let Some(monster) = is_special
                    && monster.color != game::Color::Commander
                {
                    format!(", {}", monster.color.short(settings.game_language))
                } else {
                    String::new()
                };
                result.push(html! {
                    <tr>
                        <td>
                            {item.monster.unwrap().name(settings.game_language)}{if item.preset {"*"}else{""}}
                            {" ("}{item.monster.unwrap().content.name(settings.game_language)}{extra}{")"}
                            if item.level != Level::Special(None) {
                                {" - "}{item.level.name(settings.game_language)}
                            }
                            <br/>
                            {icons}
                        </td>
                    </tr>
                });
            }
        } else {
            result.push(render_list_old(settings, item));
        }
    }

    result
}

fn main() {
    yew::Renderer::<App>::new().render();
}
