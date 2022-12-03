#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("Bad command {:?}", c),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RoundResult {
    Lose,
    Draw,
    Win,
}

impl From<char> for RoundResult {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Bad command {:?}", c),
        }
    }
}

impl RoundResult {
    fn result_score(&self) -> usize {
        match self {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }

    fn when_playing_against(&self, opponent: Shape) -> Shape {
        use RoundResult::*;
        use Shape::*;
        match (self, opponent) {
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Draw, Rock) => Rock,
            (Draw, Paper) => Paper,
            (Draw, Scissors) => Scissors,
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
        }
    }
}

impl Shape {
    fn shape_score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn play_against(&self, other: Shape) -> RoundResult {
        use RoundResult::*;
        use Shape::*;
        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Paper) => Draw,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper) => Win,
            (Scissors, Scissors) => Draw,
        }
    }
}

#[derive(Debug)]
pub struct RoundStrategy {
    opponent: Shape,
    your: Shape,
    desired: RoundResult,
}

impl RoundStrategy {
    fn score1(&self) -> usize {
        self.your.shape_score() + self.your.play_against(self.opponent).result_score()
    }

    fn score2(&self) -> usize {
        self.desired
            .when_playing_against(self.opponent)
            .shape_score()
            + self.desired.result_score()
    }
}

pub fn generator(input: &str) -> Vec<RoundStrategy> {
    input
        .lines()
        .map(|line| {
            let mut it = line.chars();
            let opponent = it.next().unwrap().into();
            it.next().unwrap();
            let other_param = it.next().unwrap();
            RoundStrategy {
                opponent,
                your: other_param.into(),
                desired: other_param.into(),
            }
        })
        .collect()
}

pub fn part_1(input: &[RoundStrategy]) -> usize {
    input.iter().map(|r| r.score1()).sum()
}

pub fn part_2(input: &[RoundStrategy]) -> usize {
    input.iter().map(|r| r.score2()).sum()
}
