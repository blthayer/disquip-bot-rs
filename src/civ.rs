use rand::rng;
use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

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
    all_leaders.shuffle(&mut rng());
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
}
