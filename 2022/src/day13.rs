use std::{cmp::max, str::FromStr};

use crate::AocDay;
use itertools::Itertools;

type Day = AocDay13;
#[cfg(test)]
static PUZZLE_INDEX: usize = 13;

pub struct AocDay13 {
    pairs: Vec<PacketPair>,
}

#[derive(Debug)]
struct PacketPair(PacketValue, PacketValue);

#[derive(Debug, Clone)]
enum PacketValue {
    List(Vec<PacketValue>),
    Integer(u64),
}

impl PacketValue {
    fn split_list_elements(input: &str) -> Vec<&str> {
        let mut els = vec![];
        let mut level = 0;
        let mut start = 0;

        for (i, c) in input.chars().enumerate() {
            match c {
                '[' => {
                    if level == 0 {
                        start = i
                    };
                    level += 1;
                }
                ']' => level -= 1,
                ',' if level == 0 => {
                    els.push(&input[start..i]);
                    start = i + 1;
                }
                _ => (),
            };
        }

        els.push(&input[start..]);

        els
    }
}

impl PartialEq for PacketValue {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl Eq for PacketValue {}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use PacketValue::*;

        match (self, other) {
            (Integer(s), Integer(o)) => s.cmp(o),
            (List(s), List(o)) => {
                for i in 0..max(s.len(), o.len()) {
                    if let Some(el_s) = s.get(i) {
                        if let Some(el_o) = o.get(i) {
                            let cmp = el_s.cmp(el_o);
                            if cmp != std::cmp::Ordering::Equal {
                                return cmp;
                            }
                        } else {
                            return std::cmp::Ordering::Greater;
                        }
                    } else {
                        return std::cmp::Ordering::Less;
                    }
                }

                std::cmp::Ordering::Equal
            }
            (Integer(_), List(_)) => List(vec![(*self).clone()]).cmp(other),
            (List(_), Integer(_)) => self.cmp(&List(vec![(*other).clone()])),
        }
    }
}

impl FromStr for PacketValue {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            Ok(PacketValue::List(if s.is_empty() {
                vec![]
            } else {
                // s.split(',') doesn't work for nested lists :(
                Self::split_list_elements(s)
                    .iter()
                    .map(|x| x.parse())
                    .collect::<Result<_, _>>()?
            }))
        } else {
            Ok(PacketValue::Integer(s.parse()?))
        }
    }
}

impl AocDay for Day {
    type R1 = usize;
    type R2 = usize;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut pairs = vec![];

        for mut chunk in &lines.chunks(3) {
            pairs.push(PacketPair(
                chunk.next().unwrap().parse().unwrap(),
                chunk.next().unwrap().parse().unwrap(),
            ));
        }

        Self { pairs }
    }

    fn part1(&self) -> Self::R1 {
        self.pairs
            .iter()
            .map(|p| p.0 < p.1)
            .enumerate()
            .filter(|(_, v)| *v)
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part2(&self) -> Self::R2 {
        let divider_packets = [2, 6]
            .iter()
            .map(|x| PacketValue::List(vec![PacketValue::List(vec![PacketValue::Integer(*x)])]));

        let mut packets: Vec<PacketValue> = self
            .pairs
            .iter()
            .flat_map(|p| vec![p.0.clone(), p.1.clone()])
            .collect();
        packets.extend(divider_packets.clone());
        packets.sort();

        divider_packets
            .map(|x| packets.iter().find_position(|p| **p == x))
            .map(|x| x.unwrap())
            .map(|(i, _)| i + 1)
            .product()
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
        assert_eq!(140, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
