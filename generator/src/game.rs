use enum_tools::EnumTools;

#[derive(EnumTools, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[enum_tools(Display, FromStr, as_str)]
#[repr(u8)]
#[allow(dead_code)]
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
    pub(crate) fn image_prefix(&self) -> &'static str {
        let name = self.as_str().trim_start_matches("The");
        &name[0..2]
    }
}

#[derive(Copy, Clone, EnumTools, PartialEq, Eq)]
#[enum_tools(Display, FromStr, as_str, iter)]
#[repr(u8)]
#[allow(dead_code)]
pub(crate) enum Color {
    White,
    Gray,
    Black,
    Commander,
    Special,
    SpecialCommander,
}

impl Color {
    pub(crate) fn is_any_special(self) -> bool {
        self == Color::Special || self == Color::SpecialCommander
    }
}

#[derive(Copy, Clone, PartialEq, Eq, EnumTools)]
#[enum_tools(Display, as_str, iter)]
#[repr(u8)]
#[allow(dead_code)]
pub(crate) enum Level {
    Rookie,
    Fighter,
    Veteran,
    Champion,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumTools, Ord, PartialOrd)]
#[enum_tools(Display, TryFrom)]
#[repr(u8)]
#[allow(dead_code)] // only constructed via enum_tools
pub(crate) enum Number {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}
