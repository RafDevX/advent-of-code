use crate::AocDay;
use std::{collections::HashMap, ops::Add, str::FromStr};

type Day = AocDay14;
#[cfg(test)]
static PUZZLE_INDEX: usize = 14;

#[derive(Clone)]
pub struct AocDay14 {
    grid: HashMap<Pos, TileType>,
    lower_y_bound: Option<usize>,
}

#[derive(Debug, Clone)]
enum TileType {
    Rock,
    Sand,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn points_between(&self, other: &Self) -> impl Iterator<Item = Pos> {
        let points: Vec<Pos> = match (self, other) {
            (Pos(sx, sy), Pos(ox, oy)) if sx == ox => {
                ord_range(*sy, *oy).map(|y| Pos(*sx, y)).collect()
            }
            (Pos(sx, sy), Pos(ox, oy)) if sy == oy => {
                ord_range(*sx, *ox).map(|x| Pos(x, *sy)).collect()
            }
            _ => panic!("Not a straight line"),
        };

        points.into_iter()
    }
}

// needed because len(7..2) != len(2..7)
fn ord_range<T: PartialOrd + Add<usize, Output = T>>(x: T, y: T) -> std::ops::Range<T> {
    if x <= y {
        x..(y + 1)
    } else {
        y..(x + 1)
    }
}

impl FromStr for Pos {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tup: Vec<usize> = s.split(',').map(|x| x.parse()).collect::<Result<_, _>>()?;

        Ok(Self(tup[0], tup[1]))
    }
}

struct RockPath(Vec<Pos>);

impl RockPath {
    fn max_y(&self) -> Option<usize> {
        self.0.iter().map(|Pos(_, y)| y).max().copied()
    }

    fn points(&self) -> impl Iterator<Item = Pos> {
        let mut points = vec![];

        for (i, point) in self.0.iter().enumerate() {
            if let Some(next) = self.0.get(i + 1) {
                points.extend(point.points_between(next));
            } else {
                points.push(point.clone());
            }
        }

        points.into_iter()
    }

    fn draw_on_grid(&self, grid: &mut HashMap<Pos, TileType>) {
        for point in self.points() {
            grid.insert(point, TileType::Rock);
        }
    }
}

impl FromStr for RockPath {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;

        Ok(Self(points))
    }
}

#[derive(PartialEq)]
enum SimulationMode {
    Abyss(usize),
    CaveFloor(usize),
}

impl Day {
    // Returns how many units of sand can come to rest before flowing into the abyss below
    fn simulate_sand(&mut self, start: Pos, simulation_mode: SimulationMode) -> usize {
        let mut i = 0;

        loop {
            i += 1;
            let mut sand_pos = start.clone();

            loop {
                match simulation_mode {
                    SimulationMode::Abyss(y_bound) if sand_pos.1 > y_bound => return i - 1,
                    SimulationMode::CaveFloor(y_pos) if sand_pos.1 + 1 >= y_pos => {
                        break;
                    }
                    _ => (),
                }

                let mut next_pos = Pos(sand_pos.0, sand_pos.1 + 1);
                if self.grid.contains_key(&next_pos) && sand_pos.0 > 0 {
                    next_pos = Pos(sand_pos.0 - 1, sand_pos.1 + 1);
                    if self.grid.contains_key(&next_pos) {
                        next_pos = Pos(sand_pos.0 + 1, sand_pos.1 + 1);
                        if self.grid.contains_key(&next_pos) {
                            break;
                        }
                    }
                }

                sand_pos = next_pos;
            }

            let finished = sand_pos == start;

            self.grid.insert(sand_pos, TileType::Sand);

            if finished {
                return i;
            }
        }
    }
}

impl AocDay for Day {
    type R1 = usize;
    type R2 = usize;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut grid = HashMap::new();
        let mut lower_y_bound = None;

        for line in lines {
            let path: RockPath = line.parse().unwrap();
            path.draw_on_grid(&mut grid);

            if let Some(max_y) = path.max_y() {
                if lower_y_bound.is_none() || lower_y_bound.unwrap() < max_y {
                    lower_y_bound = Some(max_y)
                }
            }
        }

        Self {
            grid,
            lower_y_bound,
        }
    }

    fn part1(&self) -> Self::R1 {
        let mut clone = (*self).to_owned();
        clone.simulate_sand(
            Pos(500, 0),
            SimulationMode::Abyss(self.lower_y_bound.unwrap()),
        )
    }

    fn part2(&self) -> Self::R2 {
        let mut clone = (*self).to_owned();
        clone.simulate_sand(
            Pos(500, 0),
            SimulationMode::CaveFloor(self.lower_y_bound.unwrap() + 2),
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(24, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(93, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
