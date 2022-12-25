use std::iter::repeat_with;

use hashbrown::HashMap;
use itertools::Itertools;

use crate::bfs::LinearBfs;

#[derive(Debug)]
pub struct ValveDescription {
    name: String,
    rate: usize,
    tunnels: Vec<String>,
}

pub fn generator(input: &str) -> Vec<ValveDescription> {
    let pattern =
        regex::Regex::new(r#"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)$"#)
            .unwrap();
    input
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            ValveDescription {
                name: captures[1].to_owned(),
                rate: captures[2].parse().unwrap(),
                tunnels: captures[3].split(", ").map(|s| s.to_owned()).collect(),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Mapping {
    idx_to_name: Vec<String>,
    name_to_idx: HashMap<String, usize>,
    relevant_indices: Vec<usize>,
    rates: Vec<usize>,
    routes: Vec<Vec<usize>>,
}

impl From<&[ValveDescription]> for Mapping {
    fn from(valves_descriptions: &[ValveDescription]) -> Self {
        let mut idx_to_name = Vec::new();
        let mut relevant_indices = Vec::new();
        let mut name_to_idx = HashMap::new();
        let mut rates = Vec::new();

        for (i, valve) in valves_descriptions.iter().enumerate() {
            idx_to_name.push(valve.name.clone());
            name_to_idx.insert(valve.name.clone(), i);
            if 0 < valve.rate {
                relevant_indices.push(i);
            }
            rates.push(valve.rate);
        }

        let tunnels = valves_descriptions
            .iter()
            .map(|valve| valve.tunnels.iter().map(|s| name_to_idx[s]).collect_vec())
            .collect_vec();

        let routes = (0..tunnels.len())
            .flat_map(|source_idx| {
                let mut bfs = LinearBfs::<usize>::new(tunnels.len());
                let mut routes = vec![Vec::new(); tunnels.len()];
                bfs.add_root(source_idx, 0);
                while let Some(visited_idx) = bfs.consider_next() {
                    routes[visited_idx] = bfs.path_to(visited_idx);
                    for neighbor in tunnels[visited_idx].iter() {
                        bfs.add_edge(visited_idx, *neighbor, 1);
                    }
                }
                routes
            })
            .collect_vec();

        Self {
            idx_to_name,
            name_to_idx,
            relevant_indices,
            rates,
            routes,
        }
    }
}

enum Step {
    Pass { pos: usize, goal: usize },
    Open(usize),
}

impl Step {
    fn idx(&self) -> usize {
        match self {
            Step::Pass { pos, goal: _ } => *pos,
            Step::Open(idx) => *idx,
        }
    }
}

struct Route<'a> {
    mapping: &'a Mapping,
    start_from: usize,
    steps: Vec<Vec<Step>>,
    time: usize,
    combined_rate: usize,
    total_released: usize,
}

impl Mapping {
    fn num_valves(&self) -> usize {
        self.idx_to_name.len()
    }

    fn route(&self, from: usize, to: usize) -> &[usize] {
        &self.routes[from * self.num_valves() + to]
    }

    fn distance(&self, from: usize, to: usize) -> usize {
        self.route(from, to).len() - 1
    }

    fn start_route(&self, start_from: usize, num_participants: usize) -> Route {
        Route {
            mapping: self,
            start_from,
            steps: repeat_with(Vec::new).take(num_participants).collect(),
            time: 0,
            combined_rate: 0,
            total_released: 0,
        }
    }
}

impl Route<'_> {
    fn push_step(&mut self, indices: &[usize]) {
        assert!(indices.len() == self.steps.len());
        let min_distance = indices
            .iter()
            .zip(self.steps.iter())
            .map(|(idx, steps)| {
                let curr_idx = steps.last().map(|s| s.idx()).unwrap_or(self.start_from);
                self.mapping.distance(curr_idx, *idx)
            })
            .min()
            .unwrap();
        let action_duration = min_distance + 1;
        self.time += action_duration;
        self.total_released += action_duration * self.combined_rate;
        for (&idx, steps) in indices.iter().zip(self.steps.iter_mut()) {
            let curr_idx = steps.last().map(|s| s.idx()).unwrap_or(self.start_from);
            let distance = self.mapping.distance(curr_idx, idx);
            if distance == min_distance {
                self.combined_rate += self.mapping.rates[idx];
                steps.push(Step::Open(idx));
            } else {
                let route = self.mapping.route(curr_idx, idx);
                steps.push(Step::Pass {
                    pos: route[action_duration],
                    goal: idx,
                });
            }
        }
    }

    fn pop_step(&mut self) {
        let mut action_duration = 0;
        for steps in self.steps.iter_mut() {
            let removed_step = steps.pop().unwrap();
            let prev_idx = steps.last().map(|s| s.idx()).unwrap_or(self.start_from);
            let distance = self.mapping.distance(prev_idx, removed_step.idx());
            match removed_step {
                Step::Pass { .. } => {
                    assert!(action_duration == 0 || action_duration == distance);
                    action_duration = distance;
                }
                Step::Open(removed_idx) => {
                    assert!(action_duration == 0 || action_duration == distance + 1);
                    action_duration = distance + 1;
                    self.combined_rate -= self.mapping.rates[removed_idx];
                }
            }
        }
        self.time -= action_duration;
        self.total_released -= action_duration * self.combined_rate;
    }

    fn total_released_if_continued_until(&self, time_limit: usize) -> usize {
        let remaining_time = time_limit - self.time;
        self.total_released + remaining_time * self.combined_rate
    }

    fn find_best_under_time_limit(&mut self, time_limit: usize) -> usize {
        let mut best = self.total_released_if_continued_until(time_limit);
        let mut already_open = self
            .steps
            .iter()
            .flatten()
            .map(|step| match step {
                Step::Pass { pos: _, goal } => goal,
                Step::Open(idx) => idx,
            })
            .collect_vec();
        already_open.sort();
        let mut already_open = already_open.into_iter().peekable();

        let left_to_open = self
            .mapping
            .relevant_indices
            .iter()
            .copied()
            .filter(|idx| {
                while let Some(open_idx) = already_open.next_if(|open_idx| *open_idx <= idx) {
                    if open_idx == idx {
                        return false;
                    }
                }
                true
            })
            .collect_vec();

        let mut indices_for_new_steps = Vec::new();
        let mut new_steps_buffer = self
            .steps
            .iter()
            .enumerate()
            .map(|(i, steps)| {
                if let Some(Step::Pass { pos: _, goal }) = steps.last() {
                    *goal
                } else {
                    indices_for_new_steps.push(i);
                    0
                }
            })
            .collect_vec();

        for try_indices in left_to_open
            .into_iter()
            .permutations(indices_for_new_steps.len())
        {
            for (i, try_idx) in indices_for_new_steps.iter().zip(try_indices.iter()) {
                new_steps_buffer[*i] = *try_idx;
            }
            self.push_step(&new_steps_buffer);
            if self.time < time_limit {
                best = best.max(self.find_best_under_time_limit(time_limit));
            }
            self.pop_step();
        }
        best
    }
}

pub fn part_1(input: &[ValveDescription]) -> usize {
    let mapping = Mapping::from(input);
    mapping
        .start_route(mapping.name_to_idx["AA"], 1)
        .find_best_under_time_limit(30)
}

pub fn part_2(input: &[ValveDescription]) -> usize {
    let mapping = Mapping::from(input);
    mapping
        .start_route(mapping.name_to_idx["AA"], 2)
        .find_best_under_time_limit(26)
}
