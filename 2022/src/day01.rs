use crate::AocDay;

type Day = AocDay01;
#[cfg(test)]
static PUZZLE_INDEX: usize = 1;

pub struct AocDay01 {
    calories_sum: Vec<i64>,
}

impl AocDay for Day {
    type R1 = i64;
    type R2 = i64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut elves = Vec::new();
        let mut elf: Vec<i64> = Vec::new();

        for line in lines {
            if line.is_empty() {
                elves.push(elf);
                elf = Vec::new();
            } else {
                elf.push(line.parse().expect("Malformed line"))
            }
        }
        elves.push(elf);

        let calories_sum = elves.iter().map(|x| x.iter().sum()).collect();

        Self { calories_sum }
    }

    fn part1(&self) -> Self::R1 {
        self.calories_sum.iter().max().unwrap().to_owned()
    }

    fn part2(&self) -> Self::R2 {
        let mut vec: Vec<i64> = self.calories_sum.to_owned();
        vec.sort();
        vec.reverse();
        vec[0..=2].iter().sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(24_000, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(45_000, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
