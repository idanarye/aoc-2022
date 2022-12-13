use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug)]
pub enum Token {
    Number(usize),
    Open,
    Close,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl From<&str> for Packet {
    fn from(line: &str) -> Self {
        let mut tokens = Vec::new();
        let mut number = String::new();
        for c in line.chars() {
            match c {
                '[' => {
                    assert!(number.is_empty());
                    tokens.push(Token::Open);
                }
                ']' => {
                    if !number.is_empty() {
                        tokens.push(Token::Number(number.parse().unwrap()));
                        number.clear();
                    }
                    tokens.push(Token::Close);
                }
                ',' => {
                    if !number.is_empty() {
                        tokens.push(Token::Number(number.parse().unwrap()));
                        number.clear();
                    }
                }
                _ => {
                    number.push(c);
                }
            }
        }
        assert!(number.is_empty());
        let mut parse_stack = Vec::new();
        parse_stack.push(Vec::<Packet>::new());
        for token in tokens.into_iter() {
            match token {
                Token::Number(number) => {
                    parse_stack.last_mut().unwrap().push(Packet::Number(number));
                }
                Token::Open => {
                    parse_stack.push(Vec::<Packet>::new());
                }
                Token::Close => {
                    let packet = Packet::List(parse_stack.pop().unwrap());
                    parse_stack.last_mut().unwrap().push(packet);
                }
            }
        }
        parse_stack
            .into_iter()
            .exactly_one()
            .unwrap()
            .into_iter()
            .exactly_one()
            .unwrap()
    }
}

pub fn generator(input: &str) -> Vec<[Packet; 2]> {
    input
        .lines()
        .batching(|it| {
            let lines = it.take_while(|line| !line.is_empty()).collect_vec();
            match lines.len() {
                0 => None,
                2 => Some([lines[0].into(), lines[1].into()]),
                _ => panic!(),
            }
        })
        .collect()
}

impl Packet {
    fn num_to_list(&self) -> Packet {
        if let Packet::Number(num) = self {
            Packet::List(vec![Packet::Number(*num)])
        } else {
            panic!()
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Number(num1), Packet::Number(num2)) => num1.cmp(num2),
            (p1 @ Packet::Number(_), p2 @ Packet::List(_)) => p1.num_to_list().cmp(p2),
            (p1 @ Packet::List(_), p2 @ Packet::Number(_)) => p1.cmp(&p2.num_to_list()),
            (Packet::List(list1), Packet::List(list2)) => list1
                .iter()
                .zip_longest(list2)
                .find_map(|pair| {
                    use itertools::EitherOrBoth;
                    match pair {
                        EitherOrBoth::Both(item1, item2) => match item1.cmp(item2) {
                            Ordering::Equal => None,
                            ord => Some(ord),
                        },
                        EitherOrBoth::Left(_) => Some(Ordering::Greater),
                        EitherOrBoth::Right(_) => Some(Ordering::Less),
                    }
                })
                .unwrap_or(Ordering::Equal),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_1(input: &[[Packet; 2]]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, [p1, p2])| if p1 <= p2 { Some(i + 1) } else { None })
        .sum()
}

pub fn part_2(input: &[[Packet; 2]]) -> usize {
    // Second value is for marking divider packets
    let mut packets = input
        .iter()
        .flatten()
        .map(|packet| (packet, false))
        .collect_vec();
    let divider_packets = ["[[2]]".into(), "[[6]]".into()];
    packets.extend(divider_packets.iter().map(|packet| (packet, true)));
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter_map(
            |(i, (_packet, is_divider))| {
                if *is_divider {
                    Some(i + 1)
                } else {
                    None
                }
            },
        )
        .product()
}
