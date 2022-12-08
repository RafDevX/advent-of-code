use crate::AocDay;

type Day = AocDay00;
#[cfg(test)]
static PUZZLE_INDEX: usize = 0;

pub struct AocDay00 {}

impl AocDay for Day {
    type R = i64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        todo!()
    }

    fn part1(&self) -> Self::R {
        todo!()
    }

    fn part2(&self) -> Self::R {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(-1, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(-1, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
