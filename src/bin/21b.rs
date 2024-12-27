use memoize::memoize;
use ndarray::prelude::*;
use std::{collections::BTreeMap, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumericButton {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DirectionalButton {
    Up,
    Right,
    Down,
    Left,
    Activate,
}

impl From<char> for NumericButton {
    fn from(value: char) -> Self {
        match value {
            '0' => NumericButton::Zero,
            '1' => NumericButton::One,
            '2' => NumericButton::Two,
            '3' => NumericButton::Three,
            '4' => NumericButton::Four,
            '5' => NumericButton::Five,
            '6' => NumericButton::Six,
            '7' => NumericButton::Seven,
            '8' => NumericButton::Eight,
            '9' => NumericButton::Nine,
            'A' => NumericButton::Activate,
            _ => panic!(),
        }
    }
}

impl From<usize> for NumericButton {
    fn from(value: usize) -> Self {
        match value {
            0 => NumericButton::Zero,
            1 => NumericButton::One,
            2 => NumericButton::Two,
            3 => NumericButton::Three,
            4 => NumericButton::Four,
            5 => NumericButton::Five,
            6 => NumericButton::Six,
            7 => NumericButton::Seven,
            8 => NumericButton::Eight,
            9 => NumericButton::Nine,
            10 => NumericButton::Activate,
            _ => panic!(),
        }
    }
}

const NUMERIC_BUTTONS: [NumericButton; 11] = [
    NumericButton::Zero,
    NumericButton::One,
    NumericButton::Two,
    NumericButton::Three,
    NumericButton::Four,
    NumericButton::Five,
    NumericButton::Six,
    NumericButton::Seven,
    NumericButton::Eight,
    NumericButton::Nine,
    NumericButton::Activate,
];

pub const DIRECTIONAL_BUTTONS: [DirectionalButton; 5] = [
    DirectionalButton::Activate,
    DirectionalButton::Up,
    DirectionalButton::Right,
    DirectionalButton::Down,
    DirectionalButton::Left,
];

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

struct PathSolver<T> {
    layout: Array2<Option<T>>,
    seen: Array2<bool>,
    end: [usize; 2],
}
impl<T> PathSolver<T>
where
    T: Eq + Copy,
{
    fn new(layout: Array2<Option<T>>) -> Self {
        let seen = Array2::from_elem(layout.raw_dim(), false);
        PathSolver {
            seen,
            layout,
            end: [0, 0],
        }
    }
    fn find_button(&self, button_to_find: &T) -> Option<[usize; 2]> {
        for (pos, button) in self.layout.indexed_iter() {
            if button == &Some(*button_to_find) {
                return Some([pos.0, pos.1]);
            }
        }
        None
    }
    fn shortest_paths(&mut self, start: &T, end: &T) -> Vec<Vec<Direction>> {
        if start == end {
            return vec![vec![]];
        }
        let start = self.find_button(start).unwrap();
        self.end = self.find_button(end).unwrap();
        let pos = start;
        self.seen.iter_mut().for_each(|x| *x = false);
        self.seen[start] = true;
        let mut potential_paths = Vec::new();
        for direction in DIRECTIONS {
            potential_paths.push(self.get_paths(pos, vec![direction]));
        }
        Self::combine_paths(potential_paths).unwrap()
    }
    fn combine_paths(mut paths: Vec<Option<Vec<Vec<Direction>>>>) -> Option<Vec<Vec<Direction>>> {
        if !paths.iter().any(|x| x.is_some()) {
            return None;
        }
        let paths: Vec<&mut Vec<Vec<Direction>>> =
            paths.iter_mut().filter_map(|x| x.as_mut()).collect();
        let shortest_len = paths.iter().map(|x| x[0].len()).min().unwrap();
        let mut out: Vec<Vec<Direction>> = Vec::new();

        for p in paths {
            if p[0].len() != shortest_len {
                continue;
            }
            out.append(p);
        }
        Some(out)
    }
    fn get_paths(&mut self, pos: [usize; 2], path: Vec<Direction>) -> Option<Vec<Vec<Direction>>> {
        let direction = path.last().unwrap();
        let next_pos = direction.position_from(pos)?;
        {
            let next_token = self.layout.get(next_pos)?;
            if next_token.is_none() {
                return None;
            }
            let next_seen = self.seen.get_mut(next_pos)?;
            if *next_seen {
                return None;
            }
            if next_pos == self.end {
                return Some(vec![path]);
            }
            *next_seen = true;
        }
        let mut potential_paths = Vec::new();
        for direction in DIRECTIONS {
            let mut new_path = path.clone();
            new_path.push(direction);
            potential_paths.push(self.get_paths(next_pos, new_path));
        }
        self.seen[next_pos] = false;
        Self::combine_paths(potential_paths)
    }
}

impl NumericButton {
    fn layout() -> Array2<Option<Self>> {
        array![
            [Some(7), Some(8), Some(9)],
            [Some(4), Some(5), Some(6)],
            [Some(1), Some(2), Some(3)],
            [None, Some(0), Some(10)],
        ]
        .mapv(|v| v.map(|v| v.into()))
    }
    fn find_routes() -> BTreeMap<[Self; 2], Vec<Vec<Direction>>> {
        let layout = Self::layout();
        let mut solver = PathSolver::new(layout);
        let mut out = BTreeMap::new();
        for start in NUMERIC_BUTTONS {
            for end in NUMERIC_BUTTONS {
                let paths = solver.shortest_paths(&start, &end);
                out.insert([start, end], paths);
            }
        }
        out
    }
}
impl From<Direction> for DirectionalButton {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => DirectionalButton::Up,
            Direction::Down => DirectionalButton::Down,
            Direction::Left => DirectionalButton::Left,
            Direction::Right => DirectionalButton::Right,
        }
    }
}
impl DirectionalButton {
    fn layout() -> Array2<Option<Self>> {
        array![
            [
                None,
                Some(DirectionalButton::Up),
                Some(DirectionalButton::Activate)
            ],
            [
                Some(DirectionalButton::Left),
                Some(DirectionalButton::Down),
                Some(DirectionalButton::Right)
            ],
        ]
    }
    pub fn find_routes() -> BTreeMap<[Self; 2], Vec<Vec<Direction>>> {
        let layout = Self::layout();
        let mut solver = PathSolver::new(layout);
        let mut out = BTreeMap::new();
        for start in DIRECTIONAL_BUTTONS {
            for end in DIRECTIONAL_BUTTONS {
                let paths = solver.shortest_paths(&start, &end);
                out.insert([start, end], paths);
            }
        }
        out
    }
}

#[memoize]
fn find_directional_routes(
    start: DirectionalButton,
    end: DirectionalButton,
) -> Vec<Vec<DirectionalButton>> {
    let layout = DirectionalButton::layout();
    let mut solver = PathSolver::new(layout);
    solver
        .shortest_paths(&start, &end)
        .iter()
        .map(|r| route_to_sequence(r))
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn position_from(&self, pos: [usize; 2]) -> Option<[usize; 2]> {
        match self {
            Direction::Up => {
                if pos[0] == 0 {
                    None
                } else {
                    Some([pos[0] - 1, pos[1]])
                }
            }
            Direction::Right => Some([pos[0], pos[1] + 1]),
            Direction::Down => Some([pos[0] + 1, pos[1]]),
            Direction::Left => {
                if pos[1] == 0 {
                    None
                } else {
                    Some([pos[0], pos[1] - 1])
                }
            }
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    numeric_sequences: Vec<Vec<NumericButton>>,
    numbers: Vec<usize>,
    numeric_routes: BTreeMap<[NumericButton; 2], Vec<Vec<Direction>>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numeric_sequences = s
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        let numbers = s
            .lines()
            .map(|line| {
                let len = line.len();
                line[0..len - 1].parse::<usize>().unwrap()
            })
            .collect();
        let numeric_routes = NumericButton::find_routes();
        Ok(Puzzle {
            numeric_sequences,
            numeric_routes,
            numbers,
        })
    }
}

fn route_to_sequence(route: &[Direction]) -> Vec<DirectionalButton> {
    let mut out = Vec::with_capacity(route.len() + 1);
    route.iter().for_each(|&d| out.push(d.into()));
    out.push(DirectionalButton::Activate);
    out
}

impl Display for DirectionalButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DirectionalButton::Up => write!(f, "^"),
            DirectionalButton::Right => write!(f, ">"),
            DirectionalButton::Down => write!(f, "v"),
            DirectionalButton::Left => write!(f, "<"),
            DirectionalButton::Activate => write!(f, "A"),
        }
    }
}
impl Display for NumericButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericButton::Zero => write!(f, "0"),
            NumericButton::One => write!(f, "1"),
            NumericButton::Two => write!(f, "2"),
            NumericButton::Three => write!(f, "3"),
            NumericButton::Four => write!(f, "4"),
            NumericButton::Five => write!(f, "5"),
            NumericButton::Six => write!(f, "6"),
            NumericButton::Seven => write!(f, "7"),
            NumericButton::Eight => write!(f, "8"),
            NumericButton::Nine => write!(f, "9"),
            NumericButton::Activate => write!(f, "A"),
        }
    }
}
pub fn print_directional_sequence(seq: &[DirectionalButton]) {
    for button in seq {
        print!("{}", button);
    }
    println!();
}
pub fn print_numeric_sequence(seq: &[NumericButton]) {
    for button in seq {
        print!("{}", button);
    }
    println!();
}

#[memoize]
fn shortest_sequence(start: DirectionalButton, end: DirectionalButton, level: usize) -> usize {
    if level == 0 {
        // we can just return the first option
        find_directional_routes(start, end)[0].len()
    } else {
        let mut shortest_option: Option<usize> = None;
        for seq in &find_directional_routes(start, end) {
            let mut inner_pos = DirectionalButton::Activate;
            let mut len = 0;
            for &inner_button in seq {
                len += shortest_sequence(inner_pos, inner_button, level - 1);
                inner_pos = inner_button;
            }
            if let Some(shortest_seq) = &shortest_option {
                if &len < shortest_seq {
                    shortest_option = Some(len);
                }
            } else {
                shortest_option = Some(len);
            }
        }
        shortest_option.unwrap()
    }
}

impl Puzzle {
    fn process(&mut self, levels: usize) -> usize {
        let mut out = 0;
        for (sequence, number) in self.numeric_sequences.iter().zip(self.numbers.iter()) {
            let shortest = self.map_sequence(sequence, levels);
            println!("{} * {}", number, shortest);
            out += shortest * number;
        }
        out
    }
    fn map_sequence(&self, sequence: &[NumericButton], levels: usize) -> usize {
        let mut pos = NumericButton::Activate;
        let mut out = 0;
        for next_button in sequence {
            let mut shortest_option: Option<usize> = None;
            for route in &self.numeric_routes[&[pos, *next_button]] {
                let seq = route_to_sequence(route);
                let mut inner_pos = DirectionalButton::Activate;
                let mut len = 0;
                for inner_button in seq {
                    len += shortest_sequence(inner_pos, inner_button, levels - 1);
                    inner_pos = inner_button;
                }
                if let Some(shortest) = &shortest_option {
                    if &len < shortest {
                        shortest_option = Some(len);
                    }
                } else {
                    shortest_option = Some(len);
                }
            }
            out += shortest_option.unwrap();
            pos = *next_button;
        }
        out
    }
}

fn main() {
    let mut puzzle = include_str!("21.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(25);
    println!("{out}");
    assert_eq!(out, 281212077733592);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("21_test.txt").parse::<Puzzle>().unwrap();
        assert_eq!(out.process(2), 126384);
    }
    #[test]
    fn test_path_finder() {
        let mut solver = PathSolver::new(NumericButton::layout());
        let paths = solver.shortest_paths(&NumericButton::Activate, &NumericButton::Seven);
        dbg!(paths);
    }
    #[test]
    fn test_path_cache_numeric_button() {
        dbg!(NumericButton::find_routes());
    }
    #[test]
    fn test_path_cache_directional_button() {
        dbg!(DirectionalButton::find_routes());
    }
}
