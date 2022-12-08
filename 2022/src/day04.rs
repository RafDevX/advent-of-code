use crate::AocDay;

type Day = AocDay04;
#[cfg(test)]
static PUZZLE_INDEX: usize = 4;

pub struct AocDay04 {
    pairs: Vec<Pair>,
}

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
        let points: Vec<Vec<usize>> = line
            .split(",")
            .map(|x| x.split("-").map(|y| y.parse().unwrap()).collect())
            .collect();

        Pair((points[0][0], points[0][1]), (points[1][0], points[1][1]))
    }
}

impl AocDay for Day {
    type R = i64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut pairs = vec![];

        for line in lines {
            pairs.push(line.into());
        }

        Self { pairs }
    }

    fn part1(&self) -> Self::R {
        self.pairs
            .iter()
            .filter(|x| x.either_fully_contained())
            .count()
            .try_into()
            .unwrap()
    }

    fn part2(&self) -> Self::R {
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
