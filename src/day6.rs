pub fn generator(input: &str) -> String {
    input.trim().to_owned()
}

fn marker_end_position(signal: &str, length: usize) -> usize {
    'outer: for i in 0..(signal.len() - length) {
        let slc = &signal[i..(i + length)];
        let mut seen = [false; 26];
        for letter in slc.chars() {
            let idx = letter as usize - 'a' as usize;
            if seen[idx] {
                continue 'outer;
            }
            seen[idx] = true;
        }
        return i + length;
    }
    0
}

pub fn part_1(input: &str) -> usize {
    marker_end_position(input, 4)
}

pub fn part_2(input: &str) -> usize {
    marker_end_position(input, 14)
}
