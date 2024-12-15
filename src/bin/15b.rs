use ndarray::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Token {
    Wall,
    BoxLeft,
    BoxRight,
    None,
}

#[derive(Debug)]
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
    robot: [usize; 2],
    directions: Vec<Direction>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (map_lines, direction_lines) = {
            let nlines = s.lines().count();
            let mut map_lines = Vec::with_capacity(nlines);
            let mut direction_lines = Vec::with_capacity(nlines);
            let mut getting_map = true;
            for line in s.lines() {
                if line.is_empty() {
                    getting_map = false;
                    continue;
                }
                if getting_map {
                    map_lines.push(line);
                } else {
                    direction_lines.push(line);
                }
            }
            (map_lines, direction_lines)
        };

        let (map, robot) = {
            let nrows = map_lines.len();
            let ncols = map_lines[0].len() * 2;
            let mut map = Vec::with_capacity(nrows * ncols);
            let mut robot = [0, 0];
            for (irow, line) in map_lines.iter().enumerate() {
                for (icol, c) in line.chars().enumerate() {
                    match c {
                        '#' => {
                            map.push(Token::Wall);
                            map.push(Token::Wall);
                        }
                        'O' => {
                            map.push(Token::BoxLeft);
                            map.push(Token::BoxRight);
                        }
                        '.' => {
                            map.push(Token::None);
                            map.push(Token::None);
                        }
                        '@' => {
                            robot = [irow, icol * 2];
                            map.push(Token::None);
                            map.push(Token::None);
                        }
                        _ => panic!(),
                    }
                }
            }
            (Array2::from_shape_vec((nrows, ncols), map).unwrap(), robot)
        };

        let mut directions =
            Vec::with_capacity(direction_lines.iter().map(|line| line.len()).sum());
        for line in direction_lines {
            for c in line.chars() {
                directions.push(match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => panic!(),
                });
            }
        }

        Ok(Puzzle {
            map,
            robot,
            directions,
        })
    }
}

impl Puzzle {
    fn process(&self, print: bool) -> usize {
        let mut map = self.map.to_owned();
        let mut robot = self.robot;
        if print {
            Puzzle::print_map(map.view(), &robot);
        }
        for direction in &self.directions {
            if print {
                println!(
                    "{}",
                    match direction {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    }
                );
            }
            if Puzzle::can_move(map.view(), &robot, direction) {
                Puzzle::do_move(map.view_mut(), &robot, direction);
                robot = direction.position_from(&robot).unwrap();
                if print {
                    Puzzle::print_map(map.view(), &robot);
                }
            }
        }
        map.indexed_iter()
            .map(|(pos, x)| match x {
                Token::BoxLeft => pos.0 * 100 + pos.1,
                Token::BoxRight => 0,
                Token::None => 0,
                Token::Wall => 0,
            })
            .sum()
    }
    fn can_move(map: ArrayView2<Token>, position: &[usize; 2], direction: &Direction) -> bool {
        let new_position = direction.position_from(position).unwrap();
        match map[new_position] {
            Token::None => true,
            Token::Wall => false,
            Token::BoxLeft => match direction {
                Direction::Left => {
                    panic!();
                }
                Direction::Right => {
                    let second_position = direction.position_from(&new_position).unwrap();
                    Puzzle::can_move(map, &second_position, direction)
                }
                Direction::Up | Direction::Down => {
                    let second_position = Direction::Right.position_from(&new_position).unwrap();
                    Puzzle::can_move(map, &new_position, direction)
                        && Puzzle::can_move(map, &second_position, direction)
                }
            },
            Token::BoxRight => match direction {
                Direction::Right => {
                    panic!();
                }
                Direction::Left => {
                    let second_position = direction.position_from(&new_position).unwrap();
                    Puzzle::can_move(map, &second_position, direction)
                }
                Direction::Up | Direction::Down => {
                    let second_position = Direction::Left.position_from(&new_position).unwrap();
                    Puzzle::can_move(map, &new_position, direction)
                        && Puzzle::can_move(map, &second_position, direction)
                }
            },
        }
    }
    fn do_move(mut map: ArrayViewMut2<Token>, position: &[usize; 2], direction: &Direction) {
        let new_position = direction.position_from(position).unwrap();
        let mut second_position = None;
        match map[new_position] {
            Token::None => (),
            Token::Wall => panic!(),
            Token::BoxLeft => match direction {
                Direction::Right | Direction::Left => {
                    Puzzle::do_move(map.view_mut(), &new_position, direction);
                }
                Direction::Down | Direction::Up => {
                    second_position = Some(Direction::Right.position_from(&new_position).unwrap());
                    Puzzle::do_move(map.view_mut(), &new_position, direction);
                    Puzzle::do_move(map.view_mut(), &second_position.unwrap(), direction);
                }
            },
            Token::BoxRight => match direction {
                Direction::Left | Direction::Right => {
                    Puzzle::do_move(map.view_mut(), &new_position, direction);
                }
                Direction::Down | Direction::Up => {
                    second_position = Some(Direction::Left.position_from(&new_position).unwrap());
                    Puzzle::do_move(map.view_mut(), &new_position, direction);
                    Puzzle::do_move(map.view_mut(), &second_position.unwrap(), direction);
                }
            },
        }
        assert!(matches!(map[new_position], Token::None));
        if let Some(second_position) = second_position {
            assert!(matches!(map[second_position], Token::None));
        }
        match map[*position] {
            Token::BoxLeft | Token::BoxRight => {
                map[new_position] = map[*position];
                map[*position] = Token::None;
            }
            Token::None => (),
            Token::Wall => panic!(),
        }
    }
    fn print_map(map: ArrayView2<Token>, robot: &[usize; 2]) {
        let mut current_irow = 0;
        let robot = (robot[0], robot[1]);
        for (pos, token) in map.indexed_iter() {
            if pos.0 != current_irow {
                println!();
                current_irow = pos.0;
            }
            print!(
                "{}",
                if pos == robot {
                    '@'
                } else {
                    match token {
                        Token::Wall => '#',
                        Token::BoxLeft => '[',
                        Token::BoxRight => ']',
                        Token::None => '.',
                    }
                }
            );
        }
        println!();
    }
}

fn main() {
    let puzzle = include_str!("15.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(false);
    println!("{out}");
    assert_eq!(out, 1524905);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let out = include_str!("15_test.txt").parse::<Puzzle>().unwrap();
        let out = out.process(true);
        assert_eq!(out, 9021);
    }
}
