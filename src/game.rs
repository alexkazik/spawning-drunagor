use enum_tools::EnumTools;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
pub(crate) struct Monster {
    name_en: &'static str,
    name_de: &'static str,
    pub(crate) content: Content,
    pub(crate) color: Color,
    #[allow(dead_code)]
    pub(crate) has_complex: bool,
}

impl PartialEq for Monster {
    fn eq(&self, other: &Self) -> bool {
        self.name_en == other.name_en
    }
}

impl Eq for Monster {}

impl Hash for Monster {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.name_en.as_ptr() as usize);
    }
}

impl Monster {
    pub(crate) fn name(&self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => self.name_en,
            GameLanguage::De => self.name_de,
        }
    }
}

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
#[enum_tools(iter, into)]
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
                Content::Apocalypse => "Apocalypse*",
                Content::Awakenings => "Awakenings*",
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
}

impl Color {
    pub(crate) fn name(self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => match self {
                Color::White => "White",
                Color::Gray => "Gray",
                Color::Black => "Black",
                Color::Commander => "Commander",
            },
            GameLanguage::De => match self {
                Color::White => "Weiß",
                Color::Gray => "Grau",
                Color::Black => "Schwarz",
                Color::Commander => "Kommandant",
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
            },
            GameLanguage::De => match self {
                Color::White => "WM",
                Color::Gray => "GM",
                Color::Black => "SM",
                Color::Commander => "Kommandant",
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
            },
            GameLanguage::De => match self {
                Color::White => "W",
                Color::Gray => "G",
                Color::Black => "S",
                Color::Commander => "K",
            },
        }
    }
}

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(Copy, Clone, EnumTools, PartialEq, Eq, Hash)]
#[enum_tools(iter, into)]
#[repr(u8)]
#[allow(dead_code)]
pub(crate) enum Special {
    UndeadKing,
    UndeadKingsMinion,
    CommanderBrute,
    MurderousApparition,
    ManifestationOfWrath,
    DrifterApparition,
    TormentOfEnvy,
    DireExecutioner,
}

impl Special {
    pub(crate) fn name(self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => match self {
                Special::UndeadKing => "Undead King",
                Special::UndeadKingsMinion => "Undead King's Minion",
                Special::CommanderBrute => "Brute",
                Special::MurderousApparition => "Murderous Apparition",
                Special::ManifestationOfWrath => "Manifestation of Wrath",
                Special::DrifterApparition => "Drifter Apparition",
                Special::TormentOfEnvy => "Torment of Envy",
                Special::DireExecutioner => "Dire Executioner",
            },
            GameLanguage::De => match self {
                Special::UndeadKing => "Untoter König",
                Special::UndeadKingsMinion => "Undead King's Minion",
                Special::CommanderBrute => "Scheusal",
                Special::MurderousApparition => "Mordende Erscheinung",
                Special::ManifestationOfWrath => "Manifestation des Zorns",
                Special::DrifterApparition => "Treibende Erscheinung",
                Special::TormentOfEnvy => "Qual der Gier",
                Special::DireExecutioner => "Henker des Schreckens",
            },
        }
    }

    pub(crate) fn monster(self) -> Option<&'static str> {
        match self {
            Special::UndeadKing | Special::CommanderBrute => None, // Special
            Special::UndeadKingsMinion => Some("Skeleton Archer"),
            Special::MurderousApparition => Some("Shadow Vampire"),
            Special::ManifestationOfWrath => Some("Shadow Knight"),
            Special::TormentOfEnvy => Some("Abomination"),
            Special::DrifterApparition | Special::DireExecutioner => Some("Executioner"),
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
    Special(Option<Special>),
}

impl Level {
    pub(crate) fn id(self) -> &'static str {
        match self {
            Level::Rookie => "Ro",
            Level::Fighter => "Fi",
            Level::Veteran => "Ve",
            Level::Champion => "Ch",
            Level::Special(_) => "Sp",
        }
    }

    pub(crate) fn name(self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => match self {
                Level::Rookie => "Rookie",
                Level::Fighter => "Fighter",
                Level::Veteran => "Veteran",
                Level::Champion => "Champion",
                Level::Special(s) => s.map_or("", |s| s.name(game_language)),
            },
            GameLanguage::De => match self {
                Level::Rookie => "Novize",
                Level::Fighter => "Kämpfer",
                Level::Veteran => "Veteran",
                Level::Champion => "Meister",
                Level::Special(s) => s.map_or("", |s| s.name(game_language)),
            },
        }
    }
}

pub(crate) const MONSTERS: &[&Monster] = &[
    // Core
    &Monster {
        name_en: "Abomination",
        name_de: "Abscheulichkeit",
        content: Content::Core,
        color: Color::Black,
        has_complex: false,
    },
    &Monster {
        name_en: "Archon",
        name_de: "Archon",
        content: Content::Core,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Bane",
        name_de: "Bane",
        content: Content::Core,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Executioner",
        name_de: "Vollstrecker",
        content: Content::Core,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Rotten Flesh",
        name_de: "Fleischgolem",
        content: Content::Core,
        color: Color::Gray,
        has_complex: true,
    },
    &Monster {
        name_en: "Shadow Cultist",
        name_de: "Schattenkultist",
        content: Content::Core,
        color: Color::White,
        has_complex: true,
    },
    &Monster {
        name_en: "Shadow Knight",
        name_de: "Schattenritter",
        content: Content::Core,
        color: Color::Black,
        has_complex: true,
    },
    &Monster {
        name_en: "Shadow Vampire",
        name_de: "Schattenvampir",
        content: Content::Core,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Skeleton Archer",
        name_de: "Skelettbogenschütze",
        content: Content::Core,
        color: Color::White,
        has_complex: false,
    },
    // Apocalypse
    &Monster {
        name_en: "Dream Titan",
        name_de: "Dream Titan",
        content: Content::Apocalypse,
        color: Color::Black,
        has_complex: false,
    },
    &Monster {
        name_en: "Faceless Conjurer",
        name_de: "Faceless Conjurer",
        content: Content::Apocalypse,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Hellish Flayer",
        name_de: "Hellish Flayer",
        content: Content::Apocalypse,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Shadow Witch",
        name_de: "Shadow Witch",
        content: Content::Apocalypse,
        color: Color::White,
        has_complex: false,
    },
    &Monster {
        name_en: "Skeleton Knight",
        name_de: "Skeleton Knight",
        content: Content::Apocalypse,
        color: Color::White,
        has_complex: false,
    },
    // Awakenings
    &Monster {
        name_en: "Doctor",
        name_de: "Doctor",
        content: Content::Awakenings,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Fell Asteris",
        name_de: "Fell Asteris",
        content: Content::Awakenings,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Flinch",
        name_de: "Flinch",
        content: Content::Awakenings,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Gorgon Hexer",
        name_de: "Gorgon Hexer",
        content: Content::Awakenings,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Gorgoness Witch",
        name_de: "Gorgoness Witch",
        content: Content::Awakenings,
        color: Color::White,
        has_complex: false,
    },
    &Monster {
        name_en: "Gremlin Horde",
        name_de: "Gremlin Horde",
        content: Content::Awakenings,
        color: Color::White,
        has_complex: false,
    },
    &Monster {
        name_en: "Grim Doctor",
        name_de: "Grim Doctor",
        content: Content::Awakenings,
        color: Color::White,
        has_complex: false,
    },
    &Monster {
        name_en: "Hellspawn Brute",
        name_de: "Hellspawn Brute",
        content: Content::Awakenings,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Hexer",
        name_de: "Hexer",
        content: Content::Awakenings,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Horde",
        name_de: "Horde",
        content: Content::Awakenings,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Hunter",
        name_de: "Hunter",
        content: Content::Awakenings,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Nagian Hunter",
        name_de: "Nagian Hunter",
        content: Content::Awakenings,
        color: Color::White,
        has_complex: false,
    },
    &Monster {
        name_en: "Night Stalker",
        name_de: "Night Stalker",
        content: Content::Awakenings,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Ox",
        name_de: "Ox",
        content: Content::Awakenings,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Spawn",
        name_de: "Spawn",
        content: Content::Awakenings,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Witch",
        name_de: "Witch",
        content: Content::Awakenings,
        color: Color::Commander,
        has_complex: false,
    },
    // DesertOfHellscar
    &Monster {
        name_en: "Corrupted Worm",
        name_de: "Verdorbener Lindwurm",
        content: Content::DesertOfTheHellscar,
        color: Color::Gray,
        has_complex: false,
    },
    // FallenSisters
    &Monster {
        name_en: "Fallen Sisters",
        name_de: "Fallen Sisters",
        content: Content::FallenSisters,
        color: Color::Commander,
        has_complex: false,
    },
    // MonsterPack1
    &Monster {
        name_en: "Death Messenger",
        name_de: "Todesbote",
        content: Content::MonsterPack1,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Scout Of Darkness",
        name_de: "Dunkler Späher",
        content: Content::MonsterPack1,
        color: Color::White,
        has_complex: false,
    },
    // RiseOfTheUndeadDragon
    &Monster {
        name_en: "Corrupted Farmer",
        name_de: "Verdorbener Baue",
        content: Content::RiseOfTheUndeadDragon,
        color: Color::White,
        has_complex: false,
    },
    // SpoilsOfWar
    &Monster {
        name_en: "Bone Reaper",
        name_de: "Sensenmann",
        content: Content::SpoilsOfWar,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Shadow Mistress",
        name_de: "Schattenherrin",
        content: Content::SpoilsOfWar,
        color: Color::White,
        has_complex: false,
    },
    &Monster {
        name_en: "Thern",
        name_de: "Thern",
        content: Content::SpoilsOfWar,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Twin",
        name_de: "Twins",
        content: Content::SpoilsOfWar,
        color: Color::Commander,
        has_complex: false,
    },
    &Monster {
        name_en: "Walking Horror",
        name_de: "Wanderschrecken",
        content: Content::SpoilsOfWar,
        color: Color::White,
        has_complex: false,
    },
    // TheRuinOfLuccanor
    &Monster {
        name_en: "Lady Claw",
        name_de: "Klauennatter",
        content: Content::TheRuinOfLuccanor,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Ravager",
        name_de: "Verwüster",
        content: Content::TheRuinOfLuccanor,
        color: Color::White,
        has_complex: false,
    },
    // TheShadowWorld
    &Monster {
        name_en: "Shadow Guardian",
        name_de: "Schattenwächter",
        content: Content::TheShadowWorld,
        color: Color::Gray,
        has_complex: false,
    },
    &Monster {
        name_en: "Shadow Pain",
        name_de: "Schattenschmerz",
        content: Content::TheShadowWorld,
        color: Color::White,
        has_complex: false,
    },
];

pub(crate) const MONSTER_VERY_SPECIAL: &Monster = &Monster {
    name_en: "",
    name_de: "",
    content: Content::Core,
    color: Color::Commander,
    has_complex: false,
};

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

#[test]
fn test() {
    use std::collections::HashSet;

    let mut x = HashSet::new();
    for m in MONSTERS {
        assert!(x.insert(m.name_en), "Duplicate monster name");
    }
}
