use std::{cell::RefCell, rc::Rc};

use crate::AocDay;

type Day = AocDay07;
#[cfg(test)]
static PUZZLE_INDEX: usize = 7;

pub struct AocDay07 {
    root: DirWrapper,
    dirs: Vec<DirWrapper>,
}

const MAX_FS_SIZE: u64 = 70_000_000;
const REQUIRED_UNUSED: u64 = 30_000_000;

type DirWrapper = Rc<RefCell<Dir>>;

struct Dir {
    name: String,
    calculated_size: Option<u64>,
    children: Option<(Vec<DirWrapper>, Vec<File>)>,
    parent: Option<DirWrapper>,
}

impl Dir {
    fn find_child_dir(&self, name: &str) -> Option<DirWrapper> {
        self.children
            .as_ref()
            .expect("Parent not yet traversed!")
            .0
            .iter()
            .find(|x| x.borrow().name == name)
            .cloned()
    }

    fn calculate_size(&mut self) -> u64 {
        if self.calculated_size.is_none() {
            let children = self
                .children
                .as_mut()
                .expect("Traverse a directory before calculating its size!");

            self.calculated_size = {
                let dirs_size: u64 = children
                    .0
                    .iter()
                    .map(|x| x.as_ref().borrow_mut().calculate_size())
                    .sum();
                let files_size: u64 = children.1.iter().map(|x| x.size).sum();

                Some(dirs_size + files_size)
            }
        }

        self.calculated_size.unwrap()
    }
}

struct File {
    // name: String, // unused
    size: u64,
}

impl AocDay for Day {
    type R1 = u64;
    type R2 = u64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let root = Rc::new(RefCell::new(Dir {
            name: "/".to_owned(),
            calculated_size: None,
            children: None,
            parent: None,
        }));
        let mut dirs = vec![root.clone()];
        let mut cwd: Option<DirWrapper> = None;
        let mut listing = false;

        for line in lines {
            if let Some(line) = line.strip_prefix("$ cd ") {
                listing = false;

                cwd = Some(match line {
                    "/" => root.clone(),
                    ".." => cwd
                        .and_then(|x| x.borrow().parent.as_ref().cloned())
                        .unwrap_or_else(|| root.clone()),
                    dir_name => cwd
                        .and_then(|x| x.borrow().find_child_dir(dir_name))
                        .unwrap_or_else(|| root.clone()),
                })
            } else if line == "$ ls" && cwd.is_some() {
                let cwd = cwd.as_ref().unwrap().as_ref();
                if cwd.borrow().children.is_none() {
                    cwd.borrow_mut().children = Some((vec![], vec![]));
                    listing = true;
                }
            } else if listing && cwd.is_some() {
                let mut dir = cwd.as_ref().unwrap().as_ref().borrow_mut();
                let children = dir.children.as_mut().unwrap();
                if let Some(line) = line.strip_prefix("dir ") {
                    let new_dir = Rc::new(RefCell::new(Dir {
                        name: line.to_owned(),
                        calculated_size: None,
                        children: None,
                        parent: Some(cwd.as_ref().unwrap().clone()),
                    }));
                    dirs.push(new_dir.clone());
                    children.0.push(new_dir);
                } else {
                    let mut split = line.split(' ');

                    children.1.push(File {
                        size: split.next().unwrap().parse().unwrap(),
                        // name: split.next().unwrap().to_owned(),
                    })
                }
            }
        }

        Self { root, dirs }
    }

    fn part1(&self) -> Self::R1 {
        self.dirs
            .iter()
            .map(|x| x.as_ref().borrow_mut().calculate_size())
            .filter(|x| *x <= 100_000)
            .sum()
    }

    fn part2(&self) -> Self::R2 {
        let used_up_space = self.root.borrow_mut().calculate_size();
        let unused_space = MAX_FS_SIZE - used_up_space;
        let required_to_delete = REQUIRED_UNUSED - unused_space;

        self.dirs
            .iter()
            .map(|x| x.as_ref().borrow_mut().calculate_size())
            .filter(|x| *x >= required_to_delete)
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(95437, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(24933642, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
