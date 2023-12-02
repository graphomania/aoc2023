use std::fmt;
use std::num::ParseIntError;
use crate::Cube::{Blue, Green, Red};

#[derive(Debug, Clone)]
struct ParseError {
    what: String,
}

impl ParseError {
    pub fn from<T: ToString>(s: &T) -> ParseError {
        Self {
            what: s.to_string(),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(value: ParseIntError) -> Self {
        ParseError::from(&value.to_string())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error failed {}", self.what)
    }
}

enum Cube {
    Red(i64),
    Green(i64),
    Blue(i64),
}

impl Cube {
    pub fn from_str(s: &str) -> Result<Cube, ParseError> {
        let (num_str, name) = s.trim().split_once(" ").ok_or(ParseError::from(&"!(%d %s)"))?;
        let num = num_str.parse()?;
        match name {
            "red" => Ok(Red(num)),
            "green" => Ok(Green(num)),
            "blue" => Ok(Blue(num)),
            _ => Err(ParseError::from(&"unknown color"))
        }
    }
}

// Red, Green, Blue
type GameLimits = (i64, i64, i64);

struct Game(Vec<Cube>);

impl Game {
    pub fn from_str(s: &str) -> Result<Game, ParseError> {
        Ok(Game(s.split(", ").map(|cube_str| Cube::from_str(cube_str).unwrap()).collect()))
    }

    pub fn in_limits(&self, limits: &GameLimits) -> bool {
        for cube in &self.0 {
            match cube {
                Red(val) => {
                    if val > &limits.0 {
                        return false;
                    }
                }
                Green(val) => {
                    if val > &limits.1 {
                        return false;
                    }
                }
                Blue(val) => {
                    if val > &limits.2 {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn to_limits(&self) -> GameLimits {
        let mut limits = (0, 0, 0);
        for cube in &self.0 {
            match cube {
                Red(val) => {
                    limits.0 = *val;
                }
                Green(val) => {
                    limits.1 = *val;
                }
                Blue(val) => {
                    limits.2 = *val;
                }
            }
        }
        limits
    }
}

struct Series {
    pub id: i64,
    pub games: Vec<Game>,
}

impl Series {
    pub fn from_str(s: &str) -> Result<Self, ParseError> {
        if !s.starts_with("Game ") {
            return Err(ParseError::from(&"doesnt start from 'Game '"));
        }
        let colon = s.find(":");
        if colon.is_none() {
            return Err(ParseError::from(&"doesnt start from 'Game %d:'"));
        }

        Ok(Series {
            id: s[("Game ".len())..(colon.unwrap())].parse::<i64>()?,
            games: s[(colon.unwrap() + 1)..(s.len())].split(";").map(|game_str| Game::from_str(game_str).unwrap()).collect(),
        })
    }
}

fn sum_all_valid_games_ids(filename: &str, limits: &GameLimits) -> i64 {
    std::fs::read_to_string(filename).unwrap()
        .lines()
        .map(|line| Series::from_str(line).unwrap())
        .filter(|series| series.games.iter().all(|game| game.in_limits(&limits)))
        .map(|x| x.id)
        .sum()
}

fn sum_powers_of_the_tightest_limits(filename: &str) -> i64 {
    std::fs::read_to_string(filename).unwrap()
        .lines()
        .map(|line|
            {
                let mut limits = (0, 0, 0);
                for lim in Series::from_str(line).unwrap().games.iter().map(|game| game.to_limits()) {
                    limits.0 = limits.0.max(lim.0);
                    limits.1 = limits.1.max(lim.1);
                    limits.2 = limits.2.max(lim.2);
                }
                limits.0 * limits.1 * limits.2
            }
        )
        .sum()
}

// Game 1: 1 red, 5 blue, 10 green; 5 green, 6 blue, 12 red; 4 red, 10 blue, 4 green
fn main() {
    let limits = (12, 13, 14);
    println!("1: {:?}", sum_all_valid_games_ids("input.txt", &limits));
    println!("2: {:?}", sum_powers_of_the_tightest_limits("input.txt"));
}
