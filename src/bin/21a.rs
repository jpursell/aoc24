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
    fn find_routes() -> BTreeMap<[Self; 2], Vec<Direction>> {
        let layout = Self::layout();
        let find_button = |button_to_find: &Self| -> [usize; 2] {
            for (pos, button) in layout.indexed_iter() {
                if button.is_none() || &button.unwrap() != button_to_find {
                    continue;
                }
                return [pos.0, pos.1];
            }
            panic!();
        };
        for start in &NUMERIC_BUTTONS {
            let start_pos = find_button(start);
            todo!();
        }
        BTreeMap::new()
    }
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
}
