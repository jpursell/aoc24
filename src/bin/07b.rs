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
        let values: Vec<usize> = values
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Ok(Equation { result, values })
    }
}

fn concatenate(a: usize, b: usize) -> usize {
    let mut value = 1;
    while value <= b {
        value *= 10;
    }
    a * value + b
}

impl Equation {
    fn solvable(&self) -> bool {
        self.count_solutions(0, self.values[0]) > 0
    }
    fn count_solutions(&self, position: usize, partial_value: usize) -> usize {
        assert!(position < self.values.len() - 1);
        let at_bottom = position == self.values.len() - 2;
        let mut solutions = 0;
        // try Add
        {
            let partial_value = partial_value + self.values[position + 1];
            if partial_value <= self.result {
                if at_bottom {
                    if partial_value == self.result {
                        solutions += 1;
                    }
                } else {
                    solutions += self.count_solutions(position + 1, partial_value)
                }
            }
        }
        // try Multiply
        {
            let partial_value = partial_value * self.values[position + 1];
            if partial_value <= self.result {
                if at_bottom {
                    if partial_value == self.result {
                        solutions += 1;
                    }
                } else {
                    solutions += self.count_solutions(position + 1, partial_value)
                }
            }
        }
        // try Concatenation
        {
            let partial_value = concatenate(partial_value, self.values[position + 1]);
            if partial_value <= self.result {
                if at_bottom {
                    if partial_value == self.result {
                        solutions += 1;
                    }
                } else {
                    solutions += self.count_solutions(position + 1, partial_value)
                }
            }
        }
        solutions
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
    assert_eq!(out, 254136560217241);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut puzzle = include_str!("07_test.txt").parse::<Puzzle>().unwrap();
        let out = puzzle.process();
        assert_eq!(out, 11387);
    }
    #[test]
    fn test_no_zeros() {
        let puzzle = include_str!("07.txt").parse::<Puzzle>().unwrap();
        for equation in &puzzle.equations {
            for value in &equation.values {
                assert_ne!(value, &0);
            }
            assert_ne!(equation.result, 0);
        }
    }
}
