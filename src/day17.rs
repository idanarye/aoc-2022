use std::fmt::{Display, Write};
use std::iter::{from_fn, repeat};
use std::ops::{Index, IndexMut};

use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
pub enum Jet {
    Left,
    Right,
}

pub fn generator(input: &str) -> Vec<Jet> {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Jet::Left),
            '>' => Some(Jet::Right),
            '\n' => None,
            _ => panic!("Bad {:?}", c),
        })
        .collect()
}

struct BrickPattern {
    pattern: &'static [&'static [bool]],
    contact_down: &'static [[usize; 2]],
    contact_left: &'static [[usize; 2]],
    contact_right: &'static [[usize; 2]],
}

const BRICK_PATTERNS: &[BrickPattern] = &[
    BrickPattern {
        pattern: &[&[true, true, true, true]],
        contact_down: &[[0, 0], [1, 0], [2, 0], [3, 0]],
        contact_left: &[[0, 0]],
        contact_right: &[[3, 0]],
    },
    BrickPattern {
        pattern: &[
            &[false, true, false],
            &[true, true, true],
            &[false, true, false],
        ],
        contact_down: &[[0, 1], [1, 0], [2, 1]],
        contact_left: &[[1, 0], [0, 1], [1, 2]],
        contact_right: &[[1, 0], [2, 1], [1, 2]],
    },
    // NOTE: They are in "reverse" order (first one is bottom)
    BrickPattern {
        pattern: &[
            &[true, true, true],
            &[false, false, true],
            &[false, false, true],
        ],
        contact_down: &[[0, 0], [1, 0], [2, 0]],
        contact_left: &[[0, 0], [2, 1], [2, 2]],
        contact_right: &[[2, 0], [2, 1], [2, 2]],
    },
    BrickPattern {
        pattern: &[&[true], &[true], &[true], &[true]],
        contact_down: &[[0, 0]],
        contact_left: &[[0, 0], [0, 1], [0, 2], [0, 3]],
        contact_right: &[[0, 0], [0, 1], [0, 2], [0, 3]],
    },
    BrickPattern {
        pattern: &[&[true, true], &[true, true]],
        contact_down: &[[0, 0], [1, 0]],
        contact_left: &[[0, 0], [0, 1]],
        contact_right: &[[1, 0], [1, 1]],
    },
];

struct Arena {
    cols: usize,
    fake_row: Vec<bool>,
    rocks: Vec<bool>,
}

impl Index<[usize; 2]> for Arena {
    type Output = bool;

    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        self.rocks.get(x + y * self.cols).unwrap_or(&false)
    }
}

impl IndexMut<[usize; 2]> for Arena {
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        self.rocks.get_mut(x + y * self.cols).unwrap()
    }
}

impl Arena {
    fn rows(&self) -> usize {
        self.rocks.len() / self.cols
    }

    fn row(&self, row: usize) -> &[bool] {
        let row_start = row * self.cols;
        if row_start < self.rocks.len() {
            &self.rocks[row_start..row_start + self.cols]
        } else {
            &self.fake_row
        }
    }

    fn top_rows(&self, num_rows: usize) -> &[bool] {
        let num_cells = num_rows * self.cols;
        if self.rocks.len() < num_cells {
            &self.rocks
        } else {
            &self.rocks[self.rocks.len() - num_cells..]
        }
    }

    fn has_collision(&self, [x, y]: [usize; 2], contacts: &[[usize; 2]]) -> bool {
        contacts.iter().any(|[c, r]| self[[x + c, y + r]])
    }
}

struct State {
    arena: Arena,
    current: Option<(&'static BrickPattern, [usize; 2])>,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (falling_at, falling_rows) = if let Some((pattern, [x, y])) = self.current {
            let falling_rows = pattern
                .pattern
                .iter()
                .map(|pattern_row| {
                    let mut falling_row = vec![false; self.arena.cols];
                    for (i, pattern_rock) in pattern_row.iter().enumerate() {
                        if *pattern_rock {
                            falling_row[i + x] = true;
                        }
                    }
                    falling_row
                })
                .collect_vec();
            (y..y + falling_rows.len(), falling_rows)
        } else {
            (0..0, Vec::new())
        };

        let rows = falling_at.end.max(self.arena.rocks.len() / self.arena.cols);
        for row in (0..rows).rev() {
            f.write_str("\n+")?;
            for (col, rock) in self.arena.row(row).iter().enumerate() {
                if falling_at.contains(&row) && falling_rows[row - falling_at.start][col] {
                    f.write_char('@')?;
                } else if *rock {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('+')?;
        }
        f.write_str("\n+")?;
        for _ in 0..self.arena.cols {
            f.write_char('-')?;
        }
        f.write_char('+')?;
        Ok(())
    }
}

impl State {
    fn new(cols: usize) -> Self {
        Self {
            arena: Arena {
                cols,
                fake_row: vec![false; cols],
                rocks: Vec::new(),
            },
            current: None,
        }
    }

    fn set_brick(&mut self, pattern: &'static BrickPattern, pos: [usize; 2]) {
        assert!(self.current.is_none());
        self.current = Some((pattern, pos));
    }

    fn freeze_brick(&mut self) {
        let (pattern, [x, y]) = self.current.take().unwrap();
        let rows_required = y + pattern.pattern.len();
        if self.arena.rows() < rows_required {
            self.arena
                .rocks
                .extend(repeat(false).take(self.arena.cols * (rows_required - self.arena.rows())));
        }
        for (r, brick_row) in pattern.pattern.iter().enumerate() {
            for (c, rock) in brick_row.iter().enumerate() {
                if *rock {
                    let cell = &mut self.arena[[x + c, y + r]];
                    assert!(!*cell);
                    *cell = true;
                }
            }
        }
    }

    fn push_brick(&mut self, jet: Jet) {
        let (pattern, [x, y]) = self.current.as_mut().unwrap();
        match jet {
            Jet::Left => {
                if *x == 0 || self.arena.has_collision([*x - 1, *y], pattern.contact_left) {
                    return;
                }
                *x -= 1;
            }
            Jet::Right => {
                if self.arena.cols <= *x + pattern.pattern[0].len()
                    || self
                        .arena
                        .has_collision([*x + 1, *y], pattern.contact_right)
                {
                    return;
                }
                *x += 1;
            }
        }
    }

    fn drop_brick(&mut self) -> bool {
        let (pattern, [x, y]) = self.current.as_mut().unwrap();
        if *y == 0 || self.arena.has_collision([*x, *y - 1], pattern.contact_down) {
            false
        } else {
            *y -= 1;
            true
        }
    }
}

pub fn solve_for(input: &[Jet], total_bricks: usize) -> usize {
    type Key = (usize, usize, Vec<bool>);

    #[derive(Debug, Clone)]
    struct Step {
        added_rows: usize,
        got_to_key: Key,
    }

    let mut state = State::new(7);
    let initial_key = (0, 0, state.arena.top_rows(0).to_owned());
    let mut jets = input.iter().enumerate().cycle().peekable();
    let mut steps = HashMap::<Key, Step>::new();
    let mut step_to_add = None;
    let cycle_starts_at = BRICK_PATTERNS
        .iter()
        .enumerate()
        .cycle()
        .take(1000000)
        .find_map(|(brick_num, brick_pattern)| {
            let first_jet_num = jets.peek().unwrap().0;
            let key = (brick_num, first_jet_num, state.arena.top_rows(1).to_owned());
            if let Some((prev_key, added_rows)) = step_to_add.take() {
                steps.insert(
                    prev_key,
                    Step {
                        added_rows,
                        got_to_key: key.clone(),
                    },
                );
            }
            if steps.contains_key(&key) {
                return Some(key);
            }

            state.set_brick(brick_pattern, [2, state.arena.rows() + 3]);
            for (_, jet) in jets.by_ref() {
                state.push_brick(*jet);
                let could_fall = state.drop_brick();
                if !could_fall {
                    break;
                }
            }
            let rows_before = state.arena.rows();
            state.freeze_brick();
            let added_rows = state.arena.rows() - rows_before;

            step_to_add = Some((key, added_rows));
            None
        })
        .unwrap();

    let run_steps_from = |mut key: Key| {
        let steps = &steps;
        from_fn(move || {
            let step = steps[&key].clone();
            key = step.got_to_key.clone();
            Some(step)
        })
    };

    let calc_upto_cycle = |key: Key| {
        let mut num_rows = 0;
        for (
            num_bricks,
            Step {
                added_rows,
                got_to_key,
            },
        ) in run_steps_from(key).enumerate()
        {
            num_rows += added_rows;
            if got_to_key == cycle_starts_at {
                return (num_bricks + 1, num_rows);
            }
        }
        panic!()
    };

    let (one_time_bricks, one_time_rows) = calc_upto_cycle(initial_key);
    let (per_cycle_bricks, per_cycle_rows) = calc_upto_cycle(cycle_starts_at.clone());

    let bricks_added_in_cycles = total_bricks - one_time_bricks;
    let num_cycles = bricks_added_in_cycles / per_cycle_bricks;

    let bricks_added_in_partial_cycle = bricks_added_in_cycles % per_cycle_bricks;
    let rows_added_in_partial_cycle: usize = run_steps_from(cycle_starts_at)
        .take(bricks_added_in_partial_cycle)
        .map(|step| step.added_rows)
        .sum();

    one_time_rows + num_cycles * per_cycle_rows + rows_added_in_partial_cycle
}

pub fn part_1(input: &[Jet]) -> usize {
    solve_for(input, 2022)
}

pub fn part_2(input: &[Jet]) -> usize {
    solve_for(input, 1_000_000_000_000)
}
