const DIGITS: [&str; 20] = [
    "zero", "0",
    "one", "1",
    "two", "2",
    "three", "3",
    "four", "4",
    "five", "5",
    "six", "6",
    "seven", "7",
    "eight", "8",
    "nine", "9"
];

fn digitize(what: &str) -> Option<i64> {
    for (i, digit) in DIGITS.iter().enumerate() {
        if what.starts_with(digit) {
            return Some((i / 2) as i64);
        }
    }
    return None;
}

fn digitize_all(what: &str) -> Vec<i64> {
    let mut ret = Vec::new();
    for i in 0..what.len() {
        if let Some(digit) = digitize(&what[i..(what.len())]) {
            ret.push(digit);
        }
    }
    ret
}

// https://adventofcode.com/2023/day/1
fn trebuchet(s: &str) -> i64 {
    let digits = digitize_all(s);
    digits.first().unwrap() * 10 + digits.last().unwrap()
}

fn main() {
    let ret: i64 = std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| trebuchet(line))
        .sum();

    println!("{}", ret);
}
