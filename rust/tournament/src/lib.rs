use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Clone, Eq)]
struct Score {
    name: String,
    matches: i32,
    wins: i32,
    losses: i32,
    draws: i32,
    scores: i32,
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        let score_cmp = other.scores.cmp(&self.scores);
        let match_cmp = other.matches.cmp(&self.matches);
        let wins_cmp = other.wins.cmp(&self.wins);
        let draws_cmp = other.draws.cmp(&self.draws);
        let loses_cmp = self.losses.cmp(&other.losses);
        let name_cmp = self.name.cmp(&other.name);
        if score_cmp == Ordering::Equal {
            if match_cmp == Ordering::Equal {
                if wins_cmp == Ordering::Equal {
                    if draws_cmp == Ordering::Equal {
                        if loses_cmp == Ordering::Equal {
                            return name_cmp;
                        }
                        return loses_cmp;
                    }
                    return draws_cmp;
                }
                return match_cmp;
            }
            return wins_cmp;
        }
        score_cmp
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn tally(match_results: &str) -> String {
    let results: Vec<Vec<&str>> = match_results
        .split('\n')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split(';').collect::<Vec<&str>>())
        .collect();
    let mut scores: HashMap<&str, Score> = HashMap::new();

    for result in results {
        if result.len() < 3 {
            break;
        }
        let team1 = result[0];
        let team2 = result[1];
        let r = result[2];
        match (team1, team2, r) {
            (t1, t2, "win") => {
                let mut score1 = scores.entry(t1).or_default();
                score1.name = t1.to_string();
                score1.wins += 1;
                score1.matches += 1;
                score1.scores += 3;
                let mut score2 = scores.entry(t2).or_default();
                score2.name = t2.to_string();
                score2.losses += 1;
                score2.matches += 1;
            }
            (t1, t2, "draw") => {
                let mut score1 = scores.entry(t1).or_default();
                score1.name = t1.to_string();
                score1.draws += 1;
                score1.scores += 1;
                score1.matches += 1;
                let mut score2 = scores.entry(t2).or_default();
                score2.name = t2.to_string();
                score2.draws += 1;
                score2.scores += 1;
                score2.matches += 1;
            }
            (t1, t2, "loss") => {
                let mut score1 = scores.entry(t1).or_default();
                score1.name = t1.to_string();
                score1.losses += 1;
                score1.matches += 1;
                let mut score2 = scores.entry(t2).or_default();
                score2.name = t2.to_string();
                score2.wins += 1;
                score2.scores += 3;
                score2.matches += 1;
            }
            _ => panic!("unmatched result"),
        }
    }

    let mut sc: Vec<Score> = scores.values().cloned().collect();
    sc.sort_unstable();
    let mut results: Vec<String> =
        vec!["Team                           | MP |  W |  D |  L |  P".to_string()];
    for score in sc {
        results.push(format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            score.name, score.matches, score.wins, score.draws, score.losses, score.scores
        ))
    }

    results.join("\n")
}
