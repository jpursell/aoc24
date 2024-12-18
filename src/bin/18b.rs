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
    map_block: usize,
    step_map: Array2<Option<usize>>,
    steps: usize,
}

impl Puzzle {
    fn add_block(&self, state: &mut State, index: usize) {
        let block = &self.blocks[index];
        let pos = [block[1], block[0]];
        state.map[pos] = Token::Wall;
    }
    fn remove_block(&self, state: &mut State, index: usize) {
        let block = &self.blocks[index];
        let pos = [block[1], block[0]];
        state.map[pos] = Token::None;
    }
    fn adjust_map(&self, state: &mut State, target: usize) {
        while state.map_block != target {
            if state.map_block < target {
                state.map_block += 1;
                self.add_block(state, state.map_block);
            } else {
                self.remove_block(state, state.map_block);
                state.map_block -= 1;
            }
        }
    }
    fn process(&self, shape: [usize; 2]) -> String {
        let map = Array2::from_elem(shape, Token::None);
        let step_map = Array2::from_elem(map.raw_dim(), None);
        let mut state = State {
            map,
            map_block: 0,
            step_map,
            steps: 0,
        };
        let mut bounds = [0, self.blocks.len() - 1];
        loop {
            let new_try_n_blocks = (bounds[0] + bounds[1]) / 2;
            self.adjust_map(&mut state, new_try_n_blocks);
            self.solve_maze(&mut state);
            let solved = state.step_map[[shape[0] - 1, shape[1] - 1]].is_some();
            if solved {
                bounds[0] = new_try_n_blocks;
            } else {
                bounds[1] = new_try_n_blocks;
            }
            if bounds[1] == bounds[0] + 1 {
                break;
            }
        }
        self.adjust_map(&mut state, bounds[1]);
        self.print_path(&state);
        let out = self.blocks[bounds[1]];
        format!("{},{}", out[0], out[1])
    }
    fn solve_maze(&self, state: &mut State) {
        // reset solution
        state.steps = 0;
        state.step_map.iter_mut().for_each(|x| *x = None);
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
    let out = puzzle.process([71, 71]);
    println!("{out}");
    assert_eq!(out, "20,44");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        let out = include_str!("18_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process([7, 7]);
        assert_eq!(out, "6,1");
    }
}
