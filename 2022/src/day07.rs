use std::{cell::RefCell, rc::Rc};

use crate::AocDay;

pub struct AocDay07 {
    root: Rc<RefCell<Dir>>,
    dirs: Vec<Rc<RefCell<Dir>>>,
}

type Day = AocDay07;
#[cfg(test)]
static PUZZLE_INDEX: usize = 7;

const MAX_FS_SIZE: u64 = 70_000_000;
const REQUIRED_UNUSED: u64 = 30_000_000;

struct Dir {
    name: String,
    calculated_size: Option<u64>,
    children: Option<(Vec<Rc<RefCell<Dir>>>, Vec<File>)>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn find_child_dir(&self, name: &str) -> Option<Rc<RefCell<Dir>>> {
        self.children
            .as_ref()
            .expect("Parent not yet traversed!")
            .0
            .iter()
            .find(|x| x.borrow().name == name)
            .map(|x| x.clone())
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
    type R = u64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let root = Rc::new(RefCell::new(Dir {
            name: "/".to_owned(),
            calculated_size: None,
            children: None,
            parent: None,
        }));
        let mut dirs = vec![root.clone()];
        let mut cwd: Option<Rc<RefCell<Dir>>> = None;
        let mut listing = false;

        for line in lines {
            // how to do a match with line[..4] here?
            // "expected str, found &str" in arms

            if line.starts_with("$ cd ") {
                listing = false;

                cwd = Some(match &line[5..] {
                    "/" => root.clone(),
                    ".." => cwd
                        .map(|x| x.borrow().parent.as_ref().map(|y| y.clone()))
                        .flatten()
                        .map(|x| x.clone())
                        .unwrap_or(root.clone()),
                    dir_name => cwd
                        .map(|x| x.borrow().find_child_dir(dir_name))
                        .flatten()
                        .unwrap_or(root.clone()),
                })
            } else if line == "$ ls" && cwd.is_some() {
                let cwd = (&cwd).as_ref().unwrap().as_ref();
                if cwd.borrow().children.is_none() {
                    cwd.borrow_mut().children = Some((vec![], vec![]));
                    listing = true;
                }
            } else if listing && cwd.is_some() {
                let mut dir = (&cwd).as_ref().unwrap().as_ref().borrow_mut();
                let children = dir.children.as_mut().unwrap();
                if line.starts_with("dir ") {
                    let new_dir = Rc::new(RefCell::new(Dir {
                        name: line[4..].to_owned(),
                        calculated_size: None,
                        children: None,
                        parent: Some((&cwd).as_ref().unwrap().clone()),
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

    fn part1(&self) -> Self::R {
        self.dirs
            .iter()
            .map(|x| x.as_ref().borrow_mut().calculate_size())
            .filter(|x| *x <= 100_000)
            .sum()
    }

    fn part2(&self) -> Self::R {
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
