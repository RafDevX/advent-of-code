use crate::AocDay;

type Day = AocDay03;
#[cfg(test)]
static PUZZLE_INDEX: usize = 3;

pub struct AocDay03 {
    rucksacks: Vec<Rucksack>,
}

struct Rucksack {
    all: String,
    compartments: (String, String),
}

impl Day {
    fn get_priority(c: char) -> i64 {
        assert!(c.is_alphabetic());
        let base = if c.is_ascii_lowercase() {
            ('a', 1)
        } else {
            ('A', 27)
        };

        ((c as usize) - (base.0 as usize) + base.1)
            .try_into()
            .unwrap()
    }
}

impl AocDay for Day {
    type R1 = i64;
    type R2 = i64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut rucksacks = vec![];

        for line in lines {
            let len = line.len(); // we have guarantee that 1 char = 1 byte for this input
            assert_eq!(0, len % 2, "Line size {} not even!", len);
            let half = len / 2;
            rucksacks.push(Rucksack {
                compartments: (line[..half].to_owned(), line[half..].to_owned()),
                all: line,
            })
        }

        Self { rucksacks }
    }

    fn part1(&self) -> Self::R1 {
        let mut priorities = vec![];

        for rucksack in self.rucksacks.iter() {
            for c in rucksack.compartments.0.chars() {
                // input is small enough for this to be acceptable efficiency
                if rucksack.compartments.1.chars().any(|x| x == c) {
                    priorities.push(Day::get_priority(c));
                    break;
                }
            }
            // assuming there's always a repeated one
        }

        priorities.iter().sum()
    }

    fn part2(&self) -> Self::R2 {
        let mut priorities = vec![];

        for group in self.rucksacks.chunks(3) {
            for c in group[0].all.chars() {
                if group[1].all.chars().any(|x| x == c) && group[2].all.chars().any(|x| x == c) {
                    priorities.push(Day::get_priority(c));
                    break;
                }
            }
        }

        priorities.iter().sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(157, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(70, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
