use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
struct Lock {
    heights: [usize; 5],
}
#[derive(Debug)]
struct Key {
    heights: [usize; 5],
}
impl Lock {
    fn overlap(&self, key: &Key) -> bool {
        self.heights
            .iter()
            .zip(key.heights.iter())
            .any(|(l, k)| l + k > 5)
    }
}

#[derive(Debug)]
struct Puzzle {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for lines in &s.lines().chunks(8) {
            let mut heights = [0; 5];
            let mut first_char = None;
            for line in lines {
                for (i, char) in line.chars().enumerate() {
                    if first_char.is_none() {
                        first_char = Some(char);
                    }
                    match char {
                        '.' => (),
                        '#' => {
                            heights[i] += 1;
                        }
                        _ => panic!(),
                    }
                }
            }
            heights.iter_mut().for_each(|x| {
                *x -= 1;
            });
            match first_char.unwrap() {
                '#' => locks.push(Lock { heights }),
                '.' => keys.push(Key { heights }),
                _ => panic!(),
            }
        }
        Ok(Puzzle { locks, keys })
    }
}

impl Puzzle {
    fn process(&self) -> usize {
        let mut out = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if !lock.overlap(key) {
                    out += 1;
                }
            }
        }
        out
    }
}

fn main() {
    let puzzle = include_str!("25.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 2854);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let out = include_str!("25_test.txt").parse::<Puzzle>().unwrap();
        // dbg!(&out);
        let out = out.process();
        assert_eq!(out, 3);
    }
}
