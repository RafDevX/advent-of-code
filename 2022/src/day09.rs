use std::collections::HashSet;

use crate::AocDay;

type Day = AocDay09;
#[cfg(test)]
static PUZZLE_INDEX: usize = 9;

pub struct AocDay09 {
    visited_one: HashSet<Coord>,
    visited_nine: HashSet<Coord>,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Coord(isize, isize);

impl Coord {
    fn move_by(&mut self, mv: &Move) {
        let unit = mv.0.unit();
        self.0 += mv.1 * unit.0;
        self.1 += mv.1 * unit.1;
    }

    fn move_towards(&mut self, dir: &Direction) {
        self.move_by(&Move(dir.clone(), 1));
    }

    fn normalize_coord(coord: isize, norm: f64) -> isize {
        let val = (coord as f64) / norm;
        if val >= 0.0 {
            val.ceil() as isize
        } else {
            val.floor() as isize
        }
    }

    fn get_base_unit(&self) -> Self {
        let norm = self.norm();
        Self(
            Self::normalize_coord(self.0, norm),
            Self::normalize_coord(self.1, norm),
        )
    }

    fn norm(&self) -> f64 {
        f64::sqrt((self.0.pow(2) + self.1.pow(2)) as f64)
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Custom(Coord),
}

impl Direction {
    fn unit(&self) -> Coord {
        match self {
            Direction::Up => Coord(0, 1),
            Direction::Down => Coord(0, -1),
            Direction::Left => Coord(-1, 0),
            Direction::Right => Coord(1, 0),
            Direction::Custom(unit) => unit.to_owned(),
        }
    }
}

#[derive(Debug)]
struct Move(Direction, isize);

impl Move {
    fn step_towards(from: &Coord, to: &Coord) -> Option<Self> {
        let delta = Coord(to.0 - from.0, to.1 - from.1);
        let norm = delta.norm();
        if norm <= f64::sqrt(2.0) {
            None
        } else {
            Some(Move(Direction::Custom(delta.get_base_unit()), 1))
        }
    }
}

impl From<String> for Move {
    fn from(line: String) -> Self {
        let mut split = line.split_whitespace();
        Self(
            match split.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction"),
            },
            split.next().unwrap().parse().unwrap(),
        )
    }
}

impl Day {
    fn simulate(moves: &Vec<Move>, n_tails: usize) -> HashSet<Coord> {
        let mut knots = vec![Coord(0, 0)];
        let mut visited = HashSet::new();
        visited.insert(Coord(0, 0));

        for _ in 0..n_tails {
            knots.push(Coord(0, 0));
        }

        for mv in moves {
            for _ in 0..mv.1 {
                knots[0].move_towards(&mv.0);
                for i in 1..(n_tails + 1) {
                    let adjustment = Move::step_towards(&knots[i], &knots[i - 1]);
                    if let Some(step) = adjustment {
                        knots[i].move_by(&step);
                        if i == n_tails {
                            visited.insert(knots[i].clone());
                        }
                    }
                }
            }
        }

        visited
    }
}

impl AocDay for Day {
    type R1 = usize;
    type R2 = usize;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut moves = vec![];
        for line in lines {
            moves.push(line.into());
        }

        let visited_one = Self::simulate(&moves, 1);
        let visited_nine = Self::simulate(&moves, 9);

        Self {
            visited_one,
            visited_nine,
        }
    }

    fn part1(&self) -> Self::R1 {
        self.visited_one.len()
    }

    fn part2(&self) -> Self::R2 {
        self.visited_nine.len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(13, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(1, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
