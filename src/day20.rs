use std::cmp::Ordering;

use itertools::Itertools;

pub fn generator(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn mix_indices(indices: &mut Vec<usize>, numbers: &[isize]) {
    for (idx, num) in numbers.iter().enumerate() {
        let pos = indices.iter().position(|i| *i == idx).unwrap();
        let new_pos = (pos as isize + num).rem_euclid(numbers.len() as isize - 1) as usize;
        match pos.cmp(&new_pos) {
            Ordering::Equal => {}
            Ordering::Greater => {
                indices.remove(pos);
                indices.insert(new_pos, idx);
            }
            Ordering::Less => {
                indices.remove(pos);
                indices.insert(
                    (new_pos as isize).rem_euclid(numbers.len() as isize - 1) as usize,
                    idx,
                );
            }
        }
    }
}

fn extract_answer(indices: &[usize], numbers: &[isize]) -> isize {
    let zero_pos = indices.iter().position(|idx| numbers[*idx] == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| numbers[indices[(zero_pos + i) % indices.len()]])
        .sum()
}

pub fn part_1(input: &[isize]) -> isize {
    let mut indices = (0..input.len()).collect_vec();
    mix_indices(&mut indices, input);
    extract_answer(&indices, input)
}

pub fn part_2(input: &[isize]) -> isize {
    let new_numbers = input.iter().map(|num| num * 811589153).collect_vec();
    let mut indices = (0..input.len()).collect_vec();
    for _ in 0..10 {
        mix_indices(&mut indices, &new_numbers);
    }
    extract_answer(&indices, &new_numbers)
}
