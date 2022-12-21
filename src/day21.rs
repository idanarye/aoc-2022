use std::cmp::Reverse;
use std::fmt::{Debug, Display, Write};
use std::str::FromStr;

use hashbrown::HashMap;
use itertools::Itertools;

use crate::bfs::HashMapBfs;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct MonkeyName([char; 4]);

impl Debug for MonkeyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0.iter().collect::<String>(), f)
    }
}

impl Display for MonkeyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.0 {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl FromStr for MonkeyName {
    type Err = Vec<char>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect_vec().try_into()?))
    }
}

pub type RowData = (MonkeyName, MonkeyYell);

#[derive(Debug, Clone)]
pub enum MonkeyYell {
    Number(isize),
    Add(MonkeyName, MonkeyName),
    Sub(MonkeyName, MonkeyName),
    Mul(MonkeyName, MonkeyName),
    Div(MonkeyName, MonkeyName),
}

impl MonkeyYell {
    fn dependencies(&self) -> Option<[MonkeyName; 2]> {
        match self {
            MonkeyYell::Number(_) => None,
            MonkeyYell::Add(monkey1, monkey2) => Some([*monkey1, *monkey2]),
            MonkeyYell::Sub(monkey1, monkey2) => Some([*monkey1, *monkey2]),
            MonkeyYell::Mul(monkey1, monkey2) => Some([*monkey1, *monkey2]),
            MonkeyYell::Div(monkey1, monkey2) => Some([*monkey1, *monkey2]),
        }
    }

    fn resolve_with(&self, mut get_dlg: impl FnMut(MonkeyName) -> Option<isize>) -> Option<isize> {
        match self {
            MonkeyYell::Number(number) => Some(*number),
            MonkeyYell::Add(monkey1, monkey2) => Some(get_dlg(*monkey1)? + get_dlg(*monkey2)?),
            MonkeyYell::Sub(monkey1, monkey2) => Some(get_dlg(*monkey1)? - get_dlg(*monkey2)?),
            MonkeyYell::Mul(monkey1, monkey2) => Some(get_dlg(*monkey1)? * get_dlg(*monkey2)?),
            MonkeyYell::Div(monkey1, monkey2) => Some(get_dlg(*monkey1)? / get_dlg(*monkey2)?),
        }
    }
}

pub fn generator(input: &str) -> Vec<RowData> {
    let pattern = regex::Regex::new(r#"(\w+): (?:(\d+)|(\w+) ([+\-*/]) (\w+))$"#).unwrap();
    input
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            let monkey_name: MonkeyName = captures[1].parse().unwrap();
            let monkey_yell = if let Some(number) = captures.get(2) {
                MonkeyYell::Number(number.as_str().parse().unwrap())
            } else {
                let [monkey1, monkey2] = [3, 5].map(|i| captures[i].parse::<MonkeyName>().unwrap());
                match &captures[4] {
                    "+" => MonkeyYell::Add(monkey1, monkey2),
                    "-" => MonkeyYell::Sub(monkey1, monkey2),
                    "*" => MonkeyYell::Mul(monkey1, monkey2),
                    "/" => MonkeyYell::Div(monkey1, monkey2),
                    _ => panic!(),
                }
            };
            (monkey_name, monkey_yell)
        })
        .collect()
}

fn get_concrete_numbers_ignoring(
    monkey_map: &HashMap<MonkeyName, MonkeyYell>,
    root: MonkeyName,
    mut ignore_pred: impl FnMut(MonkeyName) -> bool,
) -> HashMap<MonkeyName, isize> {
    let mut bfs = HashMapBfs::<MonkeyName, usize>::new();
    bfs.add_root(root, 0);
    while let Some(monkey) = bfs.consider_next() {
        let yell = &monkey_map[&monkey];
        for dependency in yell.dependencies().into_iter().flatten() {
            bfs.add_edge(monkey, dependency, 1);
        }
    }
    let mut concrete_numbers = HashMap::new();
    let by_order = bfs
        .all_known()
        .sorted_by_key(|monkey_name| Reverse(bfs.cost(monkey_name)));
    for monkey in by_order {
        if ignore_pred(*monkey) {
            continue;
        }
        if let Some(number) =
            monkey_map[monkey].resolve_with(|monkey| concrete_numbers.get(&monkey).copied())
        {
            concrete_numbers.insert(*monkey, number);
        }
    }
    concrete_numbers
}

const ROOT: MonkeyName = MonkeyName(['r', 'o', 'o', 't']);
const HUMN: MonkeyName = MonkeyName(['h', 'u', 'm', 'n']);

pub fn part_1(input: &[RowData]) -> isize {
    let monkey_map: HashMap<MonkeyName, MonkeyYell> = input.iter().cloned().collect();
    get_concrete_numbers_ignoring(&monkey_map, ROOT, |_| false)[&ROOT]
}

pub fn part_2(input: &[RowData]) -> isize {
    let monkey_map: HashMap<MonkeyName, MonkeyYell> = input.iter().cloned().collect();
    let concrete_numbers =
        get_concrete_numbers_ignoring(&monkey_map, ROOT, |monkey_name| monkey_name == HUMN);
    assert!(!concrete_numbers.contains_key(&ROOT));
    let root_deps = monkey_map[&ROOT].dependencies().unwrap();
    let (mut monkey_needs_to_be, mut needs_to_be) =
        match root_deps.map(|monkey_name| concrete_numbers.get(&monkey_name)) {
            [Some(num), None] => (root_deps[1], *num),
            [None, Some(num)] => (root_deps[0], *num),
            _ => panic!(),
        };
    assert!(!concrete_numbers.contains_key(&monkey_needs_to_be));

    while monkey_needs_to_be != HUMN {
        let deps = monkey_map[&monkey_needs_to_be]
            .dependencies()
            .unwrap()
            .map(|monkey_name| concrete_numbers.get(&monkey_name).copied());
        match (&monkey_map[&monkey_needs_to_be], deps) {
            (MonkeyYell::Add(monkey, _), [None, Some(number)]) => {
                needs_to_be -= number;
                monkey_needs_to_be = *monkey;
            }
            (MonkeyYell::Add(_, monkey), [Some(number), None]) => {
                needs_to_be -= number;
                monkey_needs_to_be = *monkey;
            }

            (MonkeyYell::Sub(monkey, _), [None, Some(number)]) => {
                needs_to_be += number;
                monkey_needs_to_be = *monkey;
            }
            (MonkeyYell::Sub(_, monkey), [Some(number), None]) => {
                needs_to_be = -needs_to_be + number;
                monkey_needs_to_be = *monkey;
            }

            (MonkeyYell::Mul(monkey, _), [None, Some(number)]) => {
                needs_to_be /= number;
                monkey_needs_to_be = *monkey;
            }
            (MonkeyYell::Mul(_, monkey), [Some(number), None]) => {
                needs_to_be /= number;
                monkey_needs_to_be = *monkey;
            }

            (MonkeyYell::Div(monkey, _), [None, Some(number)]) => {
                needs_to_be *= number;
                monkey_needs_to_be = *monkey;
            }
            (MonkeyYell::Div(_, monkey), [Some(number), None]) => {
                needs_to_be = number / needs_to_be;
                monkey_needs_to_be = *monkey;
            }
            _ => panic!(),
        }
    }
    needs_to_be
}
