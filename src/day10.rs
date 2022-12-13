use std::iter::from_fn;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    AddX(isize),
}

pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line == "noop" {
                Instruction::Noop
            } else {
                Instruction::AddX(line.split(' ').last().unwrap().parse().unwrap())
            }
        })
        .collect()
}

fn x_values<'a>(
    mut instructions: impl Iterator<Item = &'a Instruction>,
) -> impl Iterator<Item = isize> {
    let mut current_x_value = 1;

    enum State {
        ReadInstruction,
        ProcessAddX(isize),
    }
    let mut state = State::ReadInstruction;

    from_fn(move || {
        let result = Some(current_x_value);
        state = match state {
            State::ReadInstruction => match instructions.next()? {
                Instruction::Noop => State::ReadInstruction,
                Instruction::AddX(addition_to_x) => State::ProcessAddX(*addition_to_x),
            },
            State::ProcessAddX(addition_to_x) => {
                current_x_value += addition_to_x;
                State::ReadInstruction
            }
        };
        result
    })
}

pub fn part_1(input: &[Instruction]) -> isize {
    x_values(input.iter())
        .enumerate()
        .filter_map(|(i, x_value)| {
            let cycle_numer = i + 1;
            if (cycle_numer + 20) % 40 == 0 {
                Some(cycle_numer as isize * x_value)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_2(input: &[Instruction]) -> String {
    let mut output = String::new();
    for (i, x_value) in x_values(input.iter()).enumerate() {
        if i % 40 == 0 {
            output.push('\n');
        }
        let h_position = i % 40;
        if (-1..2).contains(&(h_position as isize - x_value)) {
            output.push('#');
        } else {
            output.push('.');
        }
    }
    output
}
