pub(crate) use crate::game::generated::{Monster, SETUPS};
use enum_tools::EnumTools;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
        match game_language {
            GameLanguage::En => match self {
                Content::Core => "Core",
                Content::Apocalypse => "Apocalypse",
                Content::Awakenings => "Awakenings",
                Content::DesertOfTheHellscar => "Desert of the Hellscar",
                Content::FallenSisters => "Fallen Sisters",
                Content::MonsterPack1 => "Monster Pack 1",
                Content::RiseOfTheUndeadDragon => "Rise of the Undead Dragon",
                Content::SpoilsOfWar => "Spoils of War",
                Content::TheRuinOfLuccanor => "The Ruin of Luccanor",
                Content::TheShadowWorld => "The Shadow World",
            },
            GameLanguage::De => match self {
                Content::Core => "Grundspiel",
                Content::Apocalypse => "Apocalypse",
                Content::Awakenings => "Erwachen",
                Content::DesertOfTheHellscar => "Wüste der Narben",
                Content::FallenSisters => "Fallen Sisters*",
                Content::MonsterPack1 => "Neue Helden & neue Monster",
                Content::RiseOfTheUndeadDragon => "Der untote Drache",
                Content::SpoilsOfWar => "Kriegsbeute",
                Content::TheRuinOfLuccanor => "Admiral Luccanors Verderben",
                Content::TheShadowWorld => "Die Schattenwelt",
            },
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
        match game_language {
            GameLanguage::En => match self {
                Color::White => "White",
                Color::Gray => "Gray",
                Color::Black => "Black",
                Color::Commander => "Commander",
                Color::Special => panic!("called Color::Special::name()"),
                Color::SpecialCommander => panic!("called Color::SpecialCommander::name()"),
            },
            GameLanguage::De => match self {
                Color::White => "Weiß",
                Color::Gray => "Grau",
                Color::Black => "Schwarz",
                Color::Commander => "Kommandant",
                Color::Special => panic!("called Color::Special::name()"),
                Color::SpecialCommander => panic!("called Color::SpecialCommander::name()"),
            },
        }
    }

    pub(crate) fn short(self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => match self {
                Color::White => "WM",
                Color::Gray => "GM",
                Color::Black => "BM",
                Color::Commander => "Commander",
                Color::Special => panic!("called Color::Special::short()"),
                Color::SpecialCommander => panic!("called Color::SpecialCommander::short()"),
            },
            GameLanguage::De => match self {
                Color::White => "WM",
                Color::Gray => "GM",
                Color::Black => "SM",
                Color::Commander => "Kommandant",
                Color::Special => panic!("called Color::Special::short()"),
                Color::SpecialCommander => panic!("called Color::SpecialCommander::short()"),
            },
        }
    }

    pub(crate) fn size(self, game_language: GameLanguage) -> Option<&'static str> {
        match game_language {
            GameLanguage::En => match self {
                Color::White | Color::Gray => Some("small"),
                Color::Black => Some("big"),
                Color::Commander | Color::Special | Color::SpecialCommander => None,
            },
            GameLanguage::De => match self {
                Color::White | Color::Gray => Some("klein"),
                Color::Black => Some("groß"),
                Color::Commander | Color::Special | Color::SpecialCommander => None,
            },
        }
    }

    pub(crate) fn prefix(self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => match self {
                Color::White => "W",
                Color::Gray => "G",
                Color::Black => "B",
                Color::Commander => "C",
                Color::Special => panic!("called Color::Special::prefix()"),
                Color::SpecialCommander => panic!("called Color::SpecialCommander::prefix()"),
            },
            GameLanguage::De => match self {
                Color::White => "W",
                Color::Gray => "G",
                Color::Black => "S",
                Color::Commander => "K",
                Color::Special => panic!("called Color::Special::prefix()"),
                Color::SpecialCommander => panic!("called Color::SpecialCommander::prefix()"),
            },
        }
    }

    pub(crate) fn prefix_lower(self) -> &'static str {
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
            Level::Rookie => "Ro",
            Level::Fighter => "Fi",
            Level::Veteran => "Ve",
            Level::Champion => "Ch",
        }
    }

    pub(crate) fn id_lower(self) -> &'static str {
        match self {
            Level::Rookie => "ro",
            Level::Fighter => "fi",
            Level::Veteran => "ve",
            Level::Champion => "ch",
        }
    }

    pub(crate) fn name(self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => match self {
                Level::Rookie => "Rookie",
                Level::Fighter => "Fighter",
                Level::Veteran => "Veteran",
                Level::Champion => "Champion",
            },
            GameLanguage::De => match self {
                Level::Rookie => "Novize",
                Level::Fighter => "Kämpfer",
                Level::Veteran => "Veteran",
                Level::Champion => "Meister",
            },
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
#[derive(Copy, Clone, Eq, PartialEq, EnumTools, serde::Serialize, serde::Deserialize)]
#[enum_tools(as_str, iter, names)]
#[repr(usize)]
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
