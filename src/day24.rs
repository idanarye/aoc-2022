use std::fmt::{Display, Write};
use std::iter::successors;
use std::ops::Index;

use enumflags2::{bitflags, make_bitflags, BitFlags};
use itertools::Itertools;

use crate::bfs::HashMapBfs;

type Coord = [isize; 2];

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wind {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
    Wall = 0b10000,
}

type WindFlags = BitFlags<Wind>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindState {
    width: usize,
    height: usize,
    cells: Vec<WindFlags>,
}

pub fn generator(input: &str) -> WindState {
    let mut it = input.lines();
    let first_line = it.next().unwrap();
    for (i, c) in first_line.chars().enumerate() {
        if i == 1 {
            assert!(c == '.');
        } else {
            assert!(c == '#');
        }
    }
    assert!(&first_line[..2] == "#.");
    assert!(&first_line.starts_with("#."));
    assert!(&first_line.chars().skip(2).all(|c| c == '#'));
    let height = input.lines().count() - 2;
    let width = first_line.len() - 2;
    let cells = it
        .by_ref()
        .take(height)
        .flat_map(|line| {
            assert!(line.len() == width + 2);
            assert!(line.starts_with('#'));
            assert!(line.ends_with('#'));
            line.chars().skip(1).take(width).map(|c| match c {
                '.' => WindFlags::EMPTY,
                '^' => WindFlags::from_flag(Wind::Up),
                'v' => WindFlags::from_flag(Wind::Down),
                '<' => WindFlags::from_flag(Wind::Left),
                '>' => WindFlags::from_flag(Wind::Right),
                _ => panic!(),
            })
        })
        .collect();
    for (i, c) in it.next().unwrap().chars().enumerate() {
        if i == width {
            assert!(c == '.');
        } else {
            assert!(c == '#');
        }
    }
    WindState {
        width,
        height,
        cells,
    }
}

impl Display for Wind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Wind::Up => '^',
            Wind::Down => 'v',
            Wind::Left => '<',
            Wind::Right => '>',
            Wind::Wall => '#',
        })
    }
}

// struct State {
// pos: Coord,
// wind: WindState,
// }
//
// const ONLY_WALL: WindFlags = WindFlags::from_flag(Wind::Wall);
const ONLY_WALL: WindFlags = make_bitflags!(Wind::{Wall});
impl Index<Coord> for WindState {
    type Output = WindFlags;

    fn index(&self, coord: Coord) -> &Self::Output {
        if let Some(idx) = self.coord_to_idx(coord) {
            &self.cells[idx]
        } else if coord == [0, -1] || coord == [self.width as isize - 1, self.height as isize] {
            &WindFlags::EMPTY
        } else {
            &ONLY_WALL
        }
    }
}

impl Display for WindState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_with_elf_at(f, [0, -2])
    }
}

impl WindState {
    fn period(&self) -> usize {
        num::integer::lcm(self.width, self.height)
    }

    fn coord_to_idx(&self, [x, y]: Coord) -> Option<usize> {
        if x < 0 || self.width <= x as usize || y < 0 || self.height <= y as usize {
            None
        } else {
            Some(x as usize + y as usize * self.width)
        }
    }

    fn print_with_elf_at(&self, f: &mut std::fmt::Formatter<'_>, pos: Coord) -> std::fmt::Result {
        for y in -1..(self.height as isize + 1) {
            writeln!(f)?;
            for x in -1..(self.width as isize + 1) {
                if [x, y] == pos {
                    f.write_char('E')?;
                    continue;
                }
                let winds = self[[x, y]];
                if let Some(wind) = winds.exactly_one() {
                    write!(f, "{}", wind)?;
                } else {
                    let num_winds = winds.len();
                    if num_winds == 0 {
                        f.write_char('.')?;
                    } else {
                        write!(f, "{}", num_winds)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn iter_coords(&self) -> impl Iterator<Item = Coord> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|(x, y)| [x as isize, y as isize])
    }

    fn advance(&self) -> Self {
        let mut new_cells = vec![WindFlags::EMPTY; self.cells.len()];
        for [x, y] in self.iter_coords() {
            for wind in self[[x, y]].iter() {
                let move_to = match wind {
                    Wind::Up => {
                        if 0 < y {
                            [x, y - 1]
                        } else {
                            [x, self.height as isize - 1]
                        }
                    }
                    Wind::Down => {
                        if y < self.height as isize - 1 {
                            [x, y + 1]
                        } else {
                            [x, 0]
                        }
                    }
                    Wind::Left => {
                        if 0 < x {
                            [x - 1, y]
                        } else {
                            [self.width as isize - 1, y]
                        }
                    }
                    Wind::Right => {
                        if x < self.width as isize - 1 {
                            [x + 1, y]
                        } else {
                            [0, y]
                        }
                    }
                    Wind::Wall => panic!("Wall inside the valley"),
                };
                new_cells[self.coord_to_idx(move_to).unwrap()] |= wind;
            }
        }
        Self {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }
    }

    #[allow(unused)]
    fn with_elf_at(&self, pos: Coord) -> WindStateWithElf {
        WindStateWithElf(self, pos)
    }

    fn start_position(&self) -> Coord {
        [0, -1]
    }

    fn end_position(&self) -> Coord {
        [self.width as isize - 1, self.height as isize]
    }
}

struct WindStateWithElf<'a>(&'a WindState, Coord);

impl Display for WindStateWithElf<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.print_with_elf_at(f, self.1)
    }
}

fn possible_movements([x, y]: Coord) -> impl Iterator<Item = Coord> {
    [[-1, 0], [1, 0], [0, -1], [0, 1], [0, 0]]
        .into_iter()
        .map(move |[dx, dy]| [x + dx, y + dy])
}

fn calc_trip(
    all_wind_states: &[WindState],
    start_state_idx: usize,
    start: Coord,
    end: Coord,
) -> usize {
    let mut bfs = HashMapBfs::<(Coord, usize), usize>::new();
    bfs.add_root((start, start_state_idx), 0);
    while let Some((pos, wind_idx)) = bfs.consider_next() {
        if pos == end {
            return *bfs.cost(&(pos, wind_idx)).unwrap();
        }
        let next_wind_idx = (wind_idx + 1) % all_wind_states.len();
        let next_wind_state = &all_wind_states[next_wind_idx];
        for next_pos in possible_movements(pos) {
            if next_wind_state[next_pos].is_empty() {
                bfs.add_edge((pos, wind_idx), (next_pos, next_wind_idx), 1);
            }
        }
    }
    0
}

pub fn part_1(input: &WindState) -> usize {
    let all_wind_states = successors(Some(input.clone()), |wind_state| Some(wind_state.advance()))
        .take(input.period())
        .collect_vec();
    calc_trip(
        &all_wind_states,
        0,
        input.start_position(),
        input.end_position(),
    )
}

pub fn part_2(input: &WindState) -> usize {
    let all_wind_states = successors(Some(input.clone()), |wind_state| Some(wind_state.advance()))
        .take(input.period())
        .collect_vec();
    let mut total = 0;
    for (from, to) in [
        (input.start_position(), input.end_position()),
        (input.end_position(), input.start_position()),
        (input.start_position(), input.end_position()),
    ] {
        total += calc_trip(&all_wind_states, total, from, to);
    }
    total
}
