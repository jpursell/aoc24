use std::{collections::BTreeSet, str::FromStr};
use ndarray::prelude::*;

fn label_image(map: ArrayView2<char>) -> Array2<usize>{
    let mut out = Array2::zeros(map.raw_dim());
    let mut label = 0;
    let shape = out.shape();
    // init rows with 1 1d label images
    for irow in 0..shape[0] {
        label += 1;
        out[[irow, 0]] = label;
        for icol in 1..shape[1] {
            if map[[irow, icol]] != map[[irow, icol - 1]] {
                label += 1;
            }
            out[[irow, icol]] = label;
        }
    }
    // connect regions
    let mut regions = Vec::new();
    for icol = 0..shape[1] {
        for irow = 0..shape[0] {
            let connected_above = irow > 0 && map[[irow, icol]] == map[[irow-1, icol]];
            if !connected_above {
                continue;
            }
            todo!();
        }
    }
    out
}

struct Region {
    positions: BTreeSet<[usize; 2]>,
    perimeter: usize,
}
impl Region {
    fn new(map: ArrayView2<char>, start: [usize;2]) -> Self {
        let char = map[start];
        let mut positions = BTreeSet::from([start]);
        let mut perimeter = 0;

        Region{positions, perimeter}
    }
}


#[derive(Debug)]
struct Puzzle {
    map: Array2<char>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(nrows * ncols);
        for line in s.lines() {
            for c in line.chars() {
                map.push(c);
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        Ok(Puzzle { map })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        let mut covered = Array2::from_elem(self.map.raw_dim(), false);
        let mut out = 0;
        for (pos, _) in self.map.indexed_iter() {
            let pos = [pos.0, pos.1];
            if covered[pos] {
                continue;
            }
            let region = Region::new(self.map.view(), pos);
            for pos in &region.positions {
                assert!(!covered[*pos]);
                covered[*pos] = true;
            }
            out += region.positions.len() * region.perimeter;
        }
        out
    }
}

fn main() {
    let mut puzzle = include_str!("12.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let mut out = include_str!("12_test_a.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 140);
    }

    #[test]
    fn test_b() {
        let mut out = include_str!("12_test_b.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 772);
    }

    #[test]
    fn test_c() {
        let mut out = include_str!("12_test_c.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 1930);
    }
}
