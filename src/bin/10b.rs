use ndarray::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
struct Puzzle {
    map: Array2<usize>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(nrows * ncols);
        for line in s.lines() {
            for c in line.chars() {
                map.push(c.to_digit(10).unwrap() as usize);
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        Ok(Puzzle { map })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        let mut walked = Array2::from_elem(self.map.raw_dim(), false);
        self.map
            .indexed_iter()
            .map(|(pos, start)| {
                if start != &0 {
                    0
                } else {
                    self.count_trails([pos.0, pos.1], *start, &mut walked)
                }
            })
            .sum()
    }
    fn count_trails(&self, pos: [usize; 2], value: usize, walked: &mut Array2<bool>) -> usize {
        // are we done?
        if value == 9 {
            return 1;
        }
        let mut count = 0;
        let mut check_new_pos = |new_pos| {
            if !walked[new_pos] {
                let new_value = self.map[new_pos];
                if new_value == value + 1 {
                    walked[new_pos] = true;
                    count += self.count_trails(new_pos, new_value, walked);
                    walked[new_pos] = false;
                }
            }
        };
        // try walking down
        if pos[0] + 1 < self.map.shape()[0] {
            let new_pos = [pos[0] + 1, pos[1]];
            check_new_pos(new_pos);
        }
        // try walking up
        if pos[0] > 0 {
            let new_pos = [pos[0] - 1, pos[1]];
            check_new_pos(new_pos);
        }
        // try walking right
        if pos[1] + 1 < self.map.shape()[1] {
            let new_pos = [pos[0], pos[1] + 1];
            check_new_pos(new_pos);
        }
        // try walking left
        if pos[1] > 0 {
            let new_pos = [pos[0], pos[1] - 1];
            check_new_pos(new_pos);
        }
        count
    }
}

fn main() {
    let mut puzzle = include_str!("10.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 1210);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut out = include_str!("10_test.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 81);
    }
}
