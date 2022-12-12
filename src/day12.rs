use std::fmt::{Display, Write};

use itertools::Itertools;

use crate::bfs::LinearBfs;
use crate::vmatrix::VMatrix;

#[derive(Debug)]
pub struct HeightMap {
    start: usize,
    end: usize,
    heights: VMatrix<usize>,
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.heights
            .to_display(|f, i, h| {
                if i == self.start {
                    f.write_char('S')
                } else if i == self.end {
                    f.write_char('E')
                } else {
                    f.write_char(char::from_u32(*h as u32 + 'a' as u32).unwrap())
                }
            })
            .fmt(f)
    }
}

pub fn generator(input: &str) -> HeightMap {
    let heights = input
        .chars()
        .map(|c| if c == '\n' { None } else { Some(c) })
        .collect::<VMatrix<char>>();
    let mut start = None;
    let mut end = None;
    let heights = heights.map(|i, c| {
        let c = match c {
            'S' => {
                start = Some(i);
                'a'
            }
            'E' => {
                end = Some(i);
                'z'
            }
            c => c,
        };
        c as usize - 'a' as usize
    });
    HeightMap {
        start: start.unwrap(),
        end: end.unwrap(),
        heights,
    }
}

pub fn part_1(input: &HeightMap) -> usize {
    let mut bfs = LinearBfs::new(input.heights.values.len());
    bfs.add_root(input.start, 0);
    while let Some(idx) = bfs.consider_next() {
        if idx == input.end {
            return *bfs.cost(idx).unwrap();
        }
        let this_height = input.heights.values[idx];
        for neighbor in input.heights.neighbors_no_diag(idx) {
            let neighbor_height = input.heights.values[neighbor];
            if neighbor_height <= this_height + 1 {
                bfs.add_edge(idx, neighbor, 1);
            }
        }
    }
    0
}

pub fn part_2(input: &HeightMap) -> usize {
    let mut bfs = LinearBfs::new(input.heights.values.len());
    bfs.add_root(input.end, 0);
    while let Some(idx) = bfs.consider_next() {
        let this_height = input.heights.values[idx];
        if this_height == 0 {
            return *bfs.cost(idx).unwrap();
        }
        for neighbor in input.heights.neighbors_no_diag(idx) {
            let neighbor_height = input.heights.values[neighbor];
            if this_height <= neighbor_height + 1 {
                bfs.add_edge(idx, neighbor, 1);
            }
        }
    }
    0
}
