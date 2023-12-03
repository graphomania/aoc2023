use std::collections::{HashSet};

fn is_adjacent_to_symbol(scheme: &Vec<Vec<char>>, i: isize, j: isize, len: isize) -> bool {
    for i in ((i - 1).max(0))..((i + 2).min(scheme.len() as isize)) {
        for j in ((j - 1).max(0))..((j + 1 + len).min(scheme[0].len() as isize)) {
            if !scheme[i as usize][j as usize].is_digit(10) && scheme[i as usize][j as usize] != '.' {
                return true;
            }
        }
    }
    return false;
}

/// Part A. Getting numbers --> checking for adjacent symbols.
/// Too much type conversions. I don't like this solution. The part B is pretty good, IMO.
pub fn gear_ratios(data: &str) -> i64 {
    let scheme: Vec<Vec<_>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut sum = 0;
    for i in 0..(scheme.len()) {
        let mut digit_j_opt = None;
        let mut digit_len = 0;
        for j in 0..(scheme.len() + 1) {
            if j != scheme.len() && scheme[i][j].is_digit(10) {
                digit_j_opt.get_or_insert(j);
                digit_len += 1;
                continue;
            }

            if let Some(digit_j) = digit_j_opt {
                if is_adjacent_to_symbol(&scheme, i as isize, digit_j as isize, digit_len) {
                    let mut num = 0;
                    for j in digit_j..(digit_j + digit_len as usize) {
                        num = num * 10 + scheme[i][j].to_digit(10).unwrap();
                    }
                    sum += num as i64;
                }
                digit_j_opt = None;
                digit_len = 0;
            }
        }
    }
    sum
}


fn adjacent_numbers(scheme: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<i64> {
    if scheme.len() == 0 || scheme[0].len() == 0 {
        return Vec::new();
    }

    let mut adj_numbers = HashSet::new();
    for i in (1.max(i) - 1)..=((i + 1).min(scheme.len() - 1)) {
        for j in (1.max(j) - 1)..=((j + 1).min(scheme[i].len() - 1)) {
            if scheme[i][j].is_digit(10) {
                let mut num_start = j;
                loop {
                    if !scheme[i][num_start].is_digit(10) {
                        num_start += 1;
                        break;
                    }
                    if num_start == 0 {
                        break;
                    }
                    num_start -= 1;
                }

                let mut num_end = j;
                for j in j..(scheme[i].len() - 1) {
                    if !scheme[i][j + 1].is_digit(10) {
                        break;
                    }
                    num_end += 1;
                }

                adj_numbers.insert((i, (num_start, num_end + 1)));
            }
        }
    }


    return adj_numbers.iter().copied().map(|(i, (start, end))| {
        let mut num = 0;
        for j in start..end {
            num = num * 10 + scheme[i][j].to_digit(10).unwrap();
        }
        num as i64
    }).collect();
}

/// Part B. Getting stars --> checking for adjacent digits --> converting to numbers.
pub fn real_gear_ratios(data: &str) -> i64 {
    let mut sum = 0;
    let scheme: Vec<Vec<_>> = data.lines().map(|line| line.chars().collect()).collect();
    for i in 0..(scheme.len()) {
        for j in 0..(scheme[i].len()) {
            if scheme[i][j] == '*' {
                let adj_numbers = adjacent_numbers(&scheme, i, j);
                if adj_numbers.len() == 2 {
                    sum += adj_numbers[0] * adj_numbers[1];
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA_BASIC: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";


    #[test]
    fn test_basic() {
        assert_eq!(gear_ratios(TESTDATA_BASIC), 4361);
    }

    #[test]
    fn test_basic_real() {
        assert_eq!(real_gear_ratios(TESTDATA_BASIC), 467835);
    }

    #[test]
    fn test_answer() {
        println!("1. {}", gear_ratios(std::fs::read_to_string("src/day03/input.txt").unwrap().as_str()));
        println!("2. {}", real_gear_ratios(std::fs::read_to_string("src/day03/input.txt").unwrap().as_str()));
    }
}