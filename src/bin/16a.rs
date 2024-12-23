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

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

struct State {
    scores: Array2<Option<usize>>,
    directions: Array2<Option<Direction>>,
    steps: usize,
}

impl Puzzle {
    fn process(&self, print: bool) -> usize {
        let mut state = State {
            scores: Array2::<Option<usize>>::from_elem(self.map.raw_dim(), None),
            directions: Array2::<Option<Direction>>::from_elem(self.map.raw_dim(), None),
            steps: 0,
        };
        state.scores[self.start] = Some(0);
        state.directions[self.start] = Some(Direction::Right);
        let mut ends = [vec![self.start], Vec::new()];
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
                next.append(&mut self.process_pos(pos, &mut state));
            }
            state.steps += 1;
        }
        if print {
            self.print_path(&state);
        }
        state.scores[self.end].unwrap()
    }
    fn process_pos(&self, pos: &[usize; 2], state: &mut State) -> Vec<[usize; 2]> {
        let mut next_ends = Vec::new();
        for next_direction in self.find_possible_dirs(pos) {
            let next_position = next_direction.position_from(pos).unwrap();
            let next_score = if next_direction == state.directions[*pos].unwrap() {
                state.scores[*pos].unwrap() + 1
            } else {
                state.scores[*pos].unwrap() + 1001
            };
            if let Some(score) = state.scores[next_position] {
                if next_score < score {
                    next_ends.push(next_position);
                    state.scores[next_position] = Some(next_score);
                    state.directions[next_position] = Some(next_direction);
                }
            } else {
                next_ends.push(next_position);
                state.scores[next_position] = Some(next_score);
                state.directions[next_position] = Some(next_direction);
            }
        }
        next_ends
    }
    fn find_possible_dirs(&self, pos: &[usize; 2]) -> Vec<Direction> {
        let mut possible_directions = Vec::new();
        for next_direction in DIRECTIONS {
            let next_position = next_direction.position_from(pos);
            if next_position.is_none() {
                continue;
            }
            let next_position = next_position.unwrap();
            let next_token = self.map.get(next_position);
            if next_token.is_none() {
                continue;
            }
            if matches!(next_token.unwrap(), Token::Wall) {
                continue;
            }
            possible_directions.push(next_direction);
        }
        possible_directions
    }
    fn print_path(&self, state: &State) {
        let score = state.scores[self.end];
        println!();
        if score.is_none() {
            println!("Score: None");
        } else {
            println!("Score: {}", score.unwrap());
        }
        let mut current_row = 0;
        for (pos, token) in self.map.indexed_iter() {
            let pos = [pos.0, pos.1];
            if current_row != pos[0] {
                println!();
                current_row = pos[0];
            }
            print!(
                "{}",
                if let Some(direction) = state.directions[pos] {
                    match direction {
                        Direction::Down => "v",
                        Direction::Up => "^",
                        Direction::Left => "<",
                        Direction::Right => ">",
                    }
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
    assert_eq!(out, 66404);
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
