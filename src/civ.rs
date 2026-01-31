use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Leader {
    #[serde(rename = "Leader")]
    name: String,
    #[serde(rename = "Civilization")]
    civ: String,
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
}
