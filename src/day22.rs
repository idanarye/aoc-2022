use std::fmt::{Display, Write};
use std::ops::{Index, Range};

use itertools::Itertools;
use num::integer::Roots;

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
    fn all() -> [Direction; 4] {
        [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ]
    }

    fn idx(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn turn(&self, times_cw: isize) -> Direction {
        Direction::all()[(self.idx() as isize + times_cw).rem_euclid(4) as usize]
    }

    fn try_move_once(
        &self,
        [r, c]: [usize; 2],
        rows_range: Range<usize>,
        cols_range: Range<usize>,
    ) -> Option<[usize; 2]> {
        match self {
            Direction::Right => (c + 1 < cols_range.end).then_some([r, c + 1]),
            Direction::Down => (r + 1 < rows_range.end).then_some([r + 1, c]),
            #[allow(clippy::unnecessary_lazy_evaluations)]
            Direction::Left => (cols_range.start < c).then(|| [r, c - 1]),
            #[allow(clippy::unnecessary_lazy_evaluations)]
            Direction::Up => (rows_range.start < r).then(|| [r - 1, c]),
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

    fn apply_instruction(
        &mut self,
        instruction: &Instruction,
        mut wrap_dlg: impl FnMut([usize; 2], Direction) -> ([usize; 2], Direction),
    ) {
        match instruction {
            Instruction::Walk(num_steps) => {
                for _ in 0..*num_steps {
                    let cols_range = &self.map.rows[self.pos[0]].0;
                    let rows_range = &self.map.col_ranges[self.pos[1]];
                    let (new_pos, new_direction) = if let Some(new_pos) = self
                        .direction
                        .try_move_once(self.pos, rows_range.clone(), cols_range.clone())
                    {
                        (new_pos, self.direction)
                    } else {
                        wrap_dlg(self.pos, self.direction)
                    };
                    if self.map[new_pos].unwrap() {
                        return;
                    }
                    self.pos = new_pos;
                    self.direction = new_direction;
                }
            }
            Instruction::Right => {
                self.direction = self.direction.turn(1);
            }
            Instruction::Left => {
                self.direction = self.direction.turn(-1);
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
        state.apply_instruction(instruction, |[r, c], direction| match direction {
            Direction::Right => ([r, state.map.rows[r].0.start], direction),
            Direction::Down => ([state.map.col_ranges[c].start, c], direction),
            Direction::Left => ([r, state.map.rows[r].0.end - 1], direction),
            Direction::Up => ([state.map.col_ranges[c].end - 1, c], direction),
        });
    }
    state.calc_password()
}

const CUBE_SIDE_LINKS: [[usize; 4]; 6] = [
    //              >  V  <  ^
    /*0 - bottom*/ [1, 2, 3, 4],
    /*1 -  right*/ [5, 2, 0, 4],
    /*2 -   down*/ [1, 5, 3, 0],
    /*3 -   left*/ [0, 2, 5, 4],
    /*4 -     up*/ [1, 0, 3, 5],
    /*5 -    top*/ [1, 4, 3, 2],
];

pub fn part_2((board_map, instructions): &(BoardMap, Vec<Instruction>)) -> usize {
    let face_side = (board_map
        .rows
        .iter()
        .map(|(range, _)| range.len())
        .sum::<usize>()
        / 6)
    .sqrt();
    for (range, _) in board_map.rows.iter() {
        assert_eq!(range.start % face_side, 0);
        assert_eq!(range.end % face_side, 0);
    }
    for range in board_map.col_ranges.iter() {
        assert_eq!(range.start % face_side, 0);
        assert_eq!(range.end % face_side, 0);
    }

    let f_rows = board_map.rows.len() / face_side;
    let f_cols = board_map.col_ranges.len() / face_side;
    let mut faces_by_position: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; f_cols]; f_rows];
    let relevant_face_positions = (0..f_rows)
        .flat_map(move |r| (0..f_cols).map(move |c| [r, c]))
        .filter(|[r, c]| board_map[[r * face_side, c * face_side]].is_some())
        .collect_vec();
    {
        let [r, c] = relevant_face_positions[0];
        faces_by_position[r][c] = Some((0, 0));
    }
    'resolving: loop {
        for &[r, c] in relevant_face_positions.iter() {
            if faces_by_position[r][c].is_some() {
                continue;
            }
            #[allow(clippy::never_loop)]
            for direction in Direction::all() {
                let Some([nr, nc]) = direction.try_move_once([r, c], 0..f_rows, 0..f_cols) else { continue };
                let Some((n_face, n_orientation)) = faces_by_position[nr][nc] else { continue };
                let face_idx = CUBE_SIDE_LINKS[n_face][(direction.idx() + 2 + n_orientation) % 4];
                let face_orientation = (0..4)
                    .find(|orientation| {
                        CUBE_SIDE_LINKS[face_idx][(direction.idx() + orientation) % 4] == n_face
                    })
                    .unwrap();
                faces_by_position[r][c] = Some((face_idx, face_orientation));
                continue 'resolving;
            }
        }
        break;
    }
    let positions_by_faces = faces_by_position
        .iter()
        .enumerate()
        .flat_map(|(fr, row)| {
            row.iter().enumerate().filter_map(move |(fc, face_info)| {
                let (face_idx, face_orientation) = (*face_info)?;
                Some((face_idx, [fr, fc], face_orientation))
            })
        })
        .sorted()
        .map(|(_, pos, orientation)| (pos, orientation))
        .collect_vec();
    let mut state = State::new(board_map);
    for instruction in instructions.iter() {
        state.apply_instruction(instruction, |pos, direction| {
            let [fr, fc] = pos.map(|n| n / face_side);
            let (current_face_idx, current_face_orientation) = faces_by_position[fr][fc].unwrap();
            let new_face_idx =
                CUBE_SIDE_LINKS[current_face_idx][(direction.idx() + current_face_orientation) % 4];
            let ([nr, nc], n_orientation) = positions_by_faces[new_face_idx];
            let [or, oc] = pos.map(|n| n % face_side);

            let idx_in_cube_side_link = CUBE_SIDE_LINKS[new_face_idx]
                .iter()
                .position(|idx| *idx == current_face_idx)
                .unwrap();
            let new_direction =
                Direction::all()[(idx_in_cube_side_link + 4 - n_orientation + 2) % 4];

            let oriented_offset = match direction {
                Direction::Right => or,
                Direction::Down => face_side - 1 - oc,
                Direction::Left => face_side - 1 - or,
                Direction::Up => oc,
            };

            let [nor, noc] = match new_direction {
                Direction::Right => [oriented_offset, 0],
                Direction::Down => [0, face_side - 1 - oriented_offset],
                Direction::Left => [face_side - 1 - oriented_offset, face_side - 1],
                Direction::Up => [face_side - 1, oriented_offset],
            };

            let new_pos = [nr * face_side + nor, nc * face_side + noc];

            (new_pos, new_direction)
        });
    }
    state.calc_password()
}
