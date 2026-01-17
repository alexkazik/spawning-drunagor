use crate::game::{
    Chapter, Color, Content, GameLanguage, Level, MONSTER_VERY_SPECIAL, MONSTERS, Number, Special,
};
use crate::select::Item;
use core::str::FromStr;
#[cfg(feature = "debug")]
use serde::Serialize;
use yewdux::Store;

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(PartialEq, Store)]
pub(crate) struct SetupStore {
    pub(crate) setups: Vec<Setup>,
}

impl Default for SetupStore {
    fn default() -> Self {
        let setups = include_str!("../setup.csv")
            .lines()
            .map(|l| l.split(',').collect::<Vec<_>>())
            .filter(|l| l.len() >= 4)
            .map(|l| Setup::read(&l))
            .collect::<Vec<_>>();

        setups.iter().fold(None, |a: Option<&Setup>, i| {
            if let Some(a) = a
                && (i.content, i.chapter) < (a.content, a.chapter)
            {
                panic!(
                    "wrong order: {}.{}.{} is before {}.{}.{}",
                    a.content.name(GameLanguage::En),
                    a.chapter.0,
                    a.name_en,
                    i.content.name(GameLanguage::En),
                    i.chapter.0,
                    i.name_en
                );
            }
            Some(i)
        });

        #[cfg(feature = "debug")]
        web_sys::console::log_1(&serde_wasm_bindgen::to_value(&setups).unwrap());

        Self { setups }
    }
}

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(PartialEq)]
pub(crate) struct Setup {
    pub(crate) content: Content,
    pub(crate) chapter: Chapter,
    name_en: &'static str,
    name_de: &'static str,
    pub(crate) monsters: Vec<Item>,
}

impl Setup {
    fn read(fields: &[&'static str]) -> Self {
        let mut m = fields[4..]
            .chunks(2)
            .map(|x| (x[0], x.get(1).map_or("", |y| *y)))
            .filter(|(x, y)| !x.is_empty() || !y.is_empty())
            .collect::<Vec<_>>();
        while let Some(x) = m.last() {
            if !x.0.is_empty() || !x.1.is_empty() {
                break;
            }
            m.pop();
        }
        let mut monsters = Vec::with_capacity(m.len());
        let mut last_number = Number::One;
        for (f1, f2) in m {
            let (number, mut color, mut level, preset) = if f1 == "Exclude" {
                (Number::One, None, Level::Special(None), true)
            } else {
                let f1b = f1.as_bytes();
                assert!(
                    f1b.len() >= 2,
                    "unknown monster \"{f1}\" on \"{}\"",
                    fields.join(",")
                );
                let f1_p2 = if f1b.len() > 2 {
                    assert_eq!(
                        f1b[2],
                        b' ',
                        "unknown monster \"{f1}\" on \"{}\"",
                        fields.join(",")
                    );
                    &f1b[3..]
                } else {
                    &[]
                };
                let co = match f1b[0] {
                    b'W' => Some(Color::White),
                    b'G' => Some(Color::Gray),
                    b'B' => Some(Color::Black),
                    b'C' => Some(Color::Commander),
                    b'S' => None,
                    _ => panic!("unknown color \"{f1}\" on \"{}\"", fields.join(",")),
                };
                let nu = match f1b[1] {
                    b'1' => Number::One,
                    b'2' => Number::Two,
                    b'3' => Number::Three,
                    b'4' => Number::Four,
                    b'5' => Number::Five,
                    _ => panic!("unknown number \"{f1}\" on \"{}\"", fields.join(",")),
                };
                let le = match co {
                    None => match f1_p2 {
                        b"" => Level::Special(None),
                        _ => panic!("unknown special level \"{f1}\" on \"{}\"", fields.join(",")),
                    },
                    Some(Color::Commander) => match f1_p2 {
                        b"" => Level::Special(None),
                        _ => panic!(
                            "unknown commander level \"{f1}\" on \"{}\"",
                            fields.join(",")
                        ),
                    },
                    Some(Color::White | Color::Gray | Color::Black) => match f1_p2 {
                        b"Ro" => Level::Rookie,
                        b"Fi" => Level::Fighter,
                        b"Ve" => Level::Veteran,
                        b"Ch" => Level::Champion,
                        _ => panic!("unknown regular level \"{f1}\" on \"{}\"", fields.join(",")),
                    },
                };
                assert!(
                    nu >= last_number,
                    "number decreased \"{}\" on \"{}\"",
                    f2,
                    fields.join(",")
                );
                last_number = nu;
                (nu, co, le, false)
            };
            let monster = if f2.is_empty() {
                None
            } else if let Some(f2) = f2.strip_prefix('*') {
                let special = Special::iter()
                    .find(|s| s.name(GameLanguage::En) == f2)
                    .unwrap_or_else(|| {
                        panic!(
                            "unknown special monster \"{}\" on \"{}\"",
                            f2,
                            fields.join(",")
                        )
                    });

                level = Level::Special(Some(special));

                if special == Special::CommanderBrute {
                    color = Some(Color::Commander);
                }

                if let Some(mo) = special.monster() {
                    Some(
                        MONSTERS
                            .iter()
                            .copied()
                            .find(|x| x.name(GameLanguage::En) == mo)
                            .unwrap_or_else(|| {
                                panic!("unknown monster \"{}\" on \"{}\"", mo, fields.join(","))
                            }),
                    )
                } else {
                    Some(MONSTER_VERY_SPECIAL)
                }
            } else {
                let mo = MONSTERS
                    .iter()
                    .copied()
                    .find(|x| x.name(GameLanguage::En) == f2)
                    .unwrap_or_else(|| {
                        panic!("unknown monster \"{}\" on \"{}\"", f2, fields.join(","))
                    });

                #[cfg(feature = "debug")]
                if let Some(co) = color {
                    assert_eq!(
                        co,
                        mo.color,
                        "color != monster \"{}\" on \"{}\"",
                        f2,
                        fields.join(",")
                    );
                }

                Some(mo)
            };
            monsters.push(Item {
                number: Some(number),
                color,
                level,
                monster,
                preset,
            });
        }
        Self {
            content: Content::iter()
                .find(|x| x.name(GameLanguage::En) == fields[0])
                .unwrap_or_else(|| panic!("unknown content on \"{}\"", fields.join(","))),
            chapter: fields
                .get(1)
                .and_then(|c| usize::from_str(c).ok())
                .map_or_else(
                    || panic!("unknown chapter on \"{}\"", fields.join(",")),
                    Chapter,
                ),
            name_en: fields[2],
            name_de: fields[3],
            monsters,
        }
    }

    pub(crate) fn name(&self, game_language: GameLanguage) -> &'static str {
        match game_language {
            GameLanguage::En => self.name_en,
            GameLanguage::De => self.name_de,
        }
    }
}
