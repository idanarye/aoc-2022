use std::iter::{from_fn, once};
use std::ops::RangeInclusive;

use hashbrown::HashSet;
use itertools::Itertools;

type Coord = [isize; 2];

#[derive(Debug)]
pub struct SensorInput {
    sensor: Coord,
    beacon: Coord,
}

pub fn generator(input: &str) -> Vec<SensorInput> {
    let pattern =
        regex::Regex::new(r#"^Sensor at x=(.+?), y=(.+?): closest beacon is at x=(.+?), y=(.+?)$"#)
            .unwrap();
    input
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            SensorInput {
                sensor: [captures[1].parse().unwrap(), captures[2].parse().unwrap()],
                beacon: [captures[3].parse().unwrap(), captures[4].parse().unwrap()],
            }
        })
        .collect()
}

impl SensorInput {
    fn detection_distance(&self) -> usize {
        self.sensor[0].abs_diff(self.beacon[0]) + self.sensor[1].abs_diff(self.beacon[1])
    }

    fn coverage_for_row(&self, row: isize) -> Option<RangeInclusive<isize>> {
        let vertical_distance = self.sensor[1].abs_diff(row);
        let detection_distance = self.detection_distance();
        if detection_distance < vertical_distance {
            return None;
        }
        let max_horizontal = (detection_distance - vertical_distance) as isize;
        let col = self.sensor[0];
        Some(col - max_horizontal..=col + max_horizontal)
    }
}

fn normalize_ranges(
    mut ranges: Vec<RangeInclusive<isize>>,
) -> impl Iterator<Item = RangeInclusive<isize>> {
    ranges.sort_by_key(|range| *range.start());
    let mut ranges = ranges.into_iter().peekable();
    from_fn(move || {
        let mut combined_range = ranges.next()?;
        while let Some(next_range) =
            ranges.next_if(|next_range| *next_range.start() <= combined_range.end() + 1)
        {
            if combined_range.end() < next_range.end() {
                combined_range = *combined_range.start()..=*next_range.end();
            }
        }
        Some(combined_range)
    })
}

fn normalize_ranges_for_row(
    input: &[SensorInput],
    row: isize,
) -> impl Iterator<Item = RangeInclusive<isize>> {
    normalize_ranges(
        input
            .iter()
            .filter_map(|si| si.coverage_for_row(row))
            .collect(),
    )
}

pub fn part_1(input: &[SensorInput]) -> usize {
    let row = 2000000;
    let num_covered_at_row: usize = normalize_ranges_for_row(input, row)
        .map(|range| (range.end() - range.start() + 1) as usize)
        .sum();
    let num_beacons_at_row = input
        .iter()
        .filter_map(|si| {
            if si.beacon[1] == row {
                Some(si.beacon[0])
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len();
    num_covered_at_row - num_beacons_at_row
}

fn find_opening(
    allowed_in: RangeInclusive<isize>,
    ranges: impl Iterator<Item = RangeInclusive<isize>>,
) -> Option<isize> {
    let ranges = once(None).chain(ranges.map(Some)).chain(once(None));
    for (prev_range, next_range) in ranges.tuple_windows() {
        match (prev_range, next_range) {
            (None, None) => panic!(),
            (None, Some(next_range)) => {
                if *next_range.start() == allowed_in.start() + 1 {
                    return Some(*allowed_in.start());
                }
            }
            (Some(prev_range), None) => {
                if *prev_range.end() + 1 == *allowed_in.end() {
                    return Some(*allowed_in.end());
                }
            }
            (Some(prev_range), Some(next_range)) => {
                if *prev_range.end() + 2 == *next_range.start() {
                    return Some(prev_range.end() + 1);
                }
            }
        }
    }
    None
}

pub fn part_2(input: &[SensorInput]) -> usize {
    const X_MULTIIPLIER: isize = 4000000;
    let allowed_in = 0..=X_MULTIIPLIER;
    for row in allowed_in.clone() {
        if let Some(col) = find_opening(allowed_in.clone(), normalize_ranges_for_row(input, row)) {
            return (col * X_MULTIIPLIER + row) as usize;
        }
    }
    0
}
