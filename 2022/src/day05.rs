use crate::AocDay;

type Day = AocDay05;
#[cfg(test)]
static PUZZLE_INDEX: usize = 5;

pub struct AocDay05 {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

impl From<String> for Move {
    fn from(line: String) -> Self {
        let mut from_split = (&line[5..]).split(" from ");
        let qty = from_split.next().unwrap().parse().unwrap();
        let mut to_split = from_split.next().unwrap().split(" to ");
        Self {
            qty,
            from: to_split.next().unwrap().parse().unwrap(),
            to: to_split.next().unwrap().parse().unwrap(),
        }
    }
}

impl AocDay for Day {
    type R = String;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut stacks = vec![];
        let mut moves = vec![];
        let mut moving = false;

        for line in lines {
            if line.is_empty() {
                moving = true;
                continue;
            } else if !moving {
                let chars: Vec<char> = line.chars().collect();
                if chars.get(1).map_or(false, |c| c.is_ascii_digit()) {
                    continue;
                }

                let line_crates = chars.chunks(4);

                for (i, chunk) in line_crates.enumerate() {
                    let item = chunk.get(1).unwrap().to_owned();
                    if item.is_ascii_alphabetic() {
                        while stacks.len() < i + 1 {
                            stacks.push(vec![]);
                        }

                        stacks.get_mut(i).unwrap().insert(0, item);
                    }
                }
            } else {
                moves.push(line.into());
            }
        }

        AocDay05 { stacks, moves }
    }

    fn part1(&self) -> Self::R {
        let mut stacks = self.stacks.clone();

        for mv in self.moves.iter() {
            for _ in 0..mv.qty {
                let item = stacks[mv.from - 1].pop().unwrap();
                stacks[mv.to - 1].push(item);
            }
        }

        stacks.iter().map(|x| x.last().unwrap()).collect()
    }

    fn part2(&self) -> Self::R {
        let mut stacks = self.stacks.clone();

        for mv in self.moves.iter() {
            let mut tmp = vec![];

            for _ in 0..mv.qty {
                tmp.push(stacks[mv.from - 1].pop().unwrap());
            }

            for _ in 0..mv.qty {
                stacks[mv.to - 1].push(tmp.pop().unwrap());
            }
        }

        stacks.iter().map(|x| x.last().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!("CMZ", setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!("MCD", setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
