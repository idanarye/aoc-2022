use hashbrown::HashSet;

#[derive(Debug)]
pub struct MovementInstruction {
    amount: usize,
    direction: Direcetion,
}

#[derive(Debug, Clone, Copy)]
enum Direcetion {
    Left,
    Right,
    Up,
    Down,
}

impl Direcetion {
    fn as_vec2(&self) -> [isize; 2] {
        match self {
            Direcetion::Left => [-1, 0],
            Direcetion::Right => [1, 0],
            Direcetion::Up => [0, 1],
            Direcetion::Down => [0, -1],
        }
    }
}

pub fn generator(input: &str) -> Vec<MovementInstruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let direction = match parts.next().unwrap() {
                "L" => Direcetion::Left,
                "R" => Direcetion::Right,
                "U" => Direcetion::Up,
                "D" => Direcetion::Down,
                _ => panic!(),
            };
            let amount = parts.next().unwrap().parse().unwrap();
            MovementInstruction { amount, direction }
        })
        .collect()
}

#[derive(Debug)]
struct State {
    nodes: Vec<[isize; 2]>,
}

impl State {
    fn new(size: usize) -> Self {
        Self {
            nodes: vec![[0, 0]; size],
        }
    }

    fn tail(&self) -> [isize; 2] {
        *self.nodes.last().unwrap()
    }

    fn move_head(&mut self, direction: Direcetion) {
        for (head, vel) in self.nodes[0].iter_mut().zip(direction.as_vec2()) {
            *head += vel;
        }
        for idx in 1..self.nodes.len() {
            let head = self.nodes[idx - 1];
            let tail = &mut self.nodes[idx];

            let touching = head
                .into_iter()
                .zip(tail.iter().copied())
                .all(|(h, t)| (h - t).abs() <= 1);
            if !touching {
                for (head, tail) in head.into_iter().zip(tail.iter_mut()) {
                    *tail += (head - *tail).signum();
                }
            }
        }
    }
}

fn calc_for(input: &[MovementInstruction], rope_size: usize) -> usize {
    let mut state = State::new(rope_size);
    let mut tail_positions = HashSet::new();
    for instruction in input {
        for _ in 0..instruction.amount {
            state.move_head(instruction.direction);
            tail_positions.insert(state.tail());
        }
    }
    tail_positions.len()
}

pub fn part_1(input: &[MovementInstruction]) -> usize {
    calc_for(input, 2)
}

pub fn part_2(input: &[MovementInstruction]) -> usize {
    calc_for(input, 10)
}
