use crate::AocDay;

type Day = AocDay02;
#[cfg(test)]
static PUZZLE_INDEX: usize = 2;

pub struct AocDay02 {
    // (First Col, Second Col [Self Move Interpretation], Second Col [Outcome Interpretation])
    rounds: Vec<(OpponentMove, SelfMove, Outcome)>,
}

#[derive(Debug, Copy, Clone)]
enum OpponentMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum SelfMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Day {
    fn calc_score(info: Vec<(SelfMove, Outcome)>) -> i64 {
        let mut score = 0;

        for round in info.iter() {
            score += match round.0 {
                SelfMove::Rock => 1,
                SelfMove::Paper => 2,
                SelfMove::Scissors => 3,
            };

            score += match round.1 {
                Outcome::Loss => 0,
                Outcome::Draw => 3,
                Outcome::Win => 6,
            };
        }

        score
    }
}

impl AocDay for Day {
    type R1 = i64;
    type R2 = i64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut rounds = vec![];

        for line in lines {
            rounds.push((
                match line.chars().next() {
                    Some('A') => OpponentMove::Rock,
                    Some('B') => OpponentMove::Paper,
                    Some('C') => OpponentMove::Scissors,
                    _ => panic!("Unknown opponent move"),
                },
                match line.chars().nth(2) {
                    Some('X') => SelfMove::Rock,
                    Some('Y') => SelfMove::Paper,
                    Some('Z') => SelfMove::Scissors,
                    _ => panic!("Unknown self move"),
                },
                match line.chars().nth(2) {
                    Some('X') => Outcome::Loss,
                    Some('Y') => Outcome::Draw,
                    Some('Z') => Outcome::Win,
                    _ => panic!("Unknown outcome"),
                },
            ))
        }

        Self { rounds }
    }

    fn part1(&self) -> Self::R1 {
        let info = self
            .rounds
            .iter()
            .map(|x| {
                (
                    x.1,
                    match x {
                        (OpponentMove::Rock, SelfMove::Rock, _) => Outcome::Draw,
                        (OpponentMove::Rock, SelfMove::Paper, _) => Outcome::Win,
                        (OpponentMove::Rock, SelfMove::Scissors, _) => Outcome::Loss,
                        (OpponentMove::Paper, SelfMove::Paper, _) => Outcome::Draw,
                        (OpponentMove::Paper, SelfMove::Rock, _) => Outcome::Loss,
                        (OpponentMove::Paper, SelfMove::Scissors, _) => Outcome::Win,
                        (OpponentMove::Scissors, SelfMove::Scissors, _) => Outcome::Draw,
                        (OpponentMove::Scissors, SelfMove::Rock, _) => Outcome::Win,
                        (OpponentMove::Scissors, SelfMove::Paper, _) => Outcome::Loss,
                    },
                )
            })
            .collect();

        Day::calc_score(info)
    }

    fn part2(&self) -> Self::R2 {
        let info = self
            .rounds
            .iter()
            .map(|x| {
                (
                    match x {
                        (OpponentMove::Rock, _, Outcome::Loss) => SelfMove::Scissors,
                        (OpponentMove::Rock, _, Outcome::Draw) => SelfMove::Rock,
                        (OpponentMove::Rock, _, Outcome::Win) => SelfMove::Paper,
                        (OpponentMove::Paper, _, Outcome::Loss) => SelfMove::Rock,
                        (OpponentMove::Paper, _, Outcome::Draw) => SelfMove::Paper,
                        (OpponentMove::Paper, _, Outcome::Win) => SelfMove::Scissors,
                        (OpponentMove::Scissors, _, Outcome::Loss) => SelfMove::Paper,
                        (OpponentMove::Scissors, _, Outcome::Draw) => SelfMove::Scissors,
                        (OpponentMove::Scissors, _, Outcome::Win) => SelfMove::Rock,
                    },
                    x.2,
                )
            })
            .collect();

        Day::calc_score(info)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(15, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(12, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
