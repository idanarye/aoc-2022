use std::iter::from_fn;
use std::ops::RangeInclusive;

use hashbrown::HashSet;
use itertools::Itertools;

use crate::bfs::HashMapBfs;

type Coord = [isize; 3];

pub fn generator(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect()
}

struct CubeMap {
    cubes: HashSet<Coord>,
    limits: [RangeInclusive<isize>; 3],
}

impl FromIterator<Coord> for CubeMap {
    fn from_iter<T: IntoIterator<Item = Coord>>(iter: T) -> Self {
        let cubes: HashSet<Coord> = iter.into_iter().collect();
        let limits = (0..3)
            .map(|i| match cubes.iter().map(|cube| cube[i]).minmax() {
                itertools::MinMaxResult::NoElements => panic!(),
                itertools::MinMaxResult::OneElement(c) => (c - 1)..=(c + 2),
                itertools::MinMaxResult::MinMax(mn, mx) => (mn - 1)..=(mx + 1),
            })
            .collect_vec()
            .try_into()
            .unwrap();
        Self { cubes, limits }
    }
}

fn neighbors(coord: Coord) -> impl Iterator<Item = Coord> {
    [
        [-1, 0, 0],
        [1, 0, 0],
        [0, -1, 0],
        [0, 1, 0],
        [0, 0, -1],
        [0, 0, 1],
    ]
    .iter()
    .map(move |adj| {
        let mut neighbor = coord;
        for (c, a) in neighbor.iter_mut().zip(adj) {
            *c += a;
        }
        neighbor
    })
}

impl CubeMap {
    fn count_surface_area(&self) -> usize {
        self.cubes
            .iter()
            .flat_map(|cube| {
                neighbors(*cube).map(
                    |neighbor| {
                        if self.cubes.contains(&neighbor) {
                            0
                        } else {
                            1
                        }
                    },
                )
            })
            .sum()
    }

    fn flood_fill(&self, start: Coord) -> impl '_ + Iterator<Item = Coord> {
        let mut bfs = HashMapBfs::<Coord, u8>::new();
        bfs.add_root(start, 0);
        from_fn(move || {
            bfs.consider_next().map(|coord| {
                for neighbor in neighbors(coord) {
                    if !self.cubes.contains(&neighbor)
                        && self
                            .limits
                            .iter()
                            .zip(neighbor)
                            .all(|(l, n)| l.contains(&n))
                    {
                        bfs.add_edge(coord, neighbor, 0);
                    }
                }
                coord
            })
        })
    }
}

pub fn part_1(input: &[Coord]) -> usize {
    let cubes_map = input.iter().copied().collect::<CubeMap>();
    cubes_map.count_surface_area()
}

pub fn part_2(input: &[Coord]) -> usize {
    let mut cubes_map = input.iter().copied().collect::<CubeMap>();
    let mut non_interior = cubes_map
        .flood_fill(cubes_map.limits.clone().map(|l| *l.start()))
        .collect::<HashSet<Coord>>();
    non_interior.extend(cubes_map.cubes.iter());
    cubes_map.cubes.extend(
        cubes_map
            .limits
            .iter()
            .cloned()
            .multi_cartesian_product()
            .filter_map(|coord| {
                let coord: Coord = coord.try_into().unwrap();
                if non_interior.contains(&coord) {
                    None
                } else {
                    Some(coord)
                }
            }),
    );
    cubes_map.count_surface_area()
}
