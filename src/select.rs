#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::inline_always)]
#![allow(clippy::too_many_lines)]

use crate::Settings;
use crate::game::{Chapter, Color, Content, GameLanguage, Level, MONSTERS, Monster};
use crate::setup::SetupStore;
use enum_tools::EnumTools;
use rand::rng;
use rand::seq::SliceRandom;
#[cfg(feature = "debug")]
use serde::Serialize;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use yew::{Html, function_component, html};
use yew_bootstrap::component::{Alert, Button};
use yew_bootstrap::icons::BI;
use yewdux::mrc::Mrc;
use yewdux::{Dispatch, Reducer, Store, use_store};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(
    Copy, Clone, Eq, PartialEq, EnumTools, Ord, PartialOrd, Deserialize_repr, Serialize_repr,
)]
#[enum_tools(as_str, iter)]
#[repr(u8)]
#[allow(dead_code)] // only constructed via enum_tools
pub(crate) enum Number {
    #[enum_tools(rename = "1")]
    One = 1,
    #[enum_tools(rename = "2")]
    Two = 2,
    #[enum_tools(rename = "3")]
    Three = 3,
    #[enum_tools(rename = "4")]
    Four = 4,
    #[enum_tools(rename = "5")]
    Five = 5,
}

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(Clone, PartialEq)]
pub(crate) struct Item {
    pub(crate) number: Option<Number>,
    pub(crate) color: Option<Color>,
    pub(crate) level: Level,
    pub(crate) monster: Option<&'static Monster>,
    pub(crate) preset: bool,
}

#[derive(Clone, PartialEq, Store)]
pub(crate) struct SelectStore {
    // custom
    selected: Mrc<Vec<Item>>,
    current_number: Option<Number>,
    current_color: Color,
    current_level: Level,
    current_monster: Mrc<Option<&'static Monster>>,
    pub(crate) output: Mrc<Vec<Item>>,
    pub(crate) setup: Option<(Content, Chapter, &'static str)>,
}

impl Default for SelectStore {
    fn default() -> Self {
        Self {
            // custom
            selected: Mrc::new(vec![]),
            current_number: None,
            current_color: Color::White,
            current_level: Level::Rookie,
            current_monster: Mrc::default(),
            output: Mrc::new(vec![]),
            setup: None,
        }
    }
}

impl SelectStore {
    pub(crate) fn adjust_content(&mut self, settings: &Settings) {
        let mut changed = false;
        self.selected.borrow_mut().iter_mut().for_each(|item| {
            if let Some(sm) = item.monster
                && !settings.content.contains(&sm.content)
            {
                item.monster = None;
                changed = true;
            }
        });
        if changed {
            self.output(Some(settings), false);
        }
    }

    fn output(&mut self, settings: Option<&Settings>, keep_setup: bool) {
        if !keep_setup {
            self.setup = None;
        }
        let mut o = self.output.borrow_mut();
        let rc_settings = Dispatch::<Settings>::global().get();
        let settings = settings.unwrap_or(&*rc_settings);

        // gather available monsters (by type)
        let avail_monsters = MONSTERS
            .iter()
            .copied()
            .filter(|monster| settings.content.contains(&monster.content))
            .collect::<Vec<_>>();

        let mut todo = HashMap::<Color, HashSet<Level>>::new();
        for item in self.selected.borrow().iter() {
            if item.monster.is_none()
                && let Some(c) = item.color
            {
                todo.entry(c).or_default().insert(item.level);
            }
        }

        o.clear();
        let selected = Self::select(
            avail_monsters
                .iter()
                .copied()
                .filter(|m| {
                    !self
                        .selected
                        .borrow()
                        .iter()
                        .any(|item| item.monster == Some(m))
                })
                .collect(),
            &todo,
        )
        .or_else(|| Self::select(avail_monsters, &todo));
        let Some(selected) = selected else {
            return;
        };
        for item in self.selected.borrow().iter() {
            if !item.preset {
                if item.monster.is_none() && item.color.is_some() {
                    o.push(Item {
                        number: item.number,
                        color: item.color,
                        level: item.level,
                        monster: selected.get(&(item.color.unwrap(), item.level)).copied(),
                        preset: false,
                    });
                } else {
                    o.push(Item {
                        number: item.number,
                        color: item.color,
                        level: item.level,
                        monster: item.monster,
                        preset: true,
                    });
                }
            }
        }
    }

    fn select(
        mut avail: Vec<&'static Monster>,
        todo: &HashMap<Color, HashSet<Level>>,
    ) -> Option<HashMap<(Color, Level), &'static Monster>> {
        avail.shuffle(&mut rng());

        let mut r = HashMap::new();
        for (co, levels) in todo {
            for le in levels {
                let (i, m) = avail
                    .iter()
                    .copied()
                    .enumerate()
                    .find(|(_, m)| m.color == *co)?;
                avail.swap_remove(i);
                r.insert((*co, *le), m);
            }
        }

        Some(r)
    }
}

impl Reducer<SelectStore> for Option<Number> {
    fn apply(self, mut rc_state: Rc<SelectStore>) -> Rc<SelectStore> {
        let state = Rc::make_mut(&mut rc_state);
        state.current_number = self;
        rc_state
    }
}

impl Reducer<SelectStore> for Color {
    fn apply(self, mut rc_state: Rc<SelectStore>) -> Rc<SelectStore> {
        let state = Rc::make_mut(&mut rc_state);
        state.current_color = self;
        rc_state
    }
}

impl Reducer<SelectStore> for Level {
    fn apply(self, mut rc_state: Rc<SelectStore>) -> Rc<SelectStore> {
        let state = Rc::make_mut(&mut rc_state);
        state.current_level = self;
        rc_state
    }
}

impl Reducer<SelectStore> for Option<&'static Monster> {
    fn apply(self, mut rc_state: Rc<SelectStore>) -> Rc<SelectStore> {
        let state = Rc::make_mut(&mut rc_state);
        state.selected.borrow_mut().push(Item {
            number: state.current_number,
            color: Some(state.current_color),
            level: state.current_level,
            monster: self,
            preset: false,
        });
        state.output(None, false);
        rc_state
    }
}

pub(crate) struct Randomize;
impl Reducer<SelectStore> for Randomize {
    fn apply(self, mut rc_state: Rc<SelectStore>) -> Rc<SelectStore> {
        let state = Rc::make_mut(&mut rc_state);
        state.output(None, true);
        rc_state
    }
}

struct Remove(usize);
impl Reducer<SelectStore> for Remove {
    fn apply(self, mut rc_state: Rc<SelectStore>) -> Rc<SelectStore> {
        let state = Rc::make_mut(&mut rc_state);
        state.selected.borrow_mut().remove(self.0);
        state.output(None, false);
        rc_state
    }
}

enum Preset {
    Content(Content),
    Chapter(Chapter),
    Show(usize),
}

impl Reducer<SelectStore> for Preset {
    fn apply(self, mut rc_state: Rc<SelectStore>) -> Rc<SelectStore> {
        let state = Rc::make_mut(&mut rc_state);
        match self {
            Preset::Content(c) => {
                Dispatch::<Settings>::global().reduce_mut(|settings| {
                    settings.preset_content = c;
                });
            }
            Preset::Chapter(ct) => {
                Dispatch::<Settings>::global().reduce_mut(|settings| {
                    settings.preset_chapter = ct;
                });
            }
            Preset::Show(index) => {
                let rc_settings = Dispatch::<Settings>::global().get();
                let setup = Dispatch::<SetupStore>::global().get();
                if let Some(setup) = setup
                    .setups
                    .iter()
                    .filter(|s| {
                        s.content == rc_settings.preset_content
                            && s.chapter == rc_settings.preset_chapter
                    })
                    .nth(index)
                {
                    {
                        let mut selected = state.selected.borrow_mut();
                        selected.clear();
                        for item in &setup.monsters {
                            selected.push(item.clone());
                        }
                    }
                    state.output(None, false);
                    let rc_settings = Dispatch::<Settings>::global().get();
                    state.setup = Some((
                        setup.content,
                        setup.chapter,
                        setup.name(rc_settings.game_language),
                    ));
                }
            }
        }
        rc_state
    }
}

#[function_component]
pub(crate) fn Select() -> Html {
    let (settings, _) = use_store::<Settings>();
    let (store, dispatch) = use_store::<SelectStore>();
    let (setup, _) = use_store::<SetupStore>();

    if settings.preset {
        let mut contents = setup.setups.iter().map(|s| s.content).collect::<Vec<_>>();
        contents.sort_by_key(|s| s.order_name(settings.game_language));
        contents.dedup();
        let contents = contents.into_iter().map(|c| {
            let onclick = dispatch.apply_callback(move |_| Preset::Content(c));
            let id = format!("preset_content_{}", c.name(GameLanguage::En));
            html! {
                <>
                    <input
                        type="radio"
                        class="btn-check"
                        name="preset_content"
                        id={id.clone()}
                        autocomplete="off"
                        checked={settings.preset_content == c}
                        onclick={onclick}
                    />
                    <label class="btn btn-outline-primary" for={id}>{c.name(settings.game_language)}</label>
                </>
            }
        });

        let mut chapters = setup
            .setups
            .iter()
            .filter(|s| s.content == settings.preset_content)
            .map(|s| s.chapter)
            .collect::<Vec<_>>();
        chapters.sort();
        chapters.dedup();
        let content_types = chapters.into_iter().map(|ct| {
            let onclick = dispatch.apply_callback(move |_| Preset::Chapter(ct));
            let id = format!("preset_content_type_{}", ct.0);
            html! {
                <>
                    <input
                        type="radio"
                        class="btn-check"
                        name="preset_content_type"
                        id={id.clone()}
                        autocomplete="off"
                        checked={settings.preset_chapter == ct}
                        onclick={onclick}
                    />
                    <label class="btn btn-outline-primary" for={id}>{ct.0}</label>
                </>
            }
        });

        let entries = setup
            .setups
            .iter()
            .filter(|s| {
                s.content == settings.preset_content && s.chapter == settings.preset_chapter
            })
            .collect::<Vec<_>>();
        let entries = entries.into_iter().enumerate().map(|(i, setup)| {
            let onclick = dispatch.apply_callback(move |_| Preset::Show(i));
            html! {
                <>
                    <button
                        class="btn btn-primary"
                        type="button"
                        onclick={onclick}
                        data-bs-toggle="collapse"
                        data-bs-target="#collapseThree"
                    >
                        {setup.name(settings.game_language)}
                    </button>
                </>
            }
        });
        html! {
            <div>
            <div class="btn-group-vertical" style="vertical-align: top" role="group" aria-label="Vertical button group">
                {for contents}
            </div>
            <div class="btn-group-vertical" style="vertical-align: top" role="group" aria-label="Vertical button group">
                {for content_types}
            </div>
            <div class="btn-group-vertical" style="vertical-align: top" role="group" aria-label="Vertical button group">
                {for entries}
            </div>
            </div>
        }
    } else {
        let list = store
            .selected
            .borrow()
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let trash_onclick = dispatch.apply_callback(move |_| Remove(i));
                if let Some(m) = item.monster {
                    html! {
                        <tr>
                            <td>if item.preset {{"*"}} else {{BI::PERSON_WALKING}{item.number.map_or("*",|x|x.as_str())}}</td>
                            <td>{item.color.map_or("",|c|c.short(settings.game_language))}{" - "}{m.name(settings.game_language)}{" - "}{item.level.name(settings.game_language)}</td>
                            <td>
                                <Button
                                    style={yew_bootstrap::util::Color::Danger}
                                    outline={true}
                                    onclick={trash_onclick}
                                >{BI::TRASH}</Button>
                            </td>
                        </tr>
                    }
                } else {
                    html! {
                        <tr>
                            <td><strong>{item.color.map_or("",|c|c.prefix(settings.game_language))}{item.number.map_or("*",|x|x.as_str())}</strong></td>
                            <td>{item.color.map_or("",|c|c.short(settings.game_language))}{" - "}{item.level.name(settings.game_language)}</td>
                            <td>
                                <Button
                                    style={yew_bootstrap::util::Color::Danger}
                                    outline={true}
                                    onclick={trash_onclick}
                                >{BI::TRASH}</Button>
                            </td>
                        </tr>
                    }
                }
            })
            .collect::<Vec<_>>();

        let numbers = Number::iter().map(|num| {
            if num > settings.players {
                return html! {};
            }
            let id = format!("number:{}", num.as_str());
            let onchange = if store.current_number == Some(num) {
                dispatch.apply_callback(move |_| Option::<Number>::None)
            } else {
                dispatch.apply_callback(move |_| Some(num))
            };

            html! {
                <>
                    <input
                        type="radio"
                        class="btn-check"
                        name="number"
                        id={id.clone()}
                        autocomplete="off"
                        checked={store.current_number == Some(num)}
                        onclick={onchange}
                    />
                    <label class="btn btn-outline-primary" for={id}>{num.as_str()}</label>
                </>
            }
        });

        let colors = Color::iter().map(|color| {
            let id = format!("color:{}", color.into());
            let onchange = dispatch.apply_callback(move |_| color);
            html! {
                <>
                    <input
                        type="radio"
                        class="btn-check"
                        name="color"
                        id={id.clone()}
                        autocomplete="off"
                        checked={store.current_color == color}
                        onchange={onchange}
                    />
                    <label class="btn btn-outline-primary" for={id}>{color.name(settings.game_language)}</label>
                </>
            }
        });

        let levels = [Level::Rookie, Level::Fighter, Level::Veteran, Level::Champion]
            .into_iter()
            .map(|level| {
                let id = format!("level:{}", level.id());
                let onchange = dispatch.apply_callback(move |_| level);
                html! {
                    <>
                        <input
                            type="radio"
                            class="btn-check"
                            name="level"
                            id={id.clone()}
                            autocomplete="off"
                            checked={store.current_level == level && store.current_color != Color::Commander}
                            onchange={onchange}
                            disabled={store.current_color == Color::Commander}
                        />
                        <label class="btn btn-outline-primary" for={id}>{level.name(settings.game_language)}</label>
                    </>
                }
            });

        let mut monsters = MONSTERS
            .iter()
            .copied()
            .filter(|monster| {
                monster.color == store.current_color && settings.content.contains(&monster.content)
            })
            .map(|monster| (monster, monster.name(settings.game_language)))
            .collect::<Vec<_>>();
        monsters.sort_by_key(|(_, name)| *name);

        let monsters = monsters.into_iter().map(|(monster, name)| {
            let onclick = dispatch.apply_callback(move |_| Some(monster));
            html! {
                <>
                    <Button
                        style={yew_bootstrap::util::Color::Secondary}
                        onclick={onclick}
                    >
                        {name}
                    </Button>
                </>
            }
        });

        html! {
        <>
            <table class="table">
                <tbody>
                    {for list}
                </tbody>
            </table>
                if !store.selected.borrow().is_empty() && store.output.borrow().is_empty() {
                    <Alert style={yew_bootstrap::util::Color::Warning}>
                        {"Too many different monsters requested"}
                    </Alert>
                }
            <div>
                <div class="btn-group-vertical" style="vertical-align: top" role="group" aria-label="Vertical button group">
                    {for numbers}
                </div>
                <div class="btn-group-vertical" style="vertical-align: top" role="group" aria-label="Vertical button group">
                    {for colors}
                </div>
                <div class="btn-group-vertical" style="vertical-align: top" role="group" aria-label="Vertical button group">
                    {for levels}
                </div>
                <div class="btn-group-vertical" style="vertical-align: top" role="group" aria-label="Vertical button group">
                    <Button
                        style={yew_bootstrap::util::Color::Success}
                        onclick={dispatch.apply_callback(|_|Option::<&'static Monster>::None)}
                    >
                        {"Random"}
                    </Button>
                    {for monsters}
                </div>
            </div>
        </>
        }
    }
}
