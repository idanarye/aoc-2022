use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct PairRanges([RangeInclusive<usize>; 2]);

pub fn generator(input: &str) -> Vec<PairRanges> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(",").map(|elf| {
                let mut it = elf.split("-");
                it.next().unwrap().parse().unwrap()..=it.next().unwrap().parse().unwrap()
            });
            PairRanges([it.next().unwrap(), it.next().unwrap()])
        })
        .collect()
}

impl PairRanges {
    fn is_i_contained_in_j(&self, i: usize, j: usize) -> bool {
        let i = &self.0[i];
        let j = &self.0[j];
        j.contains(i.start()) && j.contains(i.end())
    }

    fn is_one_contained_in_the_other(&self) -> bool {
        self.is_i_contained_in_j(0, 1) || self.is_i_contained_in_j(1, 0)
    }

    fn is_overlapping(&self) -> bool {
        let [a, b] = &self.0;
        a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
    }
}

pub fn part_1(input: &[PairRanges]) -> usize {
    input
        .iter()
        .filter(|pair_ranges| pair_ranges.is_one_contained_in_the_other())
        .count()
}

pub fn part_2(input: &[PairRanges]) -> usize {
    input
        .iter()
        .filter(|pair_ranges| pair_ranges.is_overlapping())
        .count()
}
