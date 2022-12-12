use crate::AocDay;
use pathfinding::directed::astar::astar;

type Day = AocDay12;
#[cfg(test)]
static PUZZLE_INDEX: usize = 12;

pub struct AocDay12 {
    matrix: Vec<Vec<Height>>,
    start: Pos,
    end: Pos,
    lowest_elevation: Vec<Pos>,
}
impl AocDay12 {
    fn get_successors(&self, p: &Pos) -> Vec<(Pos, u64)> {
        let &Pos(x, y) = p;
        ([(-1, 0), (1, 0), (0, -1), (0, 1)] as [(i64, i64); 4])
            .iter()
            .filter(|delta| delta.0 + (x as i64) >= 0 && delta.1 + (y as i64) >= 0)
            .filter(|delta| {
                ((delta.0 + (x as i64)) as usize) < self.matrix.len()
                    && ((delta.1 + (y as i64)) as usize) < self.matrix[0].len()
            })
            .map(|delta| {
                Pos(
                    (delta.0 + (x as i64)) as usize,
                    (delta.1 + (y as i64)) as usize,
                )
            })
            .filter(|Pos(adj_x, adj_y)| self.matrix[x][y].1 + 1 >= self.matrix[*adj_x][*adj_y].1)
            .map(|x| (x, 1))
            .collect()
    }

    fn calc_heuristic(&self, p: &Pos) -> u64 {
        let &Pos(px, py) = p;
        let Pos(gx, gy) = self.end;

        ((px.abs_diff(gx) + py.abs_diff(gy)) as u64)
            + self.matrix[px][py].1.abs_diff(self.matrix[gx][gy].1)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Pos(usize, usize);

#[derive(PartialEq, Eq, Hash, Clone)]
struct Height(char, u64);

impl From<char> for Height {
    fn from(c: char) -> Self {
        let h = match c {
            'S' => 'a',
            'E' => 'z',
            h if h.is_ascii_lowercase() => h,
            _ => panic!("Unknown height"),
        };

        Self(c, (h as u64) - ('a' as u64))
    }
}

impl AocDay for Day {
    type R1 = u64;
    type R2 = u64;

    fn preprocessing(lines: impl Iterator<Item = String>) -> Self {
        let mut matrix = vec![];
        let mut start = None;
        let mut end = None;
        let mut lowest_elevation = vec![];

        for (i, line) in lines.enumerate() {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => start = Some(Pos(i, j)),
                    'E' => end = Some(Pos(i, j)),
                    'a' => lowest_elevation.push(Pos(i, j)),
                    _ => (),
                }
                row.push(c.into());
            }
            matrix.push(row);
        }

        let start = start.expect("No start position found");
        let end = end.expect("No end position found");
        lowest_elevation.push(start.clone());
        Self {
            matrix,
            start,
            end,
            lowest_elevation,
        }
    }

    fn part1(&self) -> Self::R1 {
        let result = astar(
            &self.start,
            |p| self.get_successors(p),
            |p| self.calc_heuristic(p),
            |p| *p == self.end,
        );

        result.expect("No path found").1
    }

    fn part2(&self) -> Self::R2 {
        self.lowest_elevation
            .iter()
            .map(|start| {
                astar(
                    start,
                    |p| self.get_successors(p),
                    |p| self.calc_heuristic(p),
                    |p| *p == self.end,
                )
            })
            .filter(|result| result.is_some())
            .map(|result| result.unwrap().1)
            .min()
            .expect("No min found")
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::setup_example;

    #[test]
    fn part1_example() {
        assert_eq!(31, setup_example::<Day>(PUZZLE_INDEX).part1());
    }

    #[test]
    fn part2_example() {
        assert_eq!(29, setup_example::<Day>(PUZZLE_INDEX).part2());
    }
}
