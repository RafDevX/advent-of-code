use std::str::FromStr;

use crate::AocDay;

type Day = AocDay10;
#[cfg(test)]
static PUZZLE_INDEX: usize = 10;

pub struct AocDay10 {
    x_register_timeline: Vec<i64>,
    crt_image: Vec<String>,
}

enum Instruction {
    NoOp,
    AddX(i64),
}

#[derive(Debug)]
struct InstructionParseError;
impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::NoOp);
        } else if let Some(v) = s.strip_prefix("addx ") {
            if let Ok(v) = v.parse() {
                return Ok(Instruction::AddX(v));
            }
        }

        Err(InstructionParseError)
    }
}

struct InstructionExecutionState<'a> {
    cycles_required: usize,
    action: Box<dyn Fn(&mut i64) + 'a>,
}

impl InstructionExecutionState<'_> {
    fn new(instruction: Instruction) -> Self {
        match &instruction {
            Instruction::NoOp => Self {
                cycles_required: 1,
                action: Box::new(|_| ()),
            },
            Instruction::AddX(v) => {
                let v = *v; // how to avoid this?
                            // "returns a value referencing data owned by the current function"

                Self {
                    cycles_required: 2,
                    action: Box::new(move |x_register| *x_register += v),
                }
            }
        }
    }

    // Returns whether execution has ended
    fn tick(&mut self, x_register: &mut i64) -> bool {
        self.cycles_required -= 1;

        if self.cycles_required == 0 {
            (self.action)(x_register);
            return true;
        }

        false
    }
}

impl AocDay10 {
    fn signal_strength(&self, cycle_number: usize) -> i64 {
        (cycle_number as i64)
            * self
                .x_register_timeline
                .get(cycle_number - 1)
                .expect("No such cycle recorded")
    }

    fn is_sprite_visible(x_register_value: i64, pixel: usize) -> bool {
        x_register_value.abs_diff(pixel as i64) <= 1
    }

    fn get_crt_pixel_char(x_register_value: i64, pixel: usize) -> char {
        if Self::is_sprite_visible(x_register_value, pixel) {
            '#'
        } else {
            '.'
        }
    }
}

impl AocDay for Day {
    type R1 = i64;
    type R2 = String;

    fn preprocessing(mut lines: impl Iterator<Item = String>) -> Self {
        let mut x_register_timeline = vec![1];
        let mut executing: Option<InstructionExecutionState> = None;
        let mut crt_pixel = 0;
        let mut crt_image = vec![String::new()];

        loop {
            crt_image.last_mut().unwrap().push(Day::get_crt_pixel_char(
                *x_register_timeline.last().unwrap(),
                crt_pixel,
            ));

            crt_pixel += 1;
            if crt_pixel == 40 {
                crt_pixel = 0;
                crt_image.push(String::new());
            }

            if executing.as_ref().is_none() {
                match lines.next() {
                    Some(line) => {
                        executing = Some(InstructionExecutionState::new(line.parse().unwrap()))
                    }
                    None => break,
                }
            }

            x_register_timeline.push(*x_register_timeline.last().unwrap());
            let done = executing
                .as_mut()
                .unwrap()
                .tick(x_register_timeline.last_mut().unwrap());

            if done {
                executing = None;
            }
        }

        Self {
            x_register_timeline,
            crt_image,
        }
    }

    fn part1(&self) -> Self::R1 {
        (20..221).step_by(40).map(|x| self.signal_strength(x)).sum()
    }

    fn part2(&self) -> Self::R2 {
        "\n".to_owned() + &self.crt_image[..6].join("\n")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(13140, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....",
            setup_example::<Day>(PUZZLE_INDEX).part2()
        );
    }
}
