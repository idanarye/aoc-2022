use std::fmt::{Display, Write};
use std::ops::RangeInclusive;

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Coord = [isize; 2];

pub fn generator(input: &str) -> Vec<Coord> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                '#' => Some([x as isize, y as isize]),
                _ => panic!(),
            })
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::N => "N",
            Direction::NE => "NE",
            Direction::E => "E",
            Direction::SE => "SE",
            Direction::S => "S",
            Direction::SW => "SW",
            Direction::W => "W",
            Direction::NW => "NW",
        })
    }
}

impl Direction {
    fn all() -> [Self; 8] {
        [
            Self::N,
            Self::NE,
            Self::E,
            Self::SE,
            Self::S,
            Self::SW,
            Self::W,
            Self::NW,
        ]
    }

    fn idx(&self) -> usize {
        match self {
            Direction::N => 0,
            Direction::NE => 1,
            Direction::E => 2,
            Direction::SE => 3,
            Direction::S => 4,
            Direction::SW => 5,
            Direction::W => 6,
            Direction::NW => 7,
        }
    }

    fn order_of_movement() -> [Self; 4] {
        [Self::N, Self::S, Self::W, Self::E]
    }

    fn to_try(start_from_direction: usize) -> impl Iterator<Item = Self> {
        (0..4).map(move |i| Direction::order_of_movement()[(i + start_from_direction) % 4])
    }

    fn vec(&self) -> Coord {
        match self {
            Direction::N => [0, -1],
            Direction::NE => [1, -1],
            Direction::E => [1, 0],
            Direction::SE => [1, 1],
            Direction::S => [0, 1],
            Direction::SW => [-1, 1],
            Direction::W => [-1, 0],
            Direction::NW => [-1, -1],
        }
    }

    pub(crate) fn add_to(&self, coord: Coord) -> Coord {
        let vec = self.vec();
        [vec[0] + coord[0], vec[1] + coord[1]]
    }
}

struct State {
    elves: HashSet<Coord>,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [x_limits, y_limits] = self.limits();
        for y in y_limits {
            writeln!(f)?;
            for x in x_limits.clone() {
                if self.elves.contains(&[x, y]) {
                    f.write_char('#')?;
                } else {
                    f.write_char('.')?;
                }
            }
        }
        Ok(())
    }
}

impl State {
    fn limits(&self) -> [RangeInclusive<isize>; 2] {
        let limits = [
            self.elves
                .iter()
                .map(|[x, _]| x)
                .minmax()
                .into_option()
                .unwrap(),
            self.elves
                .iter()
                .map(|[_, y]| y)
                .minmax()
                .into_option()
                .unwrap(),
        ];
        limits.map(|(min, max)| *min..=*max)
    }

    fn step(&mut self, start_from_direction: usize) -> bool {
        let mut all_suggestions = HashMap::<Coord, Vec<Coord>>::new();
        let mut any_moved = false;
        for elf in self.elves.iter() {
            let surrounding_coords = Direction::all().map(|direction| direction.add_to(*elf));
            let surrounding_elves = surrounding_coords.map(|coord| self.elves.contains(&coord));
            let suggest = if surrounding_elves.iter().all(|has_elf| !has_elf) {
                None
            } else {
                Direction::to_try(start_from_direction).find_map(|direction| {
                    let idx = direction.idx();
                    for i in 7..10 {
                        if surrounding_elves[(i + idx) % 8] {
                            return None;
                        }
                    }
                    Some(surrounding_coords[idx])
                })
            };
            let new_pos = if let Some(suggest) = suggest {
                suggest
            } else {
                *elf
            };
            all_suggestions.entry(new_pos).or_default().push(*elf);
        }
        self.elves = all_suggestions
            .into_iter()
            .flat_map(|(pos, origs)| {
                if origs.len() == 1 {
                    if pos != origs[0] {
                        any_moved = true;
                    }
                    vec![pos]
                } else {
                    origs
                }
            })
            .collect();
        any_moved
    }

    fn calc_empty_ground(&self) -> usize {
        self.limits()
            .into_iter()
            .map(|lim| (lim.end() + 1 - lim.start()) as usize)
            .product::<usize>()
            - self.elves.len()
    }
}

pub fn part_1(input: &[Coord]) -> usize {
    let mut state = State {
        elves: input.iter().copied().collect(),
    };

    for i in 0..10 {
        state.step(i);
    }
    state.calc_empty_ground()
}

pub fn part_2(input: &[Coord]) -> usize {
    let mut state = State {
        elves: input.iter().copied().collect(),
    };
    for i in 0.. {
        let any_moved = state.step(i);
        if !any_moved {
            return i + 1;
        }
    }
    unreachable!()
}
