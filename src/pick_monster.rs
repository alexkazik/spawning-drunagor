use crate::game::{Color, Level, Monster};
use rand::RngExt;
use rand::rngs::SmallRng;
use rand::seq::{IndexedRandom, SliceRandom};
use std::collections::{HashMap, HashSet};

pub(crate) fn pick_monster(
    avail: Vec<Monster>,
    todo: &HashMap<Color, HashSet<Level>>,
) -> Option<HashMap<(Color, Level), Monster>> {
    let mut avail_by_color = HashMap::new();
    for m in avail {
        avail_by_color
            .entry(m.color())
            .or_insert_with(Vec::new)
            .push(m);
    }

    let mut best: Option<(_, _, _)> = None;

    for _ in 0..20 {
        // `?`: no solution found -> exit
        let (r, missing_dist) = pick_monster_single(&mut avail_by_color, todo)?;
        let Some((missing, dist)) = missing_dist else {
            // has picked <= 1 monster with initiative -> no way to check for best, take it
            return Some(r);
        };
        if let Some(b) = &best {
            if b.1 != missing {
                // different amount of missing -> can't check for best, take it
                return Some(r);
            }
            if dist < b.2 {
                best = Some((r, missing, dist));
            }
        } else {
            best = Some((r, missing, dist));
        }
    }

    best.map(|(r, _, _)| r)
}

#[allow(clippy::type_complexity)]
fn pick_monster_single(
    avail_by_color: &mut HashMap<Color, Vec<Monster>>,
    todo: &HashMap<Color, HashSet<Level>>,
) -> Option<(HashMap<(Color, Level), Monster>, Option<(usize, i32)>)> {
    let mut rng: SmallRng = rand::make_rng();

    let mut r = HashMap::new();
    for (co, levels) in todo {
        // `?`: if not a single monster exists -> exit
        let ms = avail_by_color.get_mut(co)?;
        if levels.len() > 1 && ms.len() >= levels.len() {
            // one monster per level
            let (ms, _) = ms.partial_shuffle(&mut rng, levels.len());
            for (le, m) in levels.iter().zip(ms) {
                r.insert((*co, *le), *m);
            }
        } else {
            // one monster for all levels (exit if empty)
            let m = ms.choose(&mut rng)?;
            for le in levels {
                r.insert((*co, *le), *m);
            }
        }
    }

    let mut ini = r
        .values()
        .filter_map(|m| m.initiative())
        .collect::<Vec<_>>();
    if ini.len() <= 1 {
        // one or fewer monsters with initiative: do not calculate distance
        return Some((r, None));
    }
    ini.sort_unstable();

    eprint!("{ini:?}");

    let missing = r.len() - ini.len();
    let accum = (0, ini.last().unwrap() - 10);

    let dist = ini
        .into_iter()
        .fold(accum, |(dist, last), this| {
            let new_dist = match this - last {
                0 => 4,
                1 => 2,
                2 => 1,
                _ => 0,
            };
            (dist + new_dist, this)
        })
        .0;

    eprint!(" => {dist}");
    let dist = dist + rng.random_range(-2..=2);
    eprintln!(" => {dist}");

    Some((r, Some((missing, dist))))
}
