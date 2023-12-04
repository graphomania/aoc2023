use std::collections::{HashMap, HashSet};
use crate::day04::err::ParseError;

pub mod err;

pub type Result<T> = std::result::Result<T, ParseError>;

pub struct Scratchcard {
    pub id: i64,
    pub winning: HashSet<i64>,
    pub scratched: Vec<i64>,
}

impl Scratchcard {
    // Card 1: 41 48 83 86 17 | 83 86 6 31 17 9 48 53
    pub fn from_str(s: &str) -> Result<Scratchcard> {
        let (card, numbers) = s.trim().split_once(":").ok_or(ParseError::from(&"no ':'"))?;
        let (winning, scratched) = numbers.split_once("|").ok_or(ParseError::from(&"no '|'"))?;

        let mut winning_numbers = HashSet::new();
        for winning in winning.split(" ").filter(|&s| s != "") {
            winning_numbers.insert(winning.parse()?);
        }

        let mut scratched_numbers = Vec::new();
        for scratch in scratched.split(" ").filter(|&s| s != "") {
            scratched_numbers.push(scratch.parse()?);
        }

        Ok(Scratchcard {
            id: card.trim().trim_start_matches("Card").trim().parse()?,
            winning: winning_numbers,
            scratched: scratched_numbers,
        })
    }

    pub fn count_win(&self) -> usize {
        self.scratched.iter()
            .filter(|scratch| self.winning.contains(scratch))
            .count()
    }

    pub fn win(&self) -> i64 {
        (1 << self.count_win()) / 2
    }
}


/// Part A. Better handled parsing of data, than in the day 02.
/// Parse --> #{a in HashSet | a in Vec} --> Sum.
pub fn total_scratched_win(data: &str) -> Result<i64> {
    let mut sum = 0;

    for scratch in data.lines().map(|line| Scratchcard::from_str(line)) {
        let scratch = scratch?;
        sum += scratch.win();
    }

    Ok(sum)
}

/// Part B. Parse --> #{a in HashSet | a in Vec} --> Update HashMap --> Sum.
pub fn total_scratched_win_with_multiply(data: &str) -> Result<i64> {
    let mut sum = 0;

    let mut increases = HashMap::new();
    for (i, scratch) in data.lines().map(|line| Scratchcard::from_str(line)).enumerate() {
        let count = 1 + increases.get(&i).unwrap_or(&0);
        let scratched = scratch?.count_win();
        for delta in 1..=scratched {
            *increases.entry(i + delta).or_insert(0) += count;
        }
        sum += count;
    }

    Ok(sum as i64)
}


#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_TEST: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn basic_test_a() {
        assert_eq!(total_scratched_win(BASIC_TEST).expect("PARSING ERROR"), 13);
    }

    #[test]
    fn basic_test_b() {
        assert_eq!(total_scratched_win_with_multiply(BASIC_TEST).expect("PARSING ERROR"), 30);
    }

    #[test]
    fn answer() {
        let data = std::fs::read_to_string("src/day04/input.txt").unwrap();
        println!("1. {}", total_scratched_win(data.as_str()).unwrap());
        println!("2. {}", total_scratched_win_with_multiply(data.as_str()).unwrap());
    }
}