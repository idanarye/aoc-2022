use std::fmt::{Display, Write};
use std::ops::{Index, Range};

use itertools::Itertools;

#[derive(Debug)]
pub struct BoardMap {
    rows: Vec<(Range<usize>, Vec<bool>)>,
    col_ranges: Vec<Range<usize>>,
}

#[derive(Debug)]
pub enum Instruction {
    Walk(usize),
    Right,
    Left,
}

pub fn generator(input: &str) -> (BoardMap, Vec<Instruction>) {
    let mut it = input.lines();
    let rows = it
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|line| {
            let mut it = line.chars().peekable();
            let offset = it.peeking_take_while(|c| *c == ' ').count();
            (
                offset..line.len(),
                it.map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!(),
                })
                .collect_vec(),
            )
        })
        .collect_vec();
    #[allow(clippy::reversed_empty_ranges)]
    let mut col_ranges =
        vec![usize::MAX..0; rows.iter().map(|(range, _)| range.end).max().unwrap()];
    for (r, (row_range, _)) in rows.iter().enumerate() {
        for c in row_range.clone() {
            let col_range = &mut col_ranges[c];
            col_range.start = col_range.start.min(r);
            col_range.end = col_range.end.max(r + 1);
        }
    }
    let board_map = BoardMap { rows, col_ranges };
    let instructions_text = it.next().unwrap();
    assert!(it.next().is_none());
    let pattern = regex::Regex::new(r#"\d+|[RL]"#).unwrap();
    let instructions = pattern
        .find_iter(instructions_text)
        .map(|m| match m.as_str() {
            "R" => Instruction::Right,
            "L" => Instruction::Left,
            num => Instruction::Walk(num.parse().unwrap()),
        })
        .collect_vec();
    (board_map, instructions)
}

impl Index<[usize; 2]> for BoardMap {
    type Output = Option<bool>;

    fn index(&self, [r, c]: [usize; 2]) -> &Self::Output {
        if self.rows.len() <= r {
            return &None;
        }
        let (row_range, row) = &self.rows[r];
        if row_range.contains(&c) {
            if row[c - row_range.start] {
                &Some(true)
            } else {
                &Some(false)
            }
        } else {
            &None
        }
    }
}

impl BoardMap {
    fn calc_step(&self, [r, c]: [usize; 2], direction: Direction) -> Option<[usize; 2]> {
        match direction {
            Direction::Right => {
                let (range, row) = &self.rows[r];
                let new_c = if c + 1 < range.end {
                    c + 1
                } else {
                    range.start
                };
                if row[new_c - range.start] {
                    None
                } else {
                    Some([r, new_c])
                }
            }
            Direction::Down => {
                let range = &self.col_ranges[c];
                let new_r = if r + 1 < range.end {
                    r + 1
                } else {
                    range.start
                };
                let new_pos = [new_r, c];
                if self[new_pos].unwrap() {
                    None
                } else {
                    Some(new_pos)
                }
            }
            Direction::Left => {
                let (range, row) = &self.rows[r];
                let new_c = if range.start < c {
                    c - 1
                } else {
                    range.end - 1
                };
                if row[new_c - range.start] {
                    None
                } else {
                    Some([r, new_c])
                }
            }
            Direction::Up => {
                let range = &self.col_ranges[c];
                let new_r = if range.start < r {
                    r - 1
                } else {
                    range.end - 1
                };
                let new_pos = [new_r, c];
                if self[new_pos].unwrap() {
                    None
                } else {
                    Some(new_pos)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Direction::Right => '>',
            Direction::Down => 'V',
            Direction::Left => '<',
            Direction::Up => '^',
        })
    }
}

impl Direction {
    fn idx(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

struct State<'a> {
    pos: [usize; 2],
    direction: Direction,
    map: &'a BoardMap,
}

impl Display for State<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.map.rows.len() {
            let (range, row) = &self.map.rows[r];
            for _ in 0..range.start {
                f.write_char(' ')?;
            }
            for (i, w) in row.iter().enumerate() {
                let c = range.start + i;
                if self.pos == [r, c] {
                    write!(f, "{}", self.direction)?;
                } else if *w {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
            f.write_char('\n')?
        }
        Ok(())
    }
}

impl<'a> State<'a> {
    fn new(map: &'a BoardMap) -> Self {
        let (range, row) = &map.rows[0];
        let first_pos_col = range.start + row.iter().position(|w| !w).unwrap();
        Self {
            pos: [0, first_pos_col],
            direction: Direction::Right,
            map,
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Walk(num_steps) => {
                for _ in 0..*num_steps {
                    if let Some(new_pos) = self.map.calc_step(self.pos, self.direction) {
                        self.pos = new_pos;
                    } else {
                        return;
                    }
                }
            }
            Instruction::Right => {
                self.direction = self.direction.turn_right();
            }
            Instruction::Left => {
                self.direction = self.direction.turn_left();
            }
        }
    }

    fn calc_password(&self) -> usize {
        1000 * (self.pos[0] + 1) + 4 * (self.pos[1] + 1) + self.direction.idx()
    }
}

pub fn part_1((board_map, instructions): &(BoardMap, Vec<Instruction>)) -> usize {
    let mut state = State::new(board_map);
    for instruction in instructions.iter() {
        state.apply_instruction(instruction);
    }
    state.calc_password()
}

pub fn part_2(input: &(BoardMap, Vec<Instruction>)) -> usize {
    let _ = input;
    0
}
