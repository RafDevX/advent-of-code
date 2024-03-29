use std::collections::VecDeque;

use crate::AocDay;

type Day = AocDay06;
#[cfg(test)]
static PUZZLE_INDEX: usize = 6;

pub struct AocDay06 {
    input: String,
}

impl AocDay06 {
    fn has_duplicates(deq: &VecDeque<char>) -> bool {
        for (i, el) in deq.iter().enumerate() {
            for (j, x) in deq.iter().enumerate() {
                if i != j && x == el {
                    return true;
                }
            }
        }

        false
    }

    fn find_first_unique_part(&self, len: usize) -> Option<usize> {
        let mut deq = VecDeque::new();

        for (i, c) in self.input.chars().enumerate() {
            deq.push_back(c);
            if deq.len() > len {
                deq.pop_front();
                if !Self::has_duplicates(&deq) {
                    return Some(i + 1);
                }
            }
        }

        None
    }
}

impl AocDay for Day {
    type R1 = i64;
    type R2 = i64;

    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        Self {
            input: lines.next().expect("Input must have at least one line"),
        }
    }

    fn part1(&self) -> Self::R1 {
        self.find_first_unique_part(4).unwrap().try_into().unwrap()
    }

    fn part2(&self) -> Self::R2 {
        self.find_first_unique_part(14).unwrap().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(11, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(26, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
