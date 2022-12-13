use std::fmt::{Display, Write};
use std::iter::from_fn;

use itertools::Itertools;

#[derive(Debug)]
pub struct Forest {
    cols: usize,
    rows: usize,
    heights: Vec<usize>,
}

impl Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, h) in self.heights.iter().enumerate() {
            if i % self.cols == 0 {
                f.write_char('\n')?;
            }
            write!(f, "{}", h)?;
        }
        Ok(())
    }
}

pub fn generator(input: &str) -> Forest {
    let mut cols = 0;
    let mut cur_col = 0;
    let heights = input
        .chars()
        .filter_map(|c| {
            if c == '\n' {
                cols = cur_col;
                cur_col = 0;
                None
            } else if ('0'..='9').contains(&c) {
                cur_col += 1;
                Some(c as usize - '0' as usize)
            } else {
                panic!("Bad char {:?}", c);
            }
        })
        .collect_vec();
    let rows = heights.len() / cols;
    Forest {
        cols,
        rows,
        heights,
    }
}

impl Forest {
    fn to_idx(&self, [x, y]: [isize; 2]) -> Option<usize> {
        if x < 0 || y < 0 || self.cols <= x as usize || self.rows <= y as usize {
            return None;
        }
        Some(x as usize + y as usize * self.cols)
    }

    fn walk_indices(&self, start: [isize; 2], dir: [isize; 2]) -> impl '_ + Iterator<Item = usize> {
        let mut pos = start;
        from_fn(move || {
            let idx = self.to_idx(pos)?;
            for (p, d) in pos.iter_mut().zip(dir) {
                *p += d;
            }
            Some(idx)
        })
    }

    fn scenic_score(&self, pos: [isize; 2]) -> usize {
        [[1, 0], [-1, 0], [0, 1], [0, -1]]
            .into_iter()
            .map(|dir| {
                let this_height = self.heights[self.to_idx(pos).unwrap()];
                let mut trees_seen = 0;
                for idx in self.walk_indices(pos, dir).skip(1) {
                    trees_seen += 1;
                    let height = self.heights[idx];
                    if this_height <= height {
                        break;
                    }
                }
                trees_seen
            })
            .product()
    }
}

pub fn part_1(forest: &Forest) -> usize {
    let mut visible = vec![false; forest.heights.len()];
    let mut make_pass = |start, dir| {
        let mut it = forest.walk_indices(start, dir);
        let idx = it.next().unwrap();
        visible[idx] = true;
        let mut visible_above = forest.heights[idx];
        for idx in it {
            let height = forest.heights[idx];
            if visible_above < height {
                visible_above = height;
                visible[idx] = true;
            }
        }
    };
    for i in 0..forest.rows {
        make_pass([0, i as isize], [1, 0]);
        make_pass([forest.cols as isize - 1, i as isize], [-1, 0]);
    }
    for i in 0..forest.cols {
        make_pass([i as isize, 0], [0, 1]);
        make_pass([i as isize, forest.rows as isize - 1], [0, -1]);
    }
    visible.iter().filter(|&v| *v).count()
}

pub fn part_2(forest: &Forest) -> usize {
    (0..forest.cols)
        .cartesian_product(0..forest.rows)
        .map(|(x, y)| forest.scenic_score([x as isize, y as isize]))
        .max()
        .unwrap()
}
