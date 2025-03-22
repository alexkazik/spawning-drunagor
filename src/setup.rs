use crate::game::{Color, Content, ContentType, GameLanguage, Level, MONSTERS, Monster};
use crate::select::Number;
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
            .map(|l| Setup::read(&l.split(',').collect::<Vec<_>>()))
            .collect();

        #[cfg(feature = "debug")]
        web_sys::console::log_1(&serde_wasm_bindgen::to_value(&setups).unwrap());

        Self { setups }
    }
}

#[cfg_attr(feature = "debug", derive(Debug, Serialize))]
#[derive(PartialEq)]
pub(crate) struct Setup {
    pub(crate) content: Content,
    pub(crate) content_type: ContentType,
    name_en: &'static str,
    name_de: &'static str,
    #[allow(clippy::type_complexity)]
    pub(crate) monsters: Vec<(
        Number,
        Option<Color>,
        Option<Level>,
        Option<&'static Monster>,
    )>,
}

impl Setup {
    fn read(fields: &[&'static str]) -> Self {
        let mut m = fields[4..]
            .chunks_exact(2)
            .map(|x| (x[0], x[1]))
            .collect::<Vec<_>>();
        while let Some(x) = m.last() {
            if !x.0.is_empty() || !x.1.is_empty() {
                break;
            }
            m.pop();
        }
        let mut monsters = Vec::with_capacity(m.len());
        for (f1, f2) in m {
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
            let mut co = match f1b[0] {
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
                b'5' => Number::Fife,
                _ => panic!("unknown number \"{f1}\" on \"{}\"", fields.join(",")),
            };
            let le = match co {
                None => match f1_p2 {
                    b"MuA" | // Murderous Apparition
                    b"MWr" | // Manifestation of Wrath
                    b"DrA" | // Drifting Apparition
                    b"TEn" | // Torment of Envy
                    b"DEx" => None, // Dire Executioner
                    _ => panic!(
                        "unknown special level \"{f1}\" on \"{}\"",
                        fields.join(",")
                    ),
                },
                Some(Color::Commander) => match f1_p2 {
                    b"" => None,
                    b"CBr" => {
                        // Commander Brute, change to special
                        co = None;
                        None
                    }
                    _ => panic!(
                        "unknown commander level \"{f1}\" on \"{}\"",
                        fields.join(",")
                    ),
                },
                Some(Color::White | Color::Gray | Color::Black) => match f1_p2 {
                    b"Ro" => Some(Level::Rookie),
                    b"Fi" => Some(Level::Fighter),
                    b"Ve" => Some(Level::Veteran),
                    b"Ch" => Some(Level::Champion),
                    _ => panic!("unknown regular level \"{f1}\" on \"{}\"", fields.join(",")),
                },
            };
            let mo = if f2.is_empty() {
                None
            } else {
                let mo = MONSTERS
                    .iter()
                    .copied()
                    .find(|x| x.name(GameLanguage::En) == f2)
                    .unwrap_or_else(|| {
                        panic!("unknown monster \"{}\" on \"{}\"", f2, fields.join(","))
                    });

                #[cfg(feature = "debug")]
                if let Some(co) = co {
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
            monsters.push((nu, co, le, mo));
        }
        Self {
            content: Content::iter()
                .find(|x| x.name(GameLanguage::En) == fields[0])
                .unwrap_or_else(|| panic!("unknown content on \"{}\"", fields.join(","))),
            content_type: match fields.get(1) {
                Some(&"book") => ContentType::Book,
                Some(&"door") => ContentType::Door,
                _ => panic!("unknown content_type on \"{}\"", fields.join(",")),
            },
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
