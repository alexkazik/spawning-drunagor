use crate::game::{Color, Content};
use anyhow::{anyhow, bail};
use std::collections::HashSet;
use std::fmt::Write;
use std::str::FromStr;

pub fn monster() -> Result<(String, Vec<Mns>), anyhow::Error> {
    let monsters = include_str!("monster.csv")
        .lines()
        .skip(1)
        .map(|l| l.split(',').collect::<Vec<_>>())
        .filter(|l| l.len() >= 5)
        .map(monster_read)
        .collect::<Result<Vec<_>, _>>()?;

    monsters.iter().try_fold(HashSet::new(), |mut s, m| {
        if !s.insert(m.name_en) {
            bail!("Duplicate monster name: {}", m.name_en);
        } else {
            Ok(s)
        }
    })?;

    let mut output = String::new();

    writeln!(
        output,
        "#[cfg_attr(feature = \"debug\", derive(Debug, Serialize))]"
    )?;
    writeln!(output, "#[derive(Copy, Clone, EnumTools, Eq, PartialEq)]")?;
    writeln!(output, "#[repr(u8)]")?;
    writeln!(output, "#[enum_tools(iter)]")?;
    writeln!(output, "#[allow(dead_code)]")?;
    writeln!(output, "pub(crate) enum Monster {{")?;
    for monster in &monsters {
        writeln!(output, "  {},", &monster.ident)?;
    }
    writeln!(output, "}}")?;
    writeln!(output, "impl Monster {{")?;
    writeln!(output, "    pub(crate) fn content(self) -> Content {{")?;
    writeln!(output, "        #[allow(clippy::match_same_arms)]")?;
    writeln!(output, "        match self {{")?;
    for monster in &monsters {
        writeln!(
            output,
            "            Monster::{} => Content::{},",
            &monster.ident, &monster.content
        )?;
    }
    writeln!(output, "        }}")?;
    writeln!(output, "    }}")?;
    writeln!(output, "    pub(crate) fn color(self) -> Color {{")?;
    writeln!(output, "        #[allow(clippy::match_same_arms)]")?;
    writeln!(output, "        match self {{")?;
    for monster in &monsters {
        writeln!(
            output,
            "            Monster::{} => Color::{},",
            &monster.ident, &monster.color
        )?;
    }
    writeln!(output, "        }}")?;
    writeln!(output, "    }}")?;
    writeln!(
        output,
        "    pub(crate) fn miniature(self) -> Option<Monster> {{"
    )?;
    writeln!(output, "        #[allow(clippy::match_same_arms)]")?;
    writeln!(output, "        match self {{")?;
    for monster in &monsters {
        if monster.miniature == "self" {
            writeln!(output, "            Monster::{} => None,", &monster.ident)?;
        } else {
            writeln!(
                output,
                "            Monster::{} => Some(Monster::{}),",
                &monster.ident,
                name_to_ident(monster.miniature)
            )?;
        }
    }
    writeln!(output, "        }}")?;
    writeln!(output, "    }}")?;
    writeln!(
        output,
        "    pub(crate) fn name(self, language: GameLanguage) -> &'static str {{"
    )?;
    writeln!(output, "        match language {{")?;
    writeln!(output, "            GameLanguage::En => match self {{")?;
    for monster in &monsters {
        writeln!(
            output,
            "                Monster::{} => {:?},",
            &monster.ident, &monster.name_en
        )?;
    }
    writeln!(output, "            }},")?;
    writeln!(output, "            GameLanguage::De => match self {{")?;
    for monster in &monsters {
        writeln!(
            output,
            "                Monster::{} => {:?},",
            &monster.ident, &monster.name_de
        )?;
    }
    writeln!(output, "            }},")?;
    writeln!(output, "        }}")?;
    writeln!(output, "    }}")?;
    writeln!(output, "}}")?;

    Ok((output, monsters))
}

pub(crate) struct Mns {
    content: Content,
    pub(crate) name_en: &'static str,
    pub(crate) color: Color,
    miniature: &'static str,
    name_de: &'static str,
    pub(crate) ident: String,
}

fn monster_read(line: Vec<&'static str>) -> anyhow::Result<Mns> {
    let content =
        Content::from_str(line[0]).map_err(|_| anyhow!("Unknown content: {}", line[0]))?;
    let name_en = line[1];
    let color = Color::from_str(line[2]).map_err(|_| anyhow!("Unknown color: {}", line[2]))?;
    let miniature = line[3];
    let name_de = line[4];
    let ident = name_to_ident(name_en);

    Ok(Mns {
        content,
        name_en,
        color,
        miniature,
        name_de,
        ident,
    })
}

fn name_to_ident(name: &str) -> String {
    name.replace(|c| !char::is_alphanumeric(c), "")
}
