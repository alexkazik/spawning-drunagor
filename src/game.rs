pub(crate) use crate::game::generated::{Monster, SETUPS};
use enum_tools::EnumTools;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use typed_i18n::TypedI18N;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(
    EnumTools,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
#[enum_tools(iter, into, as_str)]
#[repr(u8)]
pub(crate) enum Content {
    Core,
    Apocalypse,
    Awakenings,
    DesertOfTheHellscar,
    FallenSisters,
    MonsterPack1,
    RiseOfTheUndeadDragon,
    SpoilsOfWar,
    TheRuinOfLuccanor,
    TheShadowWorld,
}

impl Content {
    pub(crate) fn name(self, game_language: GameLanguage) -> &'static str {
        match self {
            Content::Core => game_language.content_core(),
            Content::Apocalypse => game_language.content_apocalypse(),
            Content::Awakenings => game_language.content_awakenings(),
            Content::DesertOfTheHellscar => game_language.content_desert_of_the_hellscar(),
            Content::FallenSisters => game_language.content_fallen_sisters(),
            Content::MonsterPack1 => game_language.content_monster_pack_1(),
            Content::RiseOfTheUndeadDragon => game_language.content_rise_of_th_undead_dragon(),
            Content::SpoilsOfWar => game_language.content_spoils_of_war(),
            Content::TheRuinOfLuccanor => game_language.content_the_ruin_of_luccanor(),
            Content::TheShadowWorld => game_language.content_the_shadow_world(),
        }
    }

    pub(crate) fn order_name(self, game_language: GameLanguage) -> &'static str {
        if self == Content::Core {
            "!first"
        } else {
            self.name(game_language)
        }
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub(crate) struct Chapter(pub usize);

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(Copy, Clone, EnumTools, PartialEq, Eq, Hash)]
#[enum_tools(iter, into)]
#[repr(u8)]
pub(crate) enum Color {
    White,
    Gray,
    Black,
    Commander,
    Special,
    SpecialCommander,
}

impl Color {
    pub(crate) fn is_any_commander(self) -> bool {
        self == Color::Commander || self == Color::SpecialCommander
    }

    pub(crate) fn is_any_special(self) -> bool {
        self == Color::Special || self == Color::SpecialCommander
    }

    pub(crate) fn name(self, game_language: GameLanguage) -> &'static str {
        match self {
            Color::White => game_language.color_white_name(),
            Color::Gray => game_language.color_gray_name(),
            Color::Black => game_language.color_black_name(),
            Color::Commander => game_language.color_commander_name(),
            Color::Special => panic!("called Color::Special::short()"),
            Color::SpecialCommander => panic!("called Color::SpecialCommander::short()"),
        }
    }

    pub(crate) fn short(self, game_language: GameLanguage) -> &'static str {
        match self {
            Color::White => game_language.color_white_short(),
            Color::Gray => game_language.color_gray_short(),
            Color::Black => game_language.color_black_short(),
            Color::Commander => game_language.color_commander_short(),
            Color::Special => panic!("called Color::Special::short()"),
            Color::SpecialCommander => panic!("called Color::SpecialCommander::short()"),
        }
    }

    pub(crate) fn size(self, game_language: GameLanguage) -> Option<&'static str> {
        match self {
            Color::White | Color::Gray => Some(game_language.size_small()),
            Color::Black => Some(game_language.size_big()),
            Color::Commander | Color::Special | Color::SpecialCommander => None,
        }
    }

    pub(crate) fn prefix(self, game_language: GameLanguage) -> &'static str {
        match self {
            Color::White => game_language.color_white_prefix(),
            Color::Gray => game_language.color_gray_prefix(),
            Color::Black => game_language.color_black_prefix(),
            Color::Commander => game_language.color_commander_prefix(),
            Color::Special => panic!("called Color::Special::prefix()"),
            Color::SpecialCommander => panic!("called Color::SpecialCommander::prefix()"),
        }
    }

    pub(crate) fn css_prefix(self) -> &'static str {
        match self {
            Color::White => "w",
            Color::Gray => "g",
            Color::Black => "b",
            Color::Commander | Color::SpecialCommander => "c",
            Color::Special => panic!("called Color::Special::prefix_lower()"),
        }
    }
}

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Level {
    Rookie,
    Fighter,
    Veteran,
    Champion,
}

impl Level {
    pub(crate) fn id(self) -> &'static str {
        match self {
            Level::Rookie => "ro",
            Level::Fighter => "fi",
            Level::Veteran => "ve",
            Level::Champion => "ch",
        }
    }

    pub(crate) fn name(self, game_language: GameLanguage) -> &'static str {
        match self {
            Level::Rookie => game_language.level_rookie(),
            Level::Fighter => game_language.level_fighter(),
            Level::Veteran => game_language.level_veteran(),
            Level::Champion => game_language.level_champion(),
        }
    }
}

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

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(
    Copy, Clone, Eq, PartialEq, EnumTools, TypedI18N, serde::Serialize, serde::Deserialize,
)]
#[enum_tools(as_str, iter, names)]
#[repr(usize)]
#[typed_i18n(filename = "game.lrc")]
#[typed_i18n(builder = "static_str")]
pub enum GameLanguage {
    #[serde(rename = "en")]
    #[enum_tools(rename = "English")]
    En,
    #[serde(rename = "de")]
    #[enum_tools(rename = "German")]
    De,
}

mod generated {
    use crate::game::{Chapter, Color, Content, GameLanguage, Level, Number};
    use crate::setup::{Setup, SetupItem};
    use enum_tools::EnumTools;
    #[cfg(feature = "debug")]
    use serde::Serialize;

    include!(concat!(env!("OUT_DIR"), "/generated_monster.rs"));
    include!(concat!(env!("OUT_DIR"), "/generated_setup.rs"));
}
