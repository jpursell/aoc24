use ndarray::prelude::*;
use std::{cmp::Ordering, collections::BTreeSet, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Token {
    Wall,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Move {
    Forward,
    Clockwise,
    CounterClockwise,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}
impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!(),
        }
    }
}
// impl Direction {
//     fn rotate_clockwise(&self) -> Self {
//         let val: usize = (*self).into();
//         (val + 1).into()
//     }
//     fn rotate_counter_clockwise(&self) -> Self {
//         let val: usize = (*self).into();
//         (val - 1).into()
//     }
// }
impl Move {
    fn position_from(&self, position: &[usize; 3]) -> Option<[usize; 3]> {
        match self {
            Move::Forward => {
                let direction = Direction::from(position[2]);
                if matches!(direction, Direction::Up) && position[0] == 0
                    || matches!(direction, Direction::Left) && position[1] == 0
                {
                    None
                } else {
                    match direction {
                        Direction::Up => Some([position[0] - 1, position[1], position[2]]),
                        Direction::Down => Some([position[0] + 1, position[1], position[2]]),
                        Direction::Left => Some([position[0], position[1] - 1, position[2]]),
                        Direction::Right => Some([position[0], position[1] + 1, position[2]]),
                    }
                }
            }
            Move::Clockwise => Some([position[0], position[1], (position[2] + 1) % 4]),
            Move::CounterClockwise => {
                if position[2] == 0 {
                    Some([position[0], position[1], 3])
                } else {
                    Some([position[0], position[1], position[2] - 1])
                }
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

const MOVES: [Move; 3] = [Move::Clockwise, Move::CounterClockwise, Move::Forward];

struct State {
    scores: Array3<Option<usize>>,
    previous: Array3<Vec<[usize; 3]>>,
    steps: usize,
}

impl Puzzle {
    fn process(&self, print: bool) -> usize {
        let map_shape = self.map.shape();
        let records_shape = [map_shape[0], map_shape[1], 4];
        let mut state = State {
            scores: Array3::<Option<usize>>::from_elem(records_shape, None),
            previous: Array3::<Vec<[usize; 3]>>::from_elem(records_shape, Vec::with_capacity(3)),
            steps: 0,
        };
        let start = [self.start[0], self.start[1], Direction::Right.into()];
        state.scores[start] = Some(0);
        let mut ends = [vec![start], Vec::new()];
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
                next.append(&mut self.process_pos(*pos, &mut state));
            }
            state.steps += 1;
        }
        if print {
            self.print_path(&state);
        }
        self.trace_back(&state, &self.best_end_position(&state).unwrap())
            .iter()
            .map(|x| [x[0], x[1]])
            .collect::<BTreeSet<[usize; 2]>>()
            .len()
    }
    fn best_end_position(&self, state: &State) -> Option<[usize; 3]> {
        let mut lowest_score = None;
        let mut best_position = None;
        for i in 0..4 {
            let pos = [self.end[0], self.end[1], i];
            if let Some(score) = state.scores[pos] {
                if let Some(best) = lowest_score {
                    if score < best {
                        lowest_score = Some(score);
                        best_position = Some(pos);
                    }
                } else {
                    lowest_score = Some(score);
                    best_position = Some(pos);
                }
            }
        }
        best_position
    }
    fn trace_back(&self, state: &State, pos: &[usize; 3]) -> BTreeSet<[usize; 3]> {
        let mut out = BTreeSet::from([*pos]);
        if pos[0] == self.start[0] && pos[1] == self.start[1] {
            return out;
        }
        for previous in &state.previous[*pos] {
            out.append(&mut self.trace_back(state, previous));
        }
        out
    }
    fn process_pos(&self, pos: [usize; 3], state: &mut State) -> Vec<[usize; 3]> {
        let mut next_ends = Vec::new();
        for (next_move, next_position) in self.find_possible_next_positions(&pos) {
            // let current_directions = state.directions[*pos]
            let next_score = if matches!(next_move, Move::Forward) {
                state.scores[pos].unwrap() + 1
            } else {
                state.scores[pos].unwrap() + 1000
            };
            if let Some(existing_score) = state.scores[next_position] {
                match next_score.cmp(&existing_score) {
                    Ordering::Less => {
                        // println!(
                        //     "DEBUG: order less: {:?}->{:?} {} < {}",
                        //     pos, next_position, next_score, existing_score
                        // );
                        next_ends.push(next_position);
                        state.scores[next_position] = Some(next_score);
                        state.previous[next_position].clear();
                        state.previous[next_position].push(pos);
                    }
                    Ordering::Equal => {
                        // println!(
                        //     "DEBUG: order equal: {:?}->{:?} {} == {}",
                        //     pos, next_position, next_score, existing_score
                        // );
                        state.previous[next_position].push(pos);
                    }
                    Ordering::Greater => {
                        // println!(
                        //     "DEBUG: order greater: {:?}->{:?} {} > {}",
                        //     pos, next_position, next_score, existing_score
                        // );
                    }
                }
            } else {
                // println!(
                //     "DEBUG: new item: {:?}->{:?} {}",
                //     pos, next_position, next_score
                // );
                next_ends.push(next_position);
                state.scores[next_position] = Some(next_score);
                state.previous[next_position].push(pos);
            }
        }
        next_ends
    }
    fn find_possible_next_positions(&self, pos: &[usize; 3]) -> Vec<(Move, [usize; 3])> {
        let mut possible_positions = Vec::new();
        for next_move in MOVES {
            let next_position = next_move.position_from(pos);
            if next_position.is_none() {
                continue;
            }
            let next_position = next_position.unwrap();
            let next_position_2d = [next_position[0], next_position[1]];
            let next_token = self.map.get(next_position_2d);
            if next_token.is_none() {
                continue;
            }
            if matches!(next_token.unwrap(), Token::Wall) {
                continue;
            }
            possible_positions.push((next_move, next_position));
        }
        possible_positions
    }
    fn print_path(&self, state: &State) {
        let path = self
            .trace_back(state, &self.best_end_position(state).unwrap())
            .iter()
            .map(|x| [x[0], x[1]])
            .collect::<BTreeSet<[usize; 2]>>();
        // let score = state.scores[self.end];
        println!();
        // if score.is_none() {
        //     println!("Score: None");
        // } else {
        //     println!("Score: {}", score.unwrap());
        // }
        let mut current_row = 0;
        for (pos, token) in self.map.indexed_iter() {
            let pos = [pos.0, pos.1];
            if current_row != pos[0] {
                println!();
                current_row = pos[0];
            }
            if path.contains(&pos) {
                print!("O");
            } else {
                print!(
                    "{}",
                    match token {
                        Token::None => ".",
                        Token::Wall => "#",
                    }
                );
            }
        }
        println!();
    }
}

fn main() {
    let puzzle = include_str!("16.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(false);
    println!("{out}");
    assert_eq!(out, 433);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_a() {
        let out = include_str!("16_test_a.txt").parse::<Puzzle>().unwrap();
        let out = out.process(true);
        assert_eq!(out, 45);
    }
    #[test]
    fn test_b() {
        let out = include_str!("16_test_b.txt").parse::<Puzzle>().unwrap();
        let out = out.process(false);
        assert_eq!(out, 64);
    }
}
