use std::cmp::Reverse;
use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct MonkeyDescription {
    monkey_idx: usize,
    starting_items: Vec<usize>,
    operation: Operation,
    test_division: usize,
    throw_to: [usize; 2],
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Squared,
}

pub fn generator(input: &str) -> Vec<MonkeyDescription> {
    let header_pattern = Regex::new(r#"^Monkey (\d+):$"#).unwrap();
    let startin_items_pattern = Regex::new(r#"^  Starting items: (.*)$"#).unwrap();
    let operation_pattern = Regex::new(r#"^  Operation: new = old (.) (.*)$"#).unwrap();
    let test_pattern = Regex::new(r#"^  Test: divisible by (\d+)$"#).unwrap();
    let throw_pattern = Regex::new(r#"^    If (true|false): throw to monkey (\d+)$"#).unwrap();
    input
        .lines()
        .batching(|it| {
            let mut get_captures = |pattern: &Regex| {
                let line = it.next()?;
                pattern.captures(line)
            };
            let monkey_idx = get_captures(&header_pattern)?[1].parse().unwrap();
            let starting_items = get_captures(&startin_items_pattern)?[1]
                .split(", ")
                .map(|txt| txt.parse().unwrap())
                .collect();
            let operation = {
                let captures = get_captures(&operation_pattern)?;
                match (&captures[1], &captures[2]) {
                    ("+", num) => Operation::Add(num.parse().unwrap()),
                    ("*", "old") => Operation::Squared,
                    ("*", num) => Operation::Multiply(num.parse().unwrap()),
                    _ => panic!("Invalid: {:?}", captures),
                }
            };
            let test_division = get_captures(&test_pattern)?[1].parse().unwrap();
            let mut throw_to = [0, 0];
            for (i, cond) in [(1, "true"), (0, "false")] {
                let captures = get_captures(&throw_pattern)?;
                assert_eq!(&captures[1], cond);
                throw_to[i] = captures[2].parse().unwrap();
            }

            if let Some(line) = it.next() {
                assert_eq!(line, "");
            }
            Some(MonkeyDescription {
                monkey_idx,
                starting_items,
                operation,
                test_division,
                throw_to,
            })
        })
        .collect()
}

impl Operation {
    fn apply(&self, worry_level: usize) -> usize {
        match self {
            Operation::Add(num) => worry_level + num,
            Operation::Multiply(num) => worry_level * num,
            Operation::Squared => worry_level * worry_level,
        }
    }
}

impl MonkeyDescription {
    fn throw_to(&self, worry_level: usize) -> usize {
        if worry_level % self.test_division == 0 {
            self.throw_to[1]
        } else {
            self.throw_to[0]
        }
    }
}

#[derive(Debug)]
struct MonkeyState {
    items: VecDeque<usize>,
    times_inspected: usize,
}

#[derive(Debug)]
struct State {
    lcm: usize,
    monkeys: Vec<MonkeyState>,
}

impl From<&[MonkeyDescription]> for State {
    fn from(descriptions: &[MonkeyDescription]) -> Self {
        let mut lcm = 1;
        for description in descriptions.iter() {
            lcm = num::integer::lcm(lcm, description.test_division);
        }
        let monkeys = descriptions
            .iter()
            .map(|description| MonkeyState {
                items: description.starting_items.iter().copied().collect(),
                times_inspected: 0,
            })
            .collect_vec();
        Self { lcm, monkeys }
    }
}

impl State {
    fn run_monkey(&mut self, monkey_description: &MonkeyDescription, divide_worry_level_by: usize) {
        while let Some(worry_level) = self.monkeys[monkey_description.monkey_idx]
            .items
            .pop_front()
        {
            let new_worry_level = (monkey_description.operation.apply(worry_level)
                / divide_worry_level_by)
                % self.lcm;
            let throw_to = monkey_description.throw_to(new_worry_level);
            self.monkeys[throw_to].items.push_back(new_worry_level);
            self.monkeys[monkey_description.monkey_idx].times_inspected += 1;
        }
    }

    fn run_round(
        &mut self,
        monkey_descriptions: &[MonkeyDescription],
        divide_worry_level_by: usize,
    ) {
        for monkey_description in monkey_descriptions.iter() {
            self.run_monkey(monkey_description, divide_worry_level_by);
        }
    }

    fn monkey_business(&self) -> usize {
        let mut times_inspected = self
            .monkeys
            .iter()
            .map(|monkey| monkey.times_inspected)
            .collect_vec();
        times_inspected.select_nth_unstable_by_key(1, |n| Reverse(*n));
        times_inspected[0] * times_inspected[1]
    }
}

pub fn part_1(input: &[MonkeyDescription]) -> usize {
    let mut state: State = input.into();
    for _ in 0..20 {
        state.run_round(input, 3);
    }
    state.monkey_business()
}

pub fn part_2(input: &[MonkeyDescription]) -> usize {
    let mut state: State = input.into();
    for _ in 0..10000 {
        state.run_round(input, 1);
    }
    state.monkey_business()
}
