use crate::game::{Color, Content, Level, Number};
use crate::monster::Mns;
use anyhow::{anyhow, bail};
use std::fmt::Write;
use std::str::FromStr;

pub fn setup(monsters: &[Mns]) -> Result<String, anyhow::Error> {
    let setups = include_str!("setup.csv")
        .lines()
        .map(|l| l.trim_end_matches(',').split(',').collect::<Vec<_>>())
        .filter(|l| l.len() >= 4)
        .map(|l| Setup::read(monsters, &l))
        .collect::<Result<Vec<_>, _>>()?;

    setups.iter().try_fold(None, |a: Option<&Setup>, i| {
        if let Some(a) = a
            && (i.content, i.chapter) < (a.content, a.chapter)
        {
            bail!(
                "wrong order: {}.{}.{} is before {}.{}.{}",
                a.content,
                a.chapter,
                a.name_en,
                i.content,
                i.chapter,
                i.name_en
            );
        }
        Ok(Some(i))
    })?;

    let mut output = String::new();

    writeln!(output, "pub const SETUPS : &[Setup] = &[")?;
    for setup in setups {
        writeln!(output, "    Setup {{")?;
        writeln!(output, "        content: Content::{},", setup.content)?;
        writeln!(output, "        chapter: Chapter({}),", setup.chapter)?;
        writeln!(output, "        name_en: {:#?},", setup.name_en)?;
        writeln!(output, "        name_de: {:#?},", setup.name_de)?;
        writeln!(output, "        monsters: &[")?;
        for item in setup.items {
            writeln!(
                output,
                "            SetupItem {{ number: Number::{}, color: Color::{}, level: Level::{}, monster: {}, exclude: {:#?} }},",
                item.number,
                item.color,
                item.level,
                item.monster
                    .map_or("None".to_string(), |m| format!("Some(Monster::{})", m)),
                item.exclude,
            )?;
        }
        writeln!(output, "        ],")?;
        writeln!(output, "    }},")?;
    }
    writeln!(output, "];")?;

    Ok(output)
}

pub(crate) struct Setup<'a> {
    content: Content,
    chapter: usize,
    name_en: &'static str,
    name_de: &'static str,
    items: Vec<SetupItem<'a>>,
}

pub(crate) struct SetupItem<'a> {
    pub(crate) number: Number,
    pub(crate) color: Color,
    pub(crate) level: Level,
    pub(crate) monster: Option<&'a str>,
    pub(crate) exclude: bool,
}

impl<'s> Setup<'s> {
    fn read<'a: 's>(all_monsters: &'a [Mns], fields: &[&'static str]) -> anyhow::Result<Self> {
        let m = fields[4..]
            .chunks(2)
            .map(|x| (x[0], x.get(1).map_or("", |y| *y)))
            .filter(|(x, y)| !x.is_empty() || !y.is_empty())
            .collect::<Vec<_>>();
        let mut monsters = Vec::with_capacity(m.len());
        let mut last_number = Number::One;
        for (f1, f2) in m {
            let error = |reason: &str| -> anyhow::Error {
                anyhow!(
                    "error \"{reason}\" on field \"{f1}\", \"{f2}\" in line \"{}\"",
                    fields.join(",")
                )
            };

            let (number, mut color, level, exclude) = if f1 == "Exclude" {
                (Number::One, Color::White, Level::Rookie, true)
            } else {
                let (f1_co_num, f1_le) = f1.split_once(' ').unwrap_or((f1, ""));
                if f1_co_num.len() < 2 {
                    return Err(error("field too short"));
                }
                let co = Color::iter()
                    .find(|c| c.as_str().starts_with(&f1_co_num[0..1]))
                    .ok_or_else(|| error("unknown color"))?;

                let nu = u8::from_str(&f1_co_num[1..2])
                    .map_err(|_| ())
                    .and_then(Number::try_from)
                    .map_err(|_| error("unknown number"))?;

                let le = match co {
                    Color::Commander | Color::Special | Color::SpecialCommander => {
                        if f1_le.is_empty() {
                            Level::Rookie
                        } else {
                            return Err(error("unknown commander/special level"));
                        }
                    }
                    _ => Level::iter()
                        .find(|level| !f1_le.is_empty() && level.as_str().starts_with(f1_le))
                        .ok_or_else(|| error("unknown regular level"))?,
                };

                if nu < last_number {
                    return Err(error("number decreased"));
                }
                last_number = nu;

                (nu, co, le, false)
            };

            let monster = if f2.is_empty() {
                if color.is_any_special() {
                    return Err(error("special without monster"));
                }
                None
            } else {
                let (sp, f2) = f2
                    .strip_prefix('*')
                    .map(|f2| (true, f2))
                    .unwrap_or((false, f2));

                let monster = all_monsters
                    .iter()
                    .find(|m| m.name_en == f2)
                    .ok_or_else(|| error("unknown monster"))?;

                if !(exclude
                    || (!sp && !color.is_any_special() && color == monster.color)
                    || (sp && color.is_any_special() && monster.color.is_any_special()))
                {
                    return Err(error(&format!(
                        "color mismatch sp={:#?}, monster-color:{}",
                        sp, monster.color,
                    )));
                }

                if color.is_any_special() && monster.color == Color::SpecialCommander {
                    color = Color::SpecialCommander;
                }

                Some(monster.ident.as_str())
            };

            monsters.push(SetupItem {
                number,
                color,
                level,
                monster,
                exclude,
            });
        }

        Ok(Self {
            content: Content::from_str(fields[0])
                .map_err(|_| anyhow!("unknown content on \"{}\"", fields.join(",")))?,
            chapter: usize::from_str(fields[1])
                .map_err(|_| anyhow!("unknown chapter on \"{}\"", fields.join(",")))?,
            name_en: fields[2],
            name_de: fields[3],
            items: monsters,
        })
    }
}
