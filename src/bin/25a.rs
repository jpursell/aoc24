use std::
    str::FromStr
;
use ndarray::prelude::*;

use itertools::Itertools;

#[derive(Debug)]
enum Schmatic {
    Key([usize;5]),
    Lock([usize;5]),
}

#[derive(Debug)]
struct Puzzle {
    schematics: Vec<Schmatic>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut schematics = Vec::new();
        for lines in &s.lines().chunks(8) {
            let mut totals = [0;5];
            let mut first_char = None;
            for line in lines {
                for (i, char) in line.chars().enumerate() {
                    if first_char.is_none() {
                        first_char = Some(char);
                    }
                    match char {
                        '.' => (),
                        '#' => {totals[i]+=1;}
                        _ => panic!(),
                    }
                }
            }
            totals.iter_mut().for_each(|x|{*x-=1;});
            match first_char.unwrap(){
                '#' => schematics.push(Schmatic::Lock(totals)),
                '.' => schematics.push(Schmatic::Key(totals)),
                _ => panic!(),
            }
        }
        Ok(Puzzle {
            schematics
        })
    }
}

impl Puzzle {
    fn process(&self) -> usize {
        0
    }
}

fn main() {
    let mut puzzle = include_str!("25.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let out = include_str!("25_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        // let out = out.process();
        // assert_eq!(out, );
    }
}
