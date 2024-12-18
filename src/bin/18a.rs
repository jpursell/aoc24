use ndarray::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Token {
    Wall,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    blocks: Vec<[usize; 2]>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::with_capacity(s.lines().count());
        for line in s.lines() {
            let (x, y) = line.split_once(",").unwrap();
            blocks.push([x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()])
        }
        Ok(Puzzle { blocks })
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

struct State {
    map: Array2<Token>,
    step_map: Array2<Option<usize>>,
    steps: usize,
}

impl Puzzle {
    fn make_map(&self, shape: [usize; 2], falls: usize) -> Array2<Token> {
        let mut map = Array2::from_elem(shape, Token::None);
        for block in &self.blocks[0..falls] {
            let pos = [block[1], block[0]];
            map[pos] = Token::Wall;
        }
        map
    }
    fn process(&self, shape: [usize; 2], falls: usize) -> usize {
        let map = self.make_map(shape, falls);
        let step_map = Array2::from_elem(map.raw_dim(), None);
        let mut state = State {
            map,
            step_map,
            steps: 0,
        };
        self.solve_maze(&mut state);
        self.print_path(&state);
        state.step_map[[shape[0] - 1, shape[1] - 1]].unwrap()
    }
    fn solve_maze(&self, state: &mut State) {
        let mut ends = [vec![[0, 0]], Vec::new()];
        loop {
            let (e0, e1) = ends.split_at_mut(1);
            let (current, next) = if state.steps % 2 == 0 {
                (&e0[0], &mut e1[0])
            } else {
                (&e1[0], &mut e0[0])
            };
            next.clear();
            if current.is_empty() {
                break;
            }
            for pos in current {
                next.append(&mut self.process_pos(pos, state));
            }
            state.steps += 1;
        }
    }
    fn process_pos(&self, pos: &[usize; 2], state: &mut State) -> Vec<[usize; 2]> {
        let mut next_ends = Vec::new();
        for next_direction in self.find_possible_dirs(pos, state) {
            let next_position = next_direction.position_from(pos).unwrap();
            state.step_map[next_position] = Some(state.steps + 1);
            next_ends.push(next_position);
        }
        next_ends
    }
    fn find_possible_dirs(&self, pos: &[usize; 2], state: &State) -> Vec<Direction> {
        let mut possible_directions = Vec::with_capacity(3);
        for next_direction in DIRECTIONS {
            let next_position = next_direction.position_from(pos);
            if next_position.is_none() {
                continue;
            }
            let next_position = next_position.unwrap();
            let next_token = state.map.get(next_position);
            if next_token.is_none() {
                continue;
            }
            if matches!(next_token.unwrap(), Token::Wall) {
                continue;
            }
            if state.step_map[next_position].is_some() {
                continue;
            }
            possible_directions.push(next_direction);
        }
        possible_directions
    }
    fn print_path(&self, state: &State) {
        println!();
        let mut current_row = 0;
        for (pos, token) in state.map.indexed_iter() {
            let pos = [pos.0, pos.1];
            if current_row != pos[0] {
                println!();
                current_row = pos[0];
            }
            print!(
                "{}",
                match token {
                    Token::None => ".",
                    Token::Wall => "#",
                }
            );
        }
        println!();
    }
}

fn main() {
    let puzzle = include_str!("18.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process([71, 71], 1024);
    println!("{out}");
    assert_eq!(out, 338);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        let out = include_str!("18_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process([7, 7], 12);
        assert_eq!(out, 22);
    }
}
