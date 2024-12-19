use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

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
        let mut out = 0;
        for needed in &self.needed {
            if self.min_towels(needed).is_some() {
                out += 1;
            }
        }
        out
    }
    fn min_towels(&self, needed: &[Token]) -> Option<usize> {
        let mut steps = 0;
        let mut ends = [BTreeSet::from([0]), BTreeSet::new()];
        loop {
            let (e0, e1) = ends.split_at_mut(1);
            let (current, next) = if steps % 2 == 0 {
                (&e0[0], &mut e1[0])
            } else {
                (&e1[0], &mut e0[0])
            };
            next.clear();
            for &i in current {
                for len in 1..=self.max_available_len {
                    if i + len > needed.len() {
                        continue;
                    }
                    if !self.available.contains(&needed[i..i + len]) {
                        continue;
                    }
                    if i + len == needed.len() {
                        return Some(steps + 1);
                    }
                    next.insert(i + len);
                }
            }
            steps += 1;
            if next.is_empty() {
                break;
            }
        }
        None
    }
}

fn main() {
    let puzzle = include_str!("19.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 360);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let out = include_str!("19_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process();
        assert_eq!(out, 6);
    }
}
