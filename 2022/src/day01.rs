use crate::AocDay;

pub struct AocDay01 {
    calories_sum: Vec<i64>,
}

impl AocDay for AocDay01 {
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

        AocDay01 { calories_sum }
    }

    fn part1(&self) -> i64 {
        self.calories_sum.iter().max().unwrap().to_owned()
    }

    fn part2(&self) -> i64 {
        let mut vec: Vec<i64> = self.calories_sum.to_owned();
        vec.sort();
        vec.reverse();
        (&vec[0..=2]).iter().sum()
    }
}
