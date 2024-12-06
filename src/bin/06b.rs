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
fn direction_index(direction: &Direction) -> usize {
    match direction {
        Direction::Down => 0,
        Direction::Up => 1,
        Direction::Left => 2,
        Direction::Right => 3,
    }
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
    position_direction: Array3<bool>,
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
        let position_direction = Array3::from_elem([nrows, ncols, 4], false);
        Ok(Puzzle {
            map,
            position,
            direction: Direction::Up,
            position_direction,
        })
    }
}

impl Puzzle {
    fn check_if_block_makes_loop(&mut self, block_location: [usize; 2]) -> bool {
        match self.map[block_location] {
            Token::Blocked => {
                panic!();
            }
            Token::Clear => {
                self.map[block_location] = Token::Blocked;
            }
        }
        let mut looping = false;
        self.position_direction.iter_mut().for_each(|x| {
            *x = false;
        });
        let mut position = self.position;
        let mut direction = self.direction;
        self.position_direction[[position[0], position[1], direction_index(&direction)]] = true;
        while let Some((new_position, new_direction)) = self.update(position, direction) {
            position = new_position;
            direction = new_direction;
            let pd_index = [position[0], position[1], direction_index(&direction)];
            if self.position_direction[pd_index] {
                looping = true;
                break;
            }
            self.position_direction[pd_index] = true;
        }
        self.map[block_location] = Token::Clear;
        looping
    }
    fn process(&mut self) -> usize {
        let positions = self.find_original_locations();
        let mut out = 0;
        for position in positions {
            if self.check_if_block_makes_loop(position) {
                out += 1;
            }
        }
        out
    }
    fn find_original_locations(&self) -> BTreeSet<[usize; 2]> {
        let mut positions = BTreeSet::new();
        let mut position = self.position;
        let mut direction = self.direction;
        positions.insert(position);
        while let Some((new_position, new_direction)) = self.update(position, direction) {
            position = new_position;
            direction = new_direction;
            positions.insert(position);
        }
        positions
    }
    /// Return new position if still on Map
    /// If guard can walk forward, that's what it will do
    /// Otherwise turn right and walk forward
    /// If guard walks off map return None
    fn update(
        &self,
        position: [usize; 2],
        direction: Direction,
    ) -> Option<([usize; 2], Direction)> {
        let shape = self.map.shape();
        // try to get forward position
        let mut new_position = update_position(position, direction, shape)?;
        // if we did get a forward position, check to see if it's blocked
        let mut blocked = true;
        if matches!(self.map[new_position], Token::Clear) {
            blocked = false;
        }
        // turn right if we need to
        let mut new_direction = direction;
        if blocked {
            new_direction = rotate_right(&direction);
            new_position = update_position(position, new_direction, shape)?;
        }
        Some((new_position, new_direction))
    }
}

fn main() {
    let mut puzzle = include_str!("06.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert!(out > 2124);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut puzzle = include_str!("06_test.txt").parse::<Puzzle>().unwrap();
        let out = puzzle.process();
        assert_eq!(out, 6);
    }
}
