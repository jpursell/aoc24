use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    str::FromStr,
};

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
        let mut antinodes = BTreeSet::<[usize; 2]>::new();
        for antenna_positions in self.antennas.values() {
            for start in antenna_positions {
                for end in antenna_positions {
                    if start == end {
                        continue;
                    }
                    let start = start.map(|x| x as i64);
                    let end = end.map(|x| x as i64);
                    let diff = [end[0] - start[0], end[1] - start[1]];
                    let mut harmonic = 1;
                    loop {
                        let antinode =
                            [start[0] + diff[0] * harmonic, start[1] + diff[1] * harmonic];
                        if antinode[0] < 0 || antinode[1] < 0 {
                            break;
                        }
                        let antinode = antinode.map(|x| x as usize);
                        if antinode[0] >= self.shape[0] || antinode[1] >= self.shape[1] {
                            break;
                        }
                        antinodes.insert(antinode);
                        harmonic += 1;
                    }
                }
            }
        }
        antinodes.len()
    }
}

fn main() {
    let mut puzzle = include_str!("08.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 1235);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut puzzle = include_str!("08_test.txt").parse::<Puzzle>().unwrap();
        let out = puzzle.process();
        assert_eq!(out, 34);
    }
}
