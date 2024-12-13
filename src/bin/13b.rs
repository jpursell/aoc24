use std::str::FromStr;

#[derive(Debug)]
struct Game {
    buttons: [[usize; 2]; 2],
    prize: [usize; 2],
}

impl Game {
    fn solve(&self) -> Option<[usize; 2]> {
        let [[ax, ay], [bx, by]] = self.buttons.map(|v| v.map(|x| x as f64));
        let [px, py] = self.prize.map(|x| x as f64);
        let b = (py * ax - ay * px) / (by * ax - bx * ay);
        let a = (px - bx * b) / ax;
        if a % 1.0 == 0.0 && b % 1.0 == 0.0 {
            Some([a as usize, b as usize])
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    games: Vec<Game>,
}

impl From<Vec<&str>> for Game {
    fn from(value: Vec<&str>) -> Self {
        let parse_num = |s: &str| {
            let (_, num) = s.split_once("+").unwrap();
            num.parse::<usize>().unwrap()
        };
        let parse_button = |s: &str| {
            let (_, rem) = s.split_once(": ").unwrap();
            let (x, y) = rem.split_once(", ").unwrap();
            [parse_num(x), parse_num(y)]
        };
        let parse_prize_num = |s: &str| {
            let (_, num) = s.split_once("=").unwrap();
            let offset = 10000000000000;
            num.parse::<usize>().unwrap() + offset
        };
        let parse_prize = |s: &str| {
            let (_, rem) = s.split_once(": ").unwrap();
            let (x, y) = rem.split_once(", ").unwrap();
            [parse_prize_num(x), parse_prize_num(y)]
        };
        let buttons = [parse_button(value[0]), parse_button(value[1])];
        let prize = parse_prize(value[2]);
        Game { buttons, prize }
    }
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = &mut s.lines();
        let mut games = Vec::new();
        loop {
            games.push(Game::from(lines.take(3).collect::<Vec<_>>()));
            if lines.next().is_none() {
                break;
            }
        }
        Ok(Puzzle { games })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        self.games
            .iter()
            .map(|g| {
                if let Some([a, b]) = g.solve() {
                    a * 3 + b
                } else {
                    0
                }
            })
            .sum()
    }
}

fn main() {
    let mut puzzle = include_str!("13.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 102255878088512);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_colinear() {
        let out = include_str!("13.txt").parse::<Puzzle>().unwrap();
        for game in &out.games {
            let ratio_a = game.buttons[0][0] as f64 / game.buttons[0][1] as f64;
            let ratio_b = game.buttons[1][0] as f64 / game.buttons[1][1] as f64;
            assert_ne!(ratio_a, ratio_b);
        }
    }
}
