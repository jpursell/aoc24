use ndarray::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Token {
    Wall,
    Box,
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
            let ncols = map_lines[0].len();
            let mut map = Vec::with_capacity(nrows * ncols);
            let mut robot = [0, 0];
            for (irow, line) in map_lines.iter().enumerate() {
                for (icol, c) in line.chars().enumerate() {
                    map.push(match c {
                        '#' => Token::Wall,
                        'O' => Token::Box,
                        '.' => Token::None,
                        '@' => {
                            robot = [irow, icol];
                            Token::None
                        }
                        _ => panic!(),
                    });
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
    fn process(&self) -> usize {
        let mut map = self.map.to_owned();
        let mut robot = self.robot;
        for direction in &self.directions {
            if Puzzle::can_move(map.view(), &robot, direction) {
                Puzzle::do_move(map.view_mut(), &robot, direction);
                robot = direction.position_from(&robot).unwrap();
            }
        }
        map.indexed_iter()
            .map(|(pos, x)| match x {
                Token::Box => pos.0 * 100 + pos.1,
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
            Token::Box => Puzzle::can_move(map, &new_position, direction),
        }
    }
    fn do_move(mut map: ArrayViewMut2<Token>, position: &[usize; 2], direction: &Direction) {
        let new_position = direction.position_from(position).unwrap();
        match map[new_position] {
            Token::None => (),
            Token::Wall => panic!(),
            Token::Box => {
                Puzzle::do_move(map.view_mut(), &new_position, direction);
            }
        }
        assert!(matches!(map[new_position], Token::None));
        match map[*position] {
            Token::Box => {
                map[new_position] = Token::Box;
                map[*position] = Token::None;
            }
            Token::None => (),
            Token::Wall => panic!(),
        }
    }
}

fn main() {
    let puzzle = include_str!("15.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 1495147);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_small() {
        let out = include_str!("15_test_small.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process();
        assert_eq!(out, 2028);
    }
    #[test]
    fn test() {
        let out = include_str!("15_test.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 10092);
    }
}
