use crate::game::{Chapter, Color, Content, GameLanguage, Level, Monster, Number};
#[cfg(feature = "debug")]
use serde::Serialize;

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(PartialEq)]
pub(crate) struct Setup {
    pub(crate) content: Content,
    pub(crate) chapter: Chapter,
    pub(crate) name_en: &'static str,
    pub(crate) name_de: &'static str,
    pub(crate) monsters: &'static [SetupItem],
}

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(Clone, PartialEq)]
pub(crate) struct SetupItem {
    pub(crate) number: Number,
    pub(crate) color: Color,
    pub(crate) level: Level,
    pub(crate) monster: Option<Monster>,
    pub(crate) exclude: bool,
}

impl Setup {
    pub(crate) fn name(&self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => self.name_en,
            GameLanguage::De => self.name_de,
        }
    }
}
