use crate::AocDay;

type Day = AocDay00;
#[cfg(test)]
static PUZZLE_INDEX: usize = 0;

pub struct AocDay00 {}

impl AocDay for Day {
    type R1 = i64;
    type R2 = i64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        todo!()
    }

    fn part1(&self) -> Self::R1 {
        todo!()
    }

    fn part2(&self) -> Self::R2 {
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
