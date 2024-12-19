use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Token {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl From<char> for Token {
    fn from(value: char) -> Self {
        match value {
            'w' => Token::White,
            'u' => Token::Blue,
            'b' => Token::Black,
            'r' => Token::Red,
            'g' => Token::Green,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    available: HashSet<Vec<Token>>,
    max_available_len: usize,
    needed: Vec<Vec<Token>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let available: HashSet<Vec<Token>> = lines[0]
            .split(", ")
            .map(|s| s.chars().map(Token::from).collect())
            .collect();
        let max_available_len = available
            .iter()
            .map(|a| a.len())
            .fold(0, |a, len| a.max(len));
        let needed = lines[2..]
            .iter()
            .map(|line| line.chars().map(Token::from).collect())
            .collect();
        Ok(Puzzle {
            available,
            max_available_len,
            needed,
        })
    }
}

impl Puzzle {
    fn process(&self) -> usize {
        self.needed
            .iter()
            .map(|needed| self.count_solutions(needed))
            .sum()
    }
    fn count_solutions(&self, needed: &[Token]) -> usize {
        let mut count_map = vec![0_usize; needed.len() + 1];
        count_map[0] = 1;
        for i in 0..needed.len() {
            let count = count_map[i];
            if count == 0 {
                continue;
            }
            for len in 1..=self.max_available_len {
                if i + len > needed.len() {
                    continue;
                }
                if !self.available.contains(&needed[i..i + len]) {
                    continue;
                }
                count_map[i + len] += count;
            }
        }
        *count_map.last().unwrap()
    }
}

fn main() {
    let puzzle = include_str!("19.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let out = include_str!("19_test.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 16);
    }
}
