use itertools::Itertools;

type RowData = Vec<usize>;

pub fn generator(input: &str) -> Vec<RowData> {
    input.lines().batching(|it| {
        let elf_items = it.map_while(|line| {
            if line.is_empty() {
                None
            } else {
                line.parse().ok()
            }
        }).collect::<Vec<usize>>();
        if elf_items.is_empty() {
            None
        } else {
            Some(elf_items)
        }
    }).collect::<Vec<_>>()
}

pub fn part_1(input: &[RowData]) -> usize {
    input.iter().map(|elf_items| elf_items.iter().sum()).max().unwrap()
}

pub fn part_2(input: &[RowData]) -> usize {
    let mut elves_totals = input.iter().map(|elf_items| elf_items.iter().sum()).collect::<Vec<usize>>();
    elves_totals.sort_by_key(|&num| std::cmp::Reverse(num));
    elves_totals.iter().take(3).sum()
}
