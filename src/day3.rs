use hashbrown::HashSet;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Rucksack(Vec<char>);

pub fn generator(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|line| Rucksack(line.chars().collect()))
        .collect()
}

fn item_type_priority(item_type: char) -> usize {
    if item_type.is_lowercase() {
        item_type as usize - 'a' as usize + 1
    } else {
        item_type as usize - 'A' as usize + 27
    }
}

impl Rucksack {
    fn compartments(&self) -> (&[char], &[char]) {
        self.0.split_at(self.0.len() / 2)
    }

    fn error(&self) -> char {
        let (first_half, second_half) = self.compartments();
        let first_half_item_types: HashSet<char> = first_half.iter().copied().collect();
        second_half
            .iter()
            .copied()
            .find(|item_type| first_half_item_types.contains(item_type))
            .unwrap()
    }
}

pub fn part_1(input: &[Rucksack]) -> usize {
    input
        .iter()
        .map(|rucksack| item_type_priority(rucksack.error()))
        .sum()
}

#[derive(Debug)]
struct ElvesGroup([Rucksack; 3]);

impl ElvesGroup {
    fn elf_items<'a>(&'a self, elf_idx: usize) -> impl 'a + Iterator<Item = char> {
        self.0[elf_idx].0.iter().copied()
    }

    fn badge(&self) -> char {
        let first_elf_item_types: HashSet<char> = self.elf_items(0).collect();
        let second_elf_item_types: HashSet<char> = self
            .elf_items(1)
            .filter(|item_type| first_elf_item_types.contains(item_type))
            .collect();
        self.elf_items(2)
            .find(|item_type| second_elf_item_types.contains(item_type))
            .unwrap()
    }
}

pub fn part_2(input: &[Rucksack]) -> usize {
    let groups = input
        .iter()
        .cloned()
        .tuples()
        .map(|(a, b, c)| ElvesGroup([a, b, c]))
        .collect::<Vec<_>>();
    groups
        .iter()
        .map(|elves_group| item_type_priority(elves_group.badge()))
        .sum()
}
