use std::fmt::Display;

use itertools::Itertools;

type Input = (Arrangement, Vec<Command>);

#[derive(Debug, Clone)]
pub struct Arrangement(Vec<Vec<char>>);

#[derive(Debug)]
pub struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

pub fn generator(input: &str) -> Input {
    let lines = input.lines().collect_vec();
    let (sepline, _) = lines.iter().find_position(|line| line.is_empty()).unwrap();
    let (arrangement, commands) = lines.split_at(sepline);
    let arrangement = Arrangement({
        let mut it = arrangement.iter().rev();
        let stacks_index = it
            .next()
            .unwrap()
            .chars()
            .map(|c| {
                if c == ' ' {
                    None
                } else {
                    Some(c as usize - '0' as usize - 1)
                }
            })
            .collect_vec();
        let num_stacks = stacks_index.iter().filter(|idx| idx.is_some()).count();
        let mut stacks = vec![Vec::<char>::new(); num_stacks];
        for line in it {
            for (c, stack_index) in line.chars().zip(stacks_index.iter()) {
                if let Some(stack_index) = stack_index {
                    if c != ' ' {
                        stacks[*stack_index].push(c);
                    }
                }
            }
        }
        stacks
    });
    let pattern = regex::Regex::new(r#"move (\d+) from (\d+) to (\d+)"#).unwrap();
    let commands = commands
        .iter()
        .skip_while(|l| l.is_empty())
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            Command {
                amount: captures[1].parse::<usize>().unwrap(),
                from: captures[2].parse::<usize>().unwrap() - 1,
                to: captures[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect_vec();
    (arrangement, commands)
}

impl Display for Arrangement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;
        let max_size = self.0.iter().map(|s| s.len()).max().unwrap_or(0);
        for j in (0..max_size).rev() {
            for (i, stack) in self.0.iter().enumerate() {
                if 0 < i {
                    f.write_char(' ')?;
                }
                if j < stack.len() {
                    write!(f, "[{}]", stack[j])?;
                } else {
                    f.write_str("   ")?;
                }
            }
            f.write_char('\n')?;
        }
        for i in 0..self.0.len() {
            if 0 < i {
                f.write_char(' ')?;
            }
            write!(f, " {} ", i + 1)?;
        }
        Ok(())
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // use std::fmt::Write;
        write!(
            f,
            "move {} from {} to {}",
            self.amount,
            self.from + 1,
            self.to + 1
        )
    }
}

impl Arrangement {
    fn apply_command(&mut self, command: &Command, reverse: bool) {
        let mut splet = {
            let from = &mut self.0[command.from];
            from.split_off(from.len() - command.amount)
        };
        if reverse {
            splet.reverse();
        }
        self.0[command.to].extend(splet);
    }

    fn code(&self) -> String {
        let mut result = String::new();
        for stack in self.0.iter() {
            result.push(*stack.last().unwrap());
        }
        result
    }
}

pub fn part_1((arrangement, commands): &Input) -> String {
    let mut arrangement = arrangement.clone();
    for command in commands {
        arrangement.apply_command(command, true);
    }
    arrangement.code()
}

pub fn part_2((arrangement, commands): &Input) -> String {
    let mut arrangement = arrangement.clone();
    for command in commands {
        arrangement.apply_command(command, false);
    }
    arrangement.code()
}
