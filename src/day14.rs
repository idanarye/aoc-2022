use std::fmt::{Display, Write};
use std::ops::Index;

use hashbrown::HashMap;
use itertools::Itertools;

type Coord = [isize; 2];
type RockPath = Vec<Coord>;

pub fn generator(input: &str) -> Vec<RockPath> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    coord
                        .split(',')
                        .map(|num| num.parse().unwrap())
                        .collect_vec()
                        .try_into()
                        .unwrap()
                })
                .collect()
        })
        .collect()
}

fn coord_op(a: Coord, b: Coord, mut dlg: impl FnMut(isize, isize) -> isize) -> Coord {
    [dlg(a[0], b[0]), dlg(a[1], b[1])]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Object {
    Air,
    Rock,
    Sand,
}

impl Object {
    fn is_air(&self) -> bool {
        *self == Object::Air
    }
}

#[derive(Default, Debug)]
struct CaveMap {
    objects: HashMap<Coord, Object>,
}

impl Index<Coord> for CaveMap {
    type Output = Object;

    fn index(&self, index: Coord) -> &Self::Output {
        self.objects.get(&index).unwrap_or(&Object::Air)
    }
}

impl Display for CaveMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut min = [isize::MAX; 2];
        let mut max = [isize::MIN; 2];
        for coord in self.objects.keys() {
            for i in 0..2 {
                min[i] = min[i].min(coord[i]);
                max[i] = max[i].max(coord[i]);
            }
        }
        for y in min[1]..(max[1] + 1) {
            f.write_char('\n')?;
            for x in min[0]..(max[0] + 1) {
                f.write_char(match self[[x, y]] {
                    Object::Air => '.',
                    Object::Rock => '#',
                    Object::Sand => 'o',
                })?
            }
        }
        Ok(())
    }
}

impl CaveMap {
    fn add_rock_path(&mut self, path: &RockPath) {
        for (from, to) in path.iter().tuple_windows() {
            let direction = coord_op(*from, *to, |f, t| (t - f).signum());
            let num_steps = if direction[0] == 0 {
                (to[1] - from[1]).abs() + 1
            } else if direction[1] == 0 {
                (to[0] - from[0]).abs() + 1
            } else {
                panic!()
            };
            for i in 0..num_steps {
                self.objects
                    .insert(coord_op(*from, direction, |f, d| f + i * d), Object::Rock);
            }
        }
    }

    fn add_sand(&mut self, pos: Coord) {
        self.objects.insert(pos, Object::Sand);
    }

    fn lowest(&self) -> isize {
        self.objects.keys().map(|[_, y]| *y).max().unwrap()
    }

    fn trace_sand(
        &self,
        mut pos: Coord,
        mut continue_while: impl FnMut(Coord) -> bool,
    ) -> Result<Coord, Coord> {
        'outer: while continue_while(pos) {
            for direction in [
                [0, 1],  // down
                [-1, 1], // down + left
                [1, 1],  // down + right
            ] {
                let new_pos = coord_op(pos, direction, |p, d| p + d);
                if self[new_pos].is_air() {
                    pos = new_pos;
                    continue 'outer;
                }
            }
            return Ok(pos);
        }
        Err(pos)
    }
}

pub fn part_1(input: &[RockPath]) -> usize {
    let mut cave_map = CaveMap::default();
    for rock_path in input.iter() {
        cave_map.add_rock_path(rock_path);
    }
    let max = cave_map.lowest();
    for i in 0.. {
        let final_pos = cave_map.trace_sand([500, 0], |[_, y]| y < max);
        if let Ok(pos) = final_pos {
            cave_map.add_sand(pos);
        } else {
            return i;
        }
    }
    0
}

pub fn part_2(input: &[RockPath]) -> usize {
    let mut cave_map = CaveMap::default();
    for rock_path in input.iter() {
        cave_map.add_rock_path(rock_path);
    }
    let lowest = cave_map.lowest() + 1; // floor is +2 - so one above it is +1
    for i in 1.. {
        let final_pos = cave_map.trace_sand([500, 0], |[_, y]| y < lowest);
        match final_pos {
            Ok(pos) => {
                cave_map.add_sand(pos);
                if pos == [500, 0] {
                    return i;
                }
            }
            Err(pos) => {
                cave_map.add_sand(pos);
            }
        }
    }
    0
}
