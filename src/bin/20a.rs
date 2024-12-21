use ndarray::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Token {
    Wall,
    Track,
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
    steps_map: Array2<Option<usize>>,
    positions: Vec<[usize; 2]>,
    cheats: Vec<usize>,
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
                    '.' => Token::Track,
                    '#' => Token::Wall,
                    'S' => {
                        start = [irow, icol];
                        Token::Track
                    }
                    'E' => {
                        end = [irow, icol];
                        Token::Track
                    }
                    _ => panic!(),
                });
            }
        }
        let map = Array2::from_shape_vec((nrows, ncols), map).unwrap();
        let steps = Array2::from_elem(map.raw_dim(), None);
        Ok(Puzzle {
            map,
            start,
            end,
            steps_map: steps,
            positions: Vec::new(),
            cheats: Vec::new(),
        })
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Puzzle {
    fn solve_steps(&mut self) {
        self.steps_map.iter_mut().for_each(|x| *x = None);
        self.steps_map[self.start] = Some(0);
        let mut pos = self.start;
        let mut steps = 0;
        self.positions.push(pos);
        while pos != self.end {
            pos = self.find_next_pos(&pos);
            self.positions.push(pos);
            steps += 1;
            self.steps_map[pos] = Some(steps);
        }
    }
    fn find_cheats(&mut self) {
        for pos in &self.positions {
            for direction in DIRECTIONS {
                let cheat_pos = direction.position_from(pos);
                if cheat_pos.is_none() {
                    continue;
                }
                let cheat_pos = direction.position_from(&cheat_pos.unwrap());
                if cheat_pos.is_none() {
                    continue;
                }
                let cheat_steps = self.steps_map.get(cheat_pos.unwrap());
                if cheat_steps.is_none() || cheat_steps.unwrap().is_none() {
                    continue;
                }
                let cheat_steps = cheat_steps.unwrap().unwrap();
                let current_steps = self.steps_map[*pos].unwrap();
                if cheat_steps > current_steps + 2 {
                    let improvement = cheat_steps - (current_steps + 2);
                    self.cheats.push(improvement);
                    // println!(
                    //     "Find Cheat {} -> {} , Improvement: {}",
                    //     current_steps, cheat_steps, improvement
                    // );
                }
            }
        }
    }
    fn process(&mut self, time_saved: usize) -> usize {
        self.solve_steps();
        self.print_steps();
        self.find_cheats();
        self.cheats.iter().filter(|&x| x >= &time_saved).count()
    }
    fn find_next_pos(&self, pos: &[usize; 2]) -> [usize; 2] {
        let mut next_pos = None;
        for next_direction in DIRECTIONS {
            let potential_next_pos = next_direction.position_from(pos);
            if potential_next_pos.is_none() {
                continue;
            }
            let potential_next_pos = potential_next_pos.unwrap();
            let next_token = self.map.get(potential_next_pos);
            if next_token.is_none() {
                continue;
            }
            if matches!(next_token.unwrap(), Token::Wall) {
                continue;
            }
            if self.steps_map[potential_next_pos].is_some() {
                continue;
            }
            if next_pos.is_some() {
                panic!();
            }
            next_pos = Some(potential_next_pos);
        }
        next_pos.unwrap()
    }
    fn print_steps(&self) {
        for irow in 0..self.steps_map.shape()[0] {
            for icol in 0..self.steps_map.shape()[1] {
                if let Some(steps) = self.steps_map[[irow, icol]] {
                    print!("{:02} ", steps);
                } else {
                    print!("   ");
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let mut puzzle = include_str!("20.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(100);
    println!("{out}");
    assert_eq!(out, 1346);
}

#[cfg(test)]
mod tests {
    use counter::Counter;

    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("20_test.txt").parse::<Puzzle>().unwrap();
        out.solve_steps();
        out.print_steps();
        out.find_cheats();
        let cheat_count = out.cheats.iter().map(|x| *x).collect::<Counter<usize>>();

        dbg!(&cheat_count);
        assert_eq!(14, cheat_count[&2]);
        assert_eq!(14, cheat_count[&4]);
        assert_eq!(2, cheat_count[&6]);
        assert_eq!(4, cheat_count[&8]);
        assert_eq!(2, cheat_count[&10]);
        assert_eq!(3, cheat_count[&12]);
        assert_eq!(1, cheat_count[&20]);
        assert_eq!(1, cheat_count[&36]);
        assert_eq!(1, cheat_count[&38]);
        assert_eq!(1, cheat_count[&40]);
        assert_eq!(1, cheat_count[&64]);
    }
    #[test]
    fn test_solve_steps() {
        let mut out = include_str!("20_test.txt").parse::<Puzzle>().unwrap();
        out.solve_steps();
        // dbg!(out);
    }
}
