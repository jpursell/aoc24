use std::{collections::{btree_map::Entry, BTreeMap}, str::FromStr};

#[derive(Debug)]
struct Puzzle {
    shape: [usize; 2],
    antennas: BTreeMap<char, Vec<[usize; 2]>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let shape = [nrows, ncols];
        let mut antennas = BTreeMap::new();
        for (irow, line) in s.lines().enumerate() {
            for (icol, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                match antennas.entry(c) {
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![[irow, icol]]);
                    }
                    Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push([irow, icol]);
                    }
                }
            }
        }
        Ok(Puzzle { shape, antennas })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        let mut out = 0;
        out
    }
}

fn main() {
    let mut puzzle = include_str!("08.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut puzzle = include_str!("08_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&puzzle);
        let out = puzzle.process();
        assert_eq!(out, 14);
    }
}
