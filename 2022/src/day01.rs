use crate::AocDay;

pub struct AocDay01 {
    elves: Vec<Vec<i64>>,
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

        AocDay01 { elves }
    }

    fn part1(&self) -> i64 {
        self.elves.iter().map(|x| x.iter().sum()).max().unwrap()
    }

    fn part2(&self) -> i64 {
        let mut vec: Vec<i64> = self.elves.iter().map(|x| x.iter().sum()).collect();
        vec.sort();
        vec.reverse();
        (&vec[0..=2]).iter().sum()
    }
}
