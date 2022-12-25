pub fn generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

fn parse_snafu(snafu: &str) -> isize {
    let mut result = 0;
    for c in snafu.chars() {
        result *= 5;
        result += match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("Illegal snafu character {}", c),
        };
    }
    result
}

fn format_snafu(mut number: isize) -> String {
    let mut result = Vec::<char>::new();
    while 0 != number {
        result.push(match number % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                number += 5;
                '='
            }
            4 => {
                number += 5;
                '-'
            }
            _ => panic!(),
        });
        number /= 5;
    }
    result.reverse();
    result.into_iter().collect()
}

pub fn part_1(input: &[String]) -> String {
    let total = input.iter().map(|s| parse_snafu(s)).sum::<isize>();
    format_snafu(total)
}

pub fn part_2(input: &[String]) -> isize {
    let _ = input;
    0
}
