use ndarray::prelude::*;
use std::{collections::BTreeSet, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Token {
    Wall,
    None,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn position_from(&self, position: &[usize; 2]) -> Option<[usize; 2]> {
        if matches!(self, Direction::Up) && position[0] == 0
            || matches!(self, Direction::Left) && position[1] == 0
        {
            None
        } else {
            match self {
                Direction::Up => Some([position[0] - 1, position[1]]),
                Direction::Down => Some([position[0] + 1, position[1]]),
                Direction::Left => Some([position[0], position[1] - 1]),
                Direction::Right => Some([position[0], position[1] + 1]),
            }
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    map: Array2<Token>,
    start: [usize; 2],
    end: [usize; 2],
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(nrows * ncols);
        let mut start = [0, 0];
        let mut end = [0, 0];
        for (irow, line) in s.lines().enumerate() {
            for (icol, c) in line.chars().enumerate() {
                map.push(match c {
                    '.' => Token::None,
                    '#' => Token::Wall,
                    'S' => {
                        start = [irow, icol];
                        Token::None
                    }
                    'E' => {
                        end = [irow, icol];
                        Token::None
                    }
                    _ => panic!(),
                });
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        Ok(Puzzle { map, start, end })
    }
}

impl Puzzle {
    fn process(&self, print: bool) -> usize {
        let direction = Direction::Right;
        let mut positions_set = BTreeSet::<[usize; 2]>::new();
        positions_set.insert(self.start);
        let mut positions_vec = vec![self.start];
        self.find_lowest_score(direction, &mut positions_set, &mut positions_vec, 0, print)
            .unwrap()
    }
    fn find_lowest_score(
        &self,
        current_direction: Direction,
        positions_set: &mut BTreeSet<[usize; 2]>,
        positions_vec: &mut Vec<[usize; 2]>,
        current_score: usize,
        print: bool,
    ) -> Option<usize> {
        let mut min_score = None;
        for next_direction in [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ] {
            {
                let next_position = next_direction.position_from(positions_vec.last().unwrap());
                if next_position.is_none() {
                    continue;
                }
                let next_position = next_position.unwrap();
                if positions_set.contains(&next_position) {
                    continue;
                }
                let next_token = self.map.get(next_position);
                if next_token.is_none() {
                    continue;
                }
                if matches!(next_token.unwrap(), Token::Wall) {
                    continue;
                }
                positions_vec.push(next_position);
                positions_set.insert(next_position);
            }
            // we have a valid move at this point to we need to update our state
            let next_score = if next_direction == current_direction {
                current_score + 1
            } else {
                current_score + 1001
            };
            if positions_vec.last().unwrap() == &self.end {
                if print {
                    println!();
                    println!("Score: {}", next_score);
                    self.print_path(positions_set);
                }
                let pos = positions_vec.pop().unwrap();
                positions_set.remove(&pos);
                return Some(next_score);
            }
            if let Some(next_min_score) = self.find_lowest_score(
                next_direction,
                positions_set,
                positions_vec,
                next_score,
                print,
            ) {
                if min_score.is_none() || next_min_score < min_score.unwrap() {
                    min_score = Some(next_min_score);
                }
            }
            let pos = positions_vec.pop().unwrap();
            positions_set.remove(&pos);
        }
        min_score
    }
    fn print_path(&self, positions: &BTreeSet<[usize; 2]>) {
        let mut current_row = 0;
        for (pos, token) in self.map.indexed_iter() {
            let pos = [pos.0, pos.1];
            if current_row != pos[0] {
                println!();
                current_row = pos[0];
            }
            print!(
                "{}",
                if positions.contains(&pos) {
                    "O"
                } else {
                    match token {
                        Token::None => ".",
                        Token::Wall => "#",
                    }
                }
            );
        }
        println!();
    }
}

fn main() {
    let puzzle = include_str!("16.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(false);
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        let out = include_str!("16_test_a.txt").parse::<Puzzle>().unwrap();
        let out = out.process(true);
        assert_eq!(out, 7036);
    }
    #[test]
    fn test_b() {
        let out = include_str!("16_test_b.txt").parse::<Puzzle>().unwrap();
        let out = out.process(false);
        assert_eq!(out, 11048);
    }
}
