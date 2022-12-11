use std::str::FromStr;

use crate::AocDay;

type Day = AocDay11;
#[cfg(test)]
static PUZZLE_INDEX: usize = 11;

#[derive(Clone)]
pub struct AocDay11 {
    monkeys: Vec<Monkey>,
    prod_divisible_by: u64,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: MonkeyOperation,
    test: MonkeyTest,
    inspected_items: u64,
}

impl Monkey {
    fn read_one(lines: &mut impl Iterator<Item = String>) -> Self {
        let items = lines
            .nth(1)
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let operation = read_strip_prefix(lines, "  Operation: ");
        let test = MonkeyTest::read_one(lines);

        Self {
            items,
            operation,
            test,
            inspected_items: 0,
        }
    }
}

#[derive(Clone)]
enum MonkeyOperation {
    Sum(MonkeyOperationArgument),
    Multiplication(MonkeyOperationArgument),
}
impl MonkeyOperation {
    fn exec(&self, old: u64) -> u64 {
        match self {
            Self::Sum(arg) => old + arg.get_value(old),
            Self::Multiplication(arg) => old * arg.get_value(old),
        }
    }
}

impl FromStr for MonkeyOperation {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("new = old ") {
            let mut split = s.split_whitespace().rev();
            let arg = split.next().unwrap().parse().unwrap();

            return match split.next().unwrap() {
                "+" => Ok(MonkeyOperation::Sum(arg)),
                "*" => Ok(MonkeyOperation::Multiplication(arg)),
                _ => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Unknown monkey operation",
                )),
            };
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Failed to read MonkeyOperation",
        ))
    }
}

#[derive(Clone)]
enum MonkeyOperationArgument {
    Old,
    Literal(u64),
}

impl MonkeyOperationArgument {
    fn get_value(&self, old: u64) -> u64 {
        match self {
            Self::Old => old,
            Self::Literal(n) => *n,
        }
    }
}

impl FromStr for MonkeyOperationArgument {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(MonkeyOperationArgument::Old)
        } else if let Ok(n) = s.parse() {
            Ok(MonkeyOperationArgument::Literal(n))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to read MonkeyOperationArgument",
            ))
        }
    }
}

#[derive(Clone)]
struct MonkeyTest {
    // ideally the "divisible by" condition wouldn't be the only option, but it's overkill to use
    // an enum here since I know it'll always be this
    check_divisible_by: u64,
    // ideally I'd like this to be &Monkey, but I don't want to deal with lifetime specifiers...
    if_true_throw_to: usize,
    if_false_throw_to: usize,
}

impl MonkeyTest {
    fn read_one(lines: &mut impl Iterator<Item = String>) -> Self {
        let check_divisible_by = read_strip_prefix(lines, "  Test: divisible by ");
        let if_true_throw_to = read_strip_prefix(lines, "    If true: throw to monkey ");
        let if_false_throw_to = read_strip_prefix(lines, "    If false: throw to monkey ");

        Self {
            check_divisible_by,
            if_true_throw_to,
            if_false_throw_to,
        }
    }

    fn exec(&self, value: u64) -> usize {
        if value % self.check_divisible_by == 0 {
            self.if_true_throw_to
        } else {
            self.if_false_throw_to
        }
    }
}

fn read_strip_prefix<T>(lines: &mut impl Iterator<Item = String>, prefix: &str) -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    lines
        .next()
        .unwrap()
        .strip_prefix(prefix)
        .unwrap()
        .parse()
        .unwrap()
}

impl Day {
    fn simulate(&mut self, n_rounds: u64, relief_factor: u64) {
        for _ in 1..(n_rounds + 1) {
            for i in 0..self.monkeys.len() {
                let mut push_backlog = vec![];

                {
                    let monkey = self.monkeys.get_mut(i).unwrap();

                    for item in monkey.items.iter() {
                        let mut worry = monkey.operation.exec(*item);
                        worry /= relief_factor;
                        worry %= self.prod_divisible_by; // prevent unmanageable worry levels

                        let new_monkey = monkey.test.exec(worry);
                        push_backlog.push((new_monkey, worry));

                        monkey.inspected_items += 1;
                    }

                    monkey.items = vec![]; // assuming a monkey doesn't throw to itself
                }

                for (new_monkey, worry) in push_backlog {
                    self.monkeys.get_mut(new_monkey).unwrap().items.push(worry);
                }
            }
        }
    }

    fn monkey_business_level(&self) -> u64 {
        let mut most = None;
        let mut second_most = None;

        for n in self.monkeys.iter().map(|x| x.inspected_items) {
            if most.is_none() || n > most.unwrap() {
                second_most = most;
                most = Some(n);
            } else if second_most.is_none() || n > second_most.unwrap() {
                second_most = Some(n);
            }
        }

        most.unwrap() * second_most.unwrap()
    }
}

impl AocDay for Day {
    type R1 = u64;
    type R2 = u64;

    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let mut monkeys = vec![];
        let mut prod_divisible_by = 1;

        loop {
            let monkey = Monkey::read_one(&mut lines);

            prod_divisible_by *= monkey.test.check_divisible_by;

            monkeys.push(monkey);

            if lines.next().is_none() {
                break;
            }
        }

        Self {
            monkeys,
            prod_divisible_by,
        }
    }

    fn part1(&self) -> Self::R1 {
        let mut clone = (*self).clone();
        clone.simulate(20, 3);
        clone.monkey_business_level()
    }

    fn part2(&self) -> Self::R2 {
        let mut clone = self.clone();
        clone.simulate(10_000, 1);
        clone.monkey_business_level()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(10605, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(2713310158, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
