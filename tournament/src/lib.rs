#[macro_use]
macro_rules! output_format {
    () => {
        "{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}"
    };
}

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq)]
pub struct Team {
    pub name: String,
    pub wins: i32,
    pub draws: i32,
    pub losses: i32,
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.points().cmp(&other.points()) {
            Ordering::Equal => match self.wins.cmp(&other.wins) {
                Ordering::Equal => match self.draws.cmp(&other.draws) {
                    Ordering::Equal => self.name.cmp(&other.name),
                    any => any,
                },
                any => any,
            },
            any => any,
        }
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.points() == other.points()
            && self.wins == other.wins
            && self.draws == other.draws
            && self.name == other.name
    }
}

impl Team {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    pub fn points(&self) -> i32 {
        self.wins * 3 + self.draws
    }

    pub fn matches(&self) -> i32 {
        self.wins + self.draws + self.losses
    }
}

pub fn tally(match_results: &str) -> String {
    // process input
    let mut teams: HashMap<&str, Team> = HashMap::new();
    for line in match_results.lines() {
        let input: Vec<&str> = line.split(';').collect();
        match input[2] {
            "win" => {
                teams
                    .entry(input[0])
                    .or_insert_with(|| Team::new(input[0]))
                    .wins += 1;
                teams
                    .entry(input[1])
                    .or_insert_with(|| Team::new(input[1]))
                    .losses += 1;
            }
            "draw" => {
                teams
                    .entry(input[0])
                    .or_insert_with(|| Team::new(input[0]))
                    .draws += 1;
                teams
                    .entry(input[1])
                    .or_insert_with(|| Team::new(input[1]))
                    .draws += 1;
            }
            "loss" => {
                teams
                    .entry(input[0])
                    .or_insert_with(|| Team::new(input[0]))
                    .losses += 1;
                teams
                    .entry(input[1])
                    .or_insert_with(|| Team::new(input[1]))
                    .wins += 1;
            }
            _ => panic!("Bad result input"),
        }
    }
    // format results
    let mut teams: Vec<Team> = teams.into_iter().map(|(_, team)| team).collect();
    teams.sort();
    let mut results_table = Vec::new();
    results_table.push(format!(output_format!(), "Team", "MP", "W", "D", "L", "P"));
    for team in teams {
        results_table.push(format!(
            output_format!(),
            team.name,
            team.matches(),
            team.wins,
            team.draws,
            team.losses,
            team.points()
        ));
    }
    // sort results
    let wins_position = results_table[0].find('W').unwrap();
    results_table[1..].sort_by(|a, b| b[wins_position..].cmp(&a[wins_position..]));
    results_table.join("\n")
}
