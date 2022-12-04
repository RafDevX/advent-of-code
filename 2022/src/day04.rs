use crate::AocDay;
use regex::Regex;

pub struct AocDay04 {
    pairs: Vec<Pair>,
}

type Day = AocDay04;
#[cfg(test)]
static PUZZLE_INDEX: usize = 4;

struct Pair((usize, usize), (usize, usize));

impl Pair {
    fn reverse(&self) -> Self {
        Pair(self.1, self.0)
    }

    fn fully_contained(&self) -> bool {
        match self {
            Pair(inner, outer) => inner.0 >= outer.0 && inner.1 <= outer.1,
        }
    }

    fn either_fully_contained(&self) -> bool {
        Pair::fully_contained(self) || Pair::fully_contained(&self.reverse())
    }

    fn overlaps(&self) -> bool {
        match self {
            Pair(a, b) => (a.1 >= b.0) && (a.0 <= b.1),
        }
    }
}

impl From<String> for Pair {
    fn from(line: String) -> Pair {
        // there's probably a better way than to build the regex every time
        let re: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
        let captures = re.captures(&line).unwrap();
        Pair(
            (
                captures.get(1).unwrap().as_str().parse().unwrap(),
                captures.get(2).unwrap().as_str().parse().unwrap(),
            ),
            (
                captures.get(3).unwrap().as_str().parse().unwrap(),
                captures.get(4).unwrap().as_str().parse().unwrap(),
            ),
        )
    }
}

impl AocDay for Day {
    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut pairs = vec![];

        for line in lines {
            pairs.push(line.into());
        }

        Self { pairs }
    }

    fn part1(&self) -> i64 {
        self.pairs
            .iter()
            .filter(|x| x.either_fully_contained())
            .count()
            .try_into()
            .unwrap()
    }

    fn part2(&self) -> i64 {
        self.pairs
            .iter()
            .filter(|x| x.overlaps())
            .count()
            .try_into()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(2, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(4, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
