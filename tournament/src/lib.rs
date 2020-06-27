use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

enum MatchResult {
    Win,
    Draw,
    Loss,
}
struct Team<T: Display> {
    name: String,
    matches_played: T,
    wins: T,
    draws: T,
    losses: T,
    points: T,
}

impl Team<u16> {
    fn new(name: String) -> Self {
        Team {
            name,
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        }
    }

    fn set_result(&mut self, result: MatchResult) -> &mut Self {
        match result {
            MatchResult::Win => self.win(),
            MatchResult::Loss => self.loss(),
            MatchResult::Draw => self.draw(),
        }
    }
    fn win(&mut self) -> &mut Self {
        self.matches_played += 1;
        self.wins += 1;
        self.points += 3;
        self
    }

    fn loss(&mut self) -> &mut Self {
        self.matches_played += 1;
        self.losses += 1;
        self
    }

    fn draw(&mut self) -> &mut Self {
        self.matches_played += 1;
        self.draws += 1;
        self.points += 1;
        self
    }
}

fn formatter<T: Display>(format: &Team<T>) -> String {
    format!(
        "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
        format.name, format.matches_played, format.wins, format.draws, format.losses, format.points
    )
}

pub fn tally(match_results: &str) -> String {
    let mut result: HashMap<String, Team<u16>> = HashMap::new();

    macro_rules! set_result {
        ($t:expr, $u:expr) => {
            result
                .entry($t.to_string())
                .or_insert_with(|| Team::new($t.to_string()))
                .set_result($u);
        };
    }
    let header = formatter(&Team {
        name: "Team".to_string(),
        matches_played: "MP",
        wins: "W",
        draws: "D",
        losses: "L",
        points: "P",
    });
    match_results.lines().for_each(|line| {
        let match_result = line.split(';').collect::<Vec<_>>();
        match match_result[2] {
            "win" => {
                set_result!(match_result[0], MatchResult::Win);
                set_result!(match_result[1], MatchResult::Loss);
            }
            "loss" => {
                set_result!(match_result[0], MatchResult::Loss);
                set_result!(match_result[1], MatchResult::Win);
            }
            "draw" => {
                set_result!(match_result[0], MatchResult::Draw);
                set_result!(match_result[1], MatchResult::Draw);
            }
            _ => unreachable!("Teams can only win, lose or draw!"),
        }
    });

    let mut teams = result.iter().map(|(_, team)| team).collect::<Vec<_>>();
    teams.sort_by(|a, b| match b.points.partial_cmp(&a.points) {
        Some(Ordering::Equal) => a.name.partial_cmp(&b.name).unwrap(),
        Some(x) => x,
        None => unreachable!("Teams always have points defined"),
    });

    let teams_result = teams
        .iter()
        .map(|&t| formatter(t))
        .collect::<Vec<_>>()
        .join("\n");

    if teams_result.is_empty() {
        return header;
    }
    format!("{}\n{}", header, teams_result)
}
