use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
};

use crate::AocDay;

type Day = AocDay15;
#[cfg(test)]
static PUZZLE_INDEX: usize = 15;

pub struct AocDay15 {
    covered_per_row: HashMap<i64, Vec<CoveredRange>>,
    objects_per_row: HashMap<i64, HashSet<i64>>,
}

type CoveredRange = RangeInclusive<i64>;

#[derive(Debug)]
struct Point(i64, i64);

impl Point {
    fn manhattan_distance(&self, other: &Self) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(", ");
        if let Some(x) = split.next().unwrap().strip_prefix("x=") {
            if let Some(y) = split.next().unwrap().strip_prefix("y=") {
                return Ok(Point(y.parse().unwrap(), x.parse().unwrap()));
            }
        }

        Err(ParseError)
    }
}

// (sensor, beacon)
#[derive(Debug)]
struct Sensor(Point, Point);

impl Sensor {
    fn get_covered_ranges(&self) -> Vec<(i64, CoveredRange)> {
        let dist = self.0.manhattan_distance(&self.1) as i64;
        let mut ranges = vec![];

        for i in (self.0 .0 - dist)..=(self.0 .0 + dist) {
            let radius = dist.abs_diff(i.abs_diff(self.0 .0) as i64) as i64;
            ranges.push((i, (self.0 .1 - radius)..=(self.0 .1 + radius)));
        }

        ranges
    }
}

impl FromStr for Sensor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");
        if let Some(sensor) = split.next().unwrap().strip_prefix("Sensor at ") {
            if let Some(beacon) = split.next().unwrap().strip_prefix("closest beacon is at ") {
                return Ok(Self(sensor.parse()?, beacon.parse()?));
            }
        }

        Err(ParseError)
    }
}

#[derive(Debug)]
struct ParseError;

impl Day {
    fn merge_new_range(ranges: &mut Vec<CoveredRange>, new: CoveredRange) {
        let mut inserting = (new.start(), new.end());
        let mut new_ranges = vec![];

        for range in ranges.iter() {
            if range.start() <= inserting.0 && inserting.1 <= range.end() {
                return; // fully contained within existing
            } else if inserting.0 <= range.start() && range.end() <= inserting.1 {
                continue; // fully contains existing
            } else if range.end() + 1 == *inserting.0 {
                inserting.0 = range.start() // adjacent
            } else if inserting.1 + 1 == *range.start() {
                inserting.1 = range.end() // adjacent
            } else if range.start() <= inserting.0
                && inserting.0 <= range.end()
                && range.end() <= inserting.1
            {
                inserting.0 = range.start();
            } else if inserting.0 <= range.start()
                && range.start() <= inserting.1
                && inserting.1 <= range.end()
            {
                inserting.1 = range.end();
            } else {
                new_ranges.push(range.to_owned());
            }
        }

        new_ranges.push(*inserting.0..=*inserting.1);

        *ranges = new_ranges;
    }

    fn find_beacon_by_constraint(&self, min_coord: i64, max_coord: i64) -> Option<Point> {
        'outer: for i in min_coord..=max_coord {
            if let Some(ranges) = self.covered_per_row.get(&i) {
                for range in ranges.iter() {
                    let mut search = None;

                    if *range.start() <= min_coord && max_coord <= *range.end() {
                        continue 'outer;
                    // } else if *range.start() <= j {
                    //     j = range.end() + 1;
                    // } else if j > max_coord {
                    //     continue 'outer;
                    // } else {
                    //     return Some(Point(i, j));
                    // }
                    } else if min_coord <= *range.start() {
                        search = Some(min_coord..*range.start());
                    } else if *range.end() <= max_coord {
                        search = Some(*range.end()..max_coord);
                    }

                    'inner: for j in search.unwrap_or(0..0) {
                        for r in ranges.iter() {
                            if *r.start() <= j && j <= *r.end() {
                                continue 'inner;
                            }
                        }

                        return Some(Point(i, j));
                    }
                }
            } else {
                return Some(Point(i, min_coord));
            }
        }

        None
    }
}

impl AocDay for Day {
    type R1 = usize;
    type R2 = usize;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut covered_per_row = HashMap::new();
        let mut objects_per_row: HashMap<i64, HashSet<i64>> = HashMap::new();

        for line in lines {
            let sensor: Sensor = line.parse().unwrap();
            (*objects_per_row.entry(sensor.0 .0).or_default()).insert(sensor.0 .1);
            (*objects_per_row.entry(sensor.1 .0).or_default()).insert(sensor.1 .1);

            for (row, range) in sensor.get_covered_ranges() {
                Self::merge_new_range(covered_per_row.entry(row).or_default(), range);
            }
        }

        Self {
            covered_per_row,
            objects_per_row,
        }
    }

    fn part1(&self) -> Self::R1 {
        #[cfg(test)]
        let row = 10;
        #[cfg(not(test))]
        let row = 2000000;

        let covered: usize = self
            .covered_per_row
            .get(&row)
            .unwrap()
            .to_owned()
            .iter_mut()
            .map(|r| r.count())
            .sum();

        covered - self.objects_per_row.get(&row).unwrap().len()
    }

    fn part2(&self) -> Self::R2 {
        #[cfg(test)]
        let max_bound = 20;
        #[cfg(not(test))]
        let max_bound = 4_000_000;

        if let Some(Point(y, x)) = self.find_beacon_by_constraint(0, max_bound) {
            (x * 4_000_000 + y) as usize
        } else {
            panic!("Could not find beacon!")
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(26, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(56_000_011, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
