use std::fmt::Debug;

use crate::AocDay;

type Day = AocDay08;
#[cfg(test)]
static PUZZLE_INDEX: usize = 8;

pub struct AocDay08 {
    matrix: Vec<Vec<Tree>>,
}

struct Tree {
    height: u64,
    visible: bool,
    scenic_score: ScenicScore,
}

#[derive(Debug)]
struct ScenicScore {
    looking_up: Option<usize>,
    looking_down: Option<usize>,
    looking_left: Option<usize>,
    looking_right: Option<usize>,
}

impl ScenicScore {
    fn get_directional_score_from_so_far(so_far: &[u64], cur_key: usize, cur_height: u64) -> usize {
        so_far
            .iter()
            .enumerate()
            .find(|(_, h)| **h >= cur_height)
            .map(|(x, _)| x + 1)
            .unwrap_or(cur_key)
    }

    fn get_overall_score(&self) -> usize {
        self.looking_up.unwrap()
            * self.looking_down.unwrap()
            * self.looking_left.unwrap()
            * self.looking_right.unwrap()
    }
}

impl Day {
    fn traverse_from_right(mut row: Vec<Tree>) -> Vec<Tree> {
        let mut max_row_from_right: Option<(usize, u64)> = None;
        let mut so_far_from_right = vec![];

        for (j, tree) in row.iter_mut().rev().enumerate() {
            if max_row_from_right.is_none() || tree.height > max_row_from_right.unwrap().1 {
                tree.visible = true;
                max_row_from_right = Some((j, tree.height));
            }
            tree.scenic_score.looking_right = Some(ScenicScore::get_directional_score_from_so_far(
                &so_far_from_right,
                j,
                tree.height,
            ));
            so_far_from_right.insert(0, tree.height);
        }

        row
    }

    fn traverse_from_bottom(mut self) -> Self {
        for j in 0..self.matrix[0].len() {
            let mut max_col_from_bottom: Option<(usize, u64)> = None;
            let mut so_far_from_bottom = vec![];
            let len = self.matrix.len();

            for i in (0..len).rev() {
                let tree = &mut self.matrix[i][j];
                if max_col_from_bottom.is_none() || tree.height > max_col_from_bottom.unwrap().1 {
                    tree.visible = true;
                    max_col_from_bottom = Some((len - 1 - i, tree.height));
                }
                tree.scenic_score.looking_down =
                    Some(ScenicScore::get_directional_score_from_so_far(
                        &so_far_from_bottom,
                        len - 1 - i,
                        tree.height,
                    ));
                so_far_from_bottom.insert(0, tree.height);
            }
        }

        self
    }
}

impl Debug for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.matrix
                .iter()
                .map(|row| {
                    (*row)
                        .iter()
                        .map(|tree| if tree.visible { '!' } else { '.' })
                        .collect::<String>()
                })
                .fold(String::new(), |acc, x| acc + &x + "\n")
        )
    }
}

impl AocDay for Day {
    type R1 = usize;
    type R2 = usize;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut matrix = vec![];
        let mut max_col_from_top = vec![];
        let mut so_far_from_top = vec![]; // i hate this, this is just storing matrix twice

        for (i, line) in lines.enumerate() {
            let mut row = vec![];
            let mut max_row_from_left: Option<(usize, u64)> = None;
            let mut so_far_from_left = vec![];

            for (j, char) in line.chars().enumerate() {
                let height: u64 = char.to_digit(10).unwrap().into();
                let edge = i == 0 || j == 0;
                let mut definitely_visible = edge;

                if max_row_from_left.is_none() || height > max_row_from_left.unwrap().1 {
                    definitely_visible = true;
                    max_row_from_left = Some((j, height));
                }

                if max_col_from_top.len() < j + 1 {
                    definitely_visible = true;
                    max_col_from_top.push((i, height));
                    so_far_from_top.push(vec![]);
                } else if height > max_col_from_top[j].1 {
                    definitely_visible = true;
                    max_col_from_top[j] = (i, height);
                }

                row.push(Tree {
                    height,
                    visible: definitely_visible,
                    scenic_score: ScenicScore {
                        looking_up: Some(ScenicScore::get_directional_score_from_so_far(
                            &so_far_from_top[j],
                            i,
                            height,
                        )),
                        looking_down: None,
                        looking_left: Some(ScenicScore::get_directional_score_from_so_far(
                            &so_far_from_left,
                            j,
                            height,
                        )),
                        looking_right: None,
                    },
                });
                so_far_from_left.insert(0, height);
                so_far_from_top[j].insert(0, height);
            }

            matrix.push(Self::traverse_from_right(row));
        }

        Self::traverse_from_bottom(Self { matrix })
    }

    fn part1(&self) -> Self::R1 {
        self.matrix
            .iter()
            .map(|row| (*row).iter().filter(|tree| tree.visible).count())
            .sum()
    }

    fn part2(&self) -> Self::R2 {
        self.matrix
            .iter()
            .map(|row| {
                (*row)
                    .iter()
                    .map(|tree| tree.scenic_score.get_overall_score())
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(21, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(8, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
