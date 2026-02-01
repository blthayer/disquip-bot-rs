use rand::Rng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

pub const GAME_MODES: [&str; 8] = [
    "Apocalpyse",
    "Barbarian Clans",
    "Dramatic Ages",
    "Heroes & Legends",
    "Monopolies and Corporations",
    "Secrete Societies",
    "Tech and Civic Shuffle",
    "Zombie Defense",
];

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Leader {
    #[serde(rename = "Leader")]
    pub name: String,
    #[serde(rename = "Civilization")]
    pub civ: String,
}

fn read_leaders() -> Vec<Leader> {
    let file = File::open("leaders.csv").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .trim(csv::Trim::All)
        .from_reader(file);
    let mut leaders: Vec<Leader> = Vec::new();
    for result in reader.deserialize() {
        let record: Leader = result.unwrap();
        leaders.push(record);
    }
    leaders
}

pub fn draw_leaders(n: usize) -> Vec<Leader> {
    // Could take leaders as input, but there's really no reason to...
    let mut all_leaders = read_leaders();
    // Since we're forbidding duplicate leaders and civs we cannot just
    // draw n_players * n_leaders
    all_leaders.shuffle(&mut rand::rng());
    let mut out: Vec<Leader> = Vec::with_capacity(n);
    let mut civs: HashSet<String> = HashSet::new();
    let mut names: HashSet<String> = HashSet::new();

    // A while loop feels more natural, but Rust's ownership
    // model essentially forces a for loop here.
    for leader in all_leaders {
        if !civs.contains(&leader.civ) && !names.contains(&leader.name) {
            // Feels bad to clone here, but I'm immediately seeing an
            // obvious way around it.
            civs.insert(leader.civ.clone());
            names.insert(leader.name.clone());
            out.push(leader);
        }

        if out.len() == n {
            break;
        }
    }
    // Technically, we should raise an error or something if out.len() < n.
    // But... let's keep it simple instead.
    out
}

/// If n is provided, draw that many modes.
/// If exclude is provided, treat as 1-based indices into
/// GAME_MODES.
pub fn draw_modes(n: Option<usize>, exclude: Option<&[usize]>) -> Vec<&'static str> {
    let mut rng = rand::rng();

    let indices = match exclude {
        Some(i) => i,
        None => &[],
    };

    let mut modes: Vec<&'static str> = Vec::with_capacity(GAME_MODES.len());

    // There's probably a more elegant way to do this, but we're talking
    // about just a few modes...
    for (idx, mode) in GAME_MODES.iter().enumerate() {
        // 1-based indexing as input, so add 1 to get 0-based.
        if !indices.contains(&(idx + 1)) {
            modes.push(mode);
        }
    }

    let n = match n {
        Some(n) => n,
        None => rng.random_range(0..modes.len()),
    };

    modes.shuffle(&mut rng);

    modes[0..n].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_leaders() {
        let results = read_leaders();
        let catherine = Leader {
            name: "Catherine De Medici (Black Queen)".to_string(),
            civ: "French".to_string(),
        };
        assert_eq!(catherine, results[6]);
    }

    #[test]
    fn test_draw_leaders() {
        for n in 1..50 {
            let leaders = draw_leaders(n);
            assert_eq!(leaders.len(), n);
        }
    }

    #[test]
    /// Basic test with no args.
    fn test_draw_modes() {
        let mut t: f64 = 0.0;
        for _ in 0..1000 {
            let modes = draw_modes(None, None);
            t += modes.len() as f64;
        }
        // Average length should be pretty close over 1000 trials.
        let avg = t / 1000.0;
        // Not gonna do the math here, but this should succeed the vast
        // majority of runs. Yes, I've written a flaky test...
        assert!((avg - 3.5).abs() < 0.1);
    }

    #[test]
    fn test_draw_modes_exclude() {
        assert!(GAME_MODES.contains(&"Apocalpyse"));
        assert!(GAME_MODES.contains(&"Monopolies and Corporations"));

        let exclude: [usize; 2] = [1, 5];
        let mut t: f64 = 0.0;
        for _ in 0..1000 {
            let modes = draw_modes(None, Some(&exclude));
            assert!(!modes.contains(&"Apocalpyse"));
            assert!(!modes.contains(&"Monopolies and Corporations"));
            t += modes.len() as f64;
        }
        // Average length should be pretty close over 1000 trials.
        let avg = t / 1000.0;
        // Not gonna do the math here, but this should succeed the vast
        // majority of runs. Yes, I've written a flaky test...
        assert!((avg - 2.5).abs() < 0.1);
    }

    #[test]
    fn test_draw_leaders_n() {
        for n in 0..GAME_MODES.len() {
            let modes = draw_modes(Some(n), None);
            assert_eq!(n, modes.len());
        }
    }

    #[test]
    fn test_draw_leaders_n_and_exclude() {
        assert!(GAME_MODES.contains(&"Dramatic Ages"));
        assert!(GAME_MODES.contains(&"Zombie Defense"));
        let exclude: [usize; 2] = [3, 8];
        for _ in 0..1000 {
            for n in 0..(GAME_MODES.len() - exclude.len()) {
                let modes = draw_modes(Some(n), Some(&exclude));
                assert_eq!(n, modes.len());

                assert!(!modes.contains(&"Dramatic Ages"));
                assert!(!modes.contains(&"Zombie Defense"));
            }
        }
    }
}
