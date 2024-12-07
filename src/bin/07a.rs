use std::str::FromStr;

#[derive(Debug)]
struct Equation {
    result: usize,
    values: Vec<usize>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, values) = s.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let values = values.split(" ").map(|x| x.parse::<usize>().unwrap()).collect();
        Ok(Equation{result, values})
    }
}

impl Equation {
    fn solvable(&self) -> bool {
        todo!();
    }
}

#[derive(Debug)]
struct Puzzle {
    equations: Vec<Equation>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.lines().count();
        let mut equations = Vec::with_capacity(count);
        for line in s.lines() {
            equations.push(line.parse::<Equation>().unwrap());
        }
        Ok(Puzzle { equations })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        let mut out = 0;
        for equation in &self.equations {
            if equation.solvable() {
                out += equation.result;
            }
        }
        out
    }
}

fn main() {
    let mut puzzle = include_str!("07.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut puzzle = include_str!("07_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&puzzle);
        let out = puzzle.process();
        assert_eq!(out, 3749);
    }
}
