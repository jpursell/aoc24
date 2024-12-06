use ndarray::prelude::*;
use std::{collections::BTreeSet, str::FromStr};

#[derive(Debug)]
enum Token {
    Clear,
    Blocked,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}
fn position_change(direction: &Direction) -> [i64; 2] {
    match direction {
        Direction::Left => [0, -1],
        Direction::Right => [0, 1],
        Direction::Up => [-1, 0],
        Direction::Down => [1, 0],
    }
}
fn rotate_right(direction: &Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down,
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
    }
}
fn update_position(
    position: [usize; 2],
    direction: Direction,
    shape: &[usize],
) -> Option<[usize; 2]> {
    let position_change = position_change(&direction);
    let position = [position[0] as i64, position[1] as i64];
    let new_position = [
        position[0] + position_change[0],
        position[1] + position_change[1],
    ];
    if new_position[0] < 0 || new_position[1] < 0 {
        return None;
    }
    let new_position = [new_position[0] as usize, new_position[1] as usize];
    if new_position[0] >= shape[0] || new_position[1] >= shape[1] {
        return None;
    }
    Some(new_position)
}

#[derive(Debug)]
struct Puzzle {
    map: Array2<Token>,
    position: [usize; 2],
    direction: Direction,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nrows = s.lines().count();
        let ncols = s.lines().next().unwrap().chars().count();
        let mut tokens = Vec::with_capacity(nrows * ncols);
        let mut position = [0, 0];
        for (irow, line) in s.lines().enumerate() {
            for (icol, char) in line.chars().enumerate() {
                let token = match char {
                    '.' => Some(Token::Clear),
                    '#' => Some(Token::Blocked),
                    '^' => {
                        position = [irow, icol];
                        Some(Token::Clear)
                    }
                    _ => None,
                };
                if token.is_none() {
                    return Err(());
                }
                tokens.push(token.unwrap());
            }
        }
        let map = Array2::<Token>::from_shape_vec((nrows, ncols), tokens).unwrap();
        Ok(Puzzle {
            map,
            position,
            direction: Direction::Up,
        })
    }
}

impl Puzzle {
    fn process(&self) -> usize {
        let mut positions = BTreeSet::new();
        let mut position = self.position;
        let mut direction = self.direction;
        positions.insert(position);
        let mut position_directions = BTreeSet::new();
        position_directions.insert((position, direction));
        loop {
            let (position, direction) = self.update(position, direction);
            if position_directions.contains(&(position, direction)) {
                break;
            }
            positions.insert(position);
            position_directions.insert((position, direction));
        }
        positions.len()
    }
    fn update(&self, position: [usize; 2], direction: Direction) -> ([usize; 2], Direction) {
        let shape = self.map.shape();
        let new_position = update_position(position, direction, shape);
        todo!();
    }
}

fn main() {
    let puzzle = include_str!("06_test.txt").parse::<Puzzle>().unwrap();
    dbg!(&puzzle);
    let out = puzzle.process();
    assert_eq!(out, 41);
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1)
    }
}
