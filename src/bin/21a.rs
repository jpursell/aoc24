use ndarray::prelude::*;
use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NumericButton {
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

// impl From<NumericButton> for usize {
//     fn from(value: NumericButton) -> usize {
//         match value {
//             NumericButton::Zero => 0,
//             NumericButton::One => 1,
//             NumericButton::Two => 2,
//             NumericButton::Three => 3,
//             NumericButton::Four => 4,
//             NumericButton::Five => 5,
//             NumericButton::Six => 6,
//             NumericButton::Seven => 7,
//             NumericButton::Eight => 8,
//             NumericButton::Nine => 9,
//             NumericButton::Activate => 10,
//         }
//     }
// }

const NUMERIC_BUTTONS: [NumericButton; 10] = [
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

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

struct PathSolver<T> {
    layout: Array2<T>,
    seen: Array2<bool>,
    end: [usize; 2],
}
impl<T> PathSolver<T>
where
    T: Eq,
{
    fn new(layout: Array2<T>) -> Self {
        let seen = Array2::from_elem(layout.raw_dim(), false);
        PathSolver {
            seen,
            layout,
            end: [0, 0],
        }
    }
    fn find_button(&self, button_to_find: &T) -> Option<[usize; 2]> {
        for (pos, button) in self.layout.indexed_iter() {
            if button == button_to_find {
                return Some([pos.0, pos.1]);
            }
        }
        None
    }
    fn shortest_paths(&mut self, start: &T, end: &T) -> Vec<Vec<Direction>> {
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
        let paths: Vec<&mut Vec<Vec<Direction>>> = paths
            .iter_mut()
            .filter(|f| f.is_some())
            .map(|x| x.as_mut().unwrap())
            .collect();
        let shortest_len = paths.iter().map(|x| x[0].len()).min().unwrap();
        let mut out: Vec<Vec<Direction>> = Vec::new();

        for mut p in paths {
            if p[0].len() != shortest_len {
                continue;
            }
            out.append(&mut p);
        }
        Some(out)
    }
    fn get_paths(&mut self, pos: [usize; 2], path: Vec<Direction>) -> Option<Vec<Vec<Direction>>> {
        let direction = path.last().unwrap();
        let next_pos = direction.position_from(pos)?;
        {
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
        .mapv(|v| match v {
            None => None,
            Some(v) => Some(v.into()),
        })
    }
    fn find_routes() -> BTreeMap<[Self; 2], Vec<Vec<Direction>>> {
        let layout = Self::layout();
        let mut solver = PathSolver::new(layout);
        let mut out = BTreeMap::new();
        for start in NUMERIC_BUTTONS {
            for end in NUMERIC_BUTTONS {
                let paths = solver.shortest_paths(&Some(start), &Some(end));
                out.insert([start, end], paths);
            }
        }
        out
    }
    // fn find_shortest_path()
    //     fn list_connections(&self) -> BTreeMap<Direction, Self> {
    //         match self {
    //             NumericButton::Zero => BTreeMap::from([
    //                 (Direction::Right, NumericButton::Activate),
    //                 (Direction::Up, NumericButton::Two),
    //             ]),
    //             NumericButton::One => BTreeMap::from([
    //                 (Direction::Right, NumericButton::Two),
    //                 (Direction::Up, NumericButton::Four),
    //             ]),
    //             NumericButton::Two => BTreeMap::from([
    //                 (Direction::Up, NumericButton::Five),
    //                 (Direction::Right, NumericButton::Three),
    //                 (Direction::Down, NumericButton::Zero),
    //                 (Direction::Left, NumericButton::One),
    //             ]),
    //             NumericButton::Three => BTreeMap::from([

    //             ]),
    //             NumericButton::Four => todo!(),
    //             NumericButton::Five => todo!(),
    //             NumericButton::Six => todo!(),
    //             NumericButton::Seven => todo!(),
    //             NumericButton::Eight => todo!(),
    //             NumericButton::Nine => todo!(),
    //             NumericButton::Activate => todo!(),
    //         }
    //     }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum DirectionalButton {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
enum Direction {
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
    numbers: Vec<Vec<NumericButton>>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();
        Ok(Puzzle { numbers })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        0
    }
}

fn main() {
    let mut puzzle = include_str!("21.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("21_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        assert_eq!(out.process(), 126384);
    }
    #[test]
    fn test_path_finder() {
        let mut solver = PathSolver::new(NumericButton::layout());
        let paths =
            solver.shortest_paths(&Some(NumericButton::Activate), &Some(NumericButton::Seven));
        dbg!(paths);
    }
}
