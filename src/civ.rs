use ahash::AHashSet;
use rand::RngExt;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use serde::Deserialize;

pub const GAME_MODES: [&str; 8] = [
    "Apocalypse",
    "Barbarian Clans",
    "Dramatic Ages",
    "Heroes & Legends",
    "Monopolies and Corporations",
    "Secret Societies",
    "Tech and Civic Shuffle",
    "Zombie Defense",
];

pub const MAPS: [&str; 30] = [
    "4-Leaf Clover",
    "6-Armed Snowflake",
    "Archipelago",
    "Continents",
    "Continents and Islands",
    "Earth",
    "Earth Huge",
    "East Asia",
    "Europe",
    "Fractal",
    "Highlands",
    "Inland Sea",
    "Island Plates",
    "Lakes",
    "Mediterranean Large",
    "Mirror",
    "Pangaea",
    "Primordial",
    "Seven Seas",
    "Shuffle",
    "Small Continents",
    "Splintered Fractal",
    "Terra",
    "Tilted Axis",
    "True Start Location Earth",
    "True Start Location Earth Huge",
    "True Start Location East Asia",
    "True Start Location Europe",
    "True Start Location Mediterranean",
    "Wetlands",
];

pub const LEADERS: [(&str, &str); 77] = [
    ("Abraham Lincoln", "American"),
    ("Alexander", "Macedonian"),
    ("Amanitore", "Nubian"),
    ("Ambiorix", "Gallic"),
    ("Ba Trieu", "Vietnamese"),
    ("Basil II", "Byzantine"),
    ("Catherine De Medici (Black Queen)", "French"),
    ("Catherine De Medici (Magnificence)", "French"),
    ("Chandragupta", "Indian"),
    ("Cleopatra (Egyptian)", "Egyptian"),
    ("Cleopatra (Ptolemaic)", "Egyptian"),
    ("Cyrus", "Persian"),
    ("Dido", "Phoenician"),
    ("Eleanor of Aquitaine", "English"),
    ("Eleanor of Aquitaine", "French"),
    ("Elizabeth", "English"),
    ("Frederick Barbarossa", "German"),
    ("Gandhi", "Indian"),
    ("Genghis Khan", "Mongolian"),
    ("Gilgamesh", "Sumerian"),
    ("Gitarja", "Indonesian"),
    ("Gorgo", "Greek"),
    ("Hammurabi", "Babylonian"),
    ("Harald Hardrada (Konge)", "Norwegian"),
    ("Harald Hardrada (Varangian)", "Norwegian"),
    ("Hojo Tokimune", "Japanese"),
    ("Jadwiga", "Polish"),
    ("Jayavarman VII", "Khmer"),
    ("Joao III", "Portuguese"),
    ("John Curtin", "Australian"),
    ("Julius Caesar", "Roman"),
    ("Kristina", "Swedish"),
    ("Kublai Khan", "Chinese"),
    ("Kublai Khan", "Mongolian"),
    ("Kupe", "Maori"),
    ("Lady Six Sky", "Mayan"),
    ("Lautaro", "Mapuche"),
    ("Ludwig II", "German"),
    ("Mansa Musa", "Malian"),
    ("Matthias Corvinus", "Hungarian"),
    ("Menelik II", "Ethiopian"),
    ("Montezuma", "Aztec"),
    ("Mvemba a Nzinga", "Kongolese"),
    ("Nader Shah", "Persian"),
    ("Nzinga Mbande", "Kongolese"),
    ("Pachacuti", "Incan"),
    ("Pedro II", "Brazilian"),
    ("Pericles", "Greek"),
    ("Peter", "Russian"),
    ("Philip II", "Spanish"),
    ("Poundmaker", "Cree"),
    ("Qin Shi Huang (Mandate of Heaven)", "Chinese"),
    ("Qin Shi Huang (Unifier)", "Chinese"),
    ("Ramses II", "Egyptian"),
    ("Robert the Bruce", "Scottish"),
    ("Saladin (Sultan)", "Arabian"),
    ("Saladin (Vizier)", "Arabian"),
    ("Sejong", "Korean"),
    ("Seondeok", "Korean"),
    ("Shaka", "Zulu"),
    ("Simon Bolivar", "Gran Colombian"),
    ("Suleiman (Kanuni)", "Ottoman"),
    ("Suleiman (Muhtesem)", "Ottoman"),
    ("Sundiata Keita", "Malian"),
    ("Tamar", "Georgian"),
    ("Teddy Roosevelt (Bull Moose)", "American"),
    ("Teddy Roosevelt (Rough Rider)", "American"),
    ("Theodora", "Byzantine"),
    ("Tokugawa", "Japanese"),
    ("Tomyris", "Scythian"),
    ("Trajan", "Roman"),
    ("Victoria (Age of Empire)", "English"),
    ("Victoria (Age of Steam)", "English"),
    ("Wilfrid Laurier", "Canadian"),
    ("Wilhelmina", "Dutch"),
    ("Wu Zetian", "Chinese"),
    ("Yongle", "Chinese"),
];

pub const CITY_STATE_RANGE: std::ops::RangeInclusive<usize> = 0..=14;
pub const DISASTER_INTENSITY_RANGE: std::ops::RangeInclusive<usize> = 0..=4;
pub const RESOURCES: [&str; 4] = ["Sparse", "Standard", "Abundant", "Random"];
pub const WORLD_AGE: [&str; 4] = ["New", "Standard", "Old", "Random"];
pub const START_POSITION: [&str; 3] = ["Balanced", "Standard", "Legendary"];
pub const TEMPERATURE: [&str; 4] = ["Hot", "Standard", "Cold", "Random"];
pub const RAINFALL: [&str; 4] = ["Arid", "Standard", "Wet", "Random"];
pub const SEA_LEVEL: [&str; 4] = ["Low", "Standard", "High", "Random"];

pub fn draw_from_slice<'a, T>(rng: &mut ThreadRng, array: &'a [T]) -> &'a T {
    // TODO: How to avoid returning a double-reference?
    &array[rng.random_range(0..array.len())]
}

pub fn draw_settings() -> String {
    let mut rng = rand::rng();

    let n_city_states = rng.random_range(CITY_STATE_RANGE);
    let disaster_intensity = rng.random_range(DISASTER_INTENSITY_RANGE);
    let map = *draw_from_slice(&mut rng, &MAPS);
    let resources = *draw_from_slice(&mut rng, &RESOURCES);
    let world_age = *draw_from_slice(&mut rng, &WORLD_AGE);
    let start_position = *draw_from_slice(&mut rng, &START_POSITION);
    let temperature = *draw_from_slice(&mut rng, &TEMPERATURE);
    let rainfall = *draw_from_slice(&mut rng, &RAINFALL);
    let sea_level = *draw_from_slice(&mut rng, &SEA_LEVEL);

    format!(
        "
```
City-States:            {n_city_states}
Disaster Intensity:     {disaster_intensity}
Map:                    {map}
Leader Pool 1:          N/A
Leader Pool 2:          N/A
Resources:              {resources}
Select City-States:     N/A
Select Natural Wonders: N/A
World Age:              {world_age}
Start Position:         {start_position}
Temperature:            {temperature}
Rainfall:               {rainfall}
Sea Level:              {sea_level}
```
        "
    )
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Leader {
    #[serde(rename = "Leader")]
    pub name: String,
    #[serde(rename = "Civilization")]
    pub civ: String,
}

fn read_leaders() -> Vec<Leader> {
    let mut leaders: Vec<Leader> = Vec::with_capacity(LEADERS.len());
    for (name, civ) in LEADERS {
        leaders.push(Leader {
            name: name.to_owned(),
            civ: civ.to_owned(),
        });
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
    let mut civs: AHashSet<String> = AHashSet::new();
    let mut names: AHashSet<String> = AHashSet::new();

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
/// `GAME_MODES`.
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

pub fn draw_map() -> &'static str {
    let mut rng = rand::rng();
    let idx = rng.random_range(0..MAPS.len());
    MAPS[idx]
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
        for _ in 0..10000 {
            let modes = draw_modes(None, None);
            t += modes.len() as f64;
        }
        // Average length should be pretty close over 1000 trials.
        let avg = t / 10000.0;
        // Not gonna do the math here, but this should succeed the vast
        // majority of runs. Yes, I've written a flaky test...
        assert!((avg - 3.5).abs() < 0.1);
    }

    #[test]
    fn test_draw_modes_exclude() {
        assert!(GAME_MODES.contains(&"Apocalypse"));
        assert!(GAME_MODES.contains(&"Monopolies and Corporations"));

        let exclude: [usize; 2] = [1, 5];
        let mut t: f64 = 0.0;
        for _ in 0..10000 {
            let modes = draw_modes(None, Some(&exclude));
            assert!(!modes.contains(&"Apocalypse"));
            assert!(!modes.contains(&"Monopolies and Corporations"));
            t += modes.len() as f64;
        }
        // Average length should be pretty close over 1000 trials.
        let avg = t / 10000.0;
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

    #[test]
    fn test_draw_map() {
        let mut set: AHashSet<&str> = AHashSet::with_capacity(MAPS.len());

        for _ in 0..10000 {
            set.insert(draw_map());
        }

        assert_eq!(set.len(), MAPS.len());
    }
}
