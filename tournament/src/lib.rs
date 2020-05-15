#[macro_use]
macro_rules! output_format {
    () => {
        "{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}"
    };
}

use std::collections::BTreeMap;

#[derive(Default)]
pub struct Stats {
    pub wins: i32,
    pub draws: i32,
    pub losses: i32,
}

impl Stats {
    pub fn points(&self) -> i32 {
        self.wins * 3 + self.draws
    }

    pub fn matches(&self) -> i32 {
        self.wins + self.draws + self.losses
    }
}

pub fn tally(match_results: &str) -> String {
    // process input
    let mut teams: BTreeMap<&str, Stats> = BTreeMap::new();
    for line in match_results.lines() {
        let input: Vec<&str> = line.split(';').collect();
        match input[2] {
            "win" => {
                teams.entry(input[0]).or_insert_with(Stats::default).wins += 1;
                teams.entry(input[1]).or_insert_with(Stats::default).losses += 1;
            }
            "draw" => {
                teams.entry(input[0]).or_insert_with(Stats::default).draws += 1;
                teams.entry(input[1]).or_insert_with(Stats::default).draws += 1;
            }
            "loss" => {
                teams.entry(input[0]).or_insert_with(Stats::default).losses += 1;
                teams.entry(input[1]).or_insert_with(Stats::default).wins += 1;
            }
            _ => panic!("Bad result input"),
        }
    }
    // format results
    let mut results_table = Vec::new();
    results_table.push(format!(output_format!(), "Team", "MP", "W", "D", "L", "P"));
    for (name, stats) in teams {
        results_table.push(format!(
            output_format!(),
            name,
            stats.matches(),
            stats.wins,
            stats.draws,
            stats.losses,
            stats.points()
        ));
    }
    // sort results
    let wins_position = results_table[0].find('W').unwrap();
    results_table[1..].sort_by(|a, b| b[wins_position..].cmp(&a[wins_position..]));
    results_table.join("\n")
}
