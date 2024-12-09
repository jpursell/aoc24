use std::str::FromStr;

#[derive(Debug)]
struct Puzzle {
    digits: Vec<usize>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.chars().count();
        let mut digits = Vec::with_capacity(count);
        for digit in s.chars() {
            digits.push(digit.to_string().parse::<usize>().unwrap());
        }
        Ok(Puzzle { digits })
    }
}

fn compact_disc(disc: &mut [Option<usize>]) {
    let mut write_head = 0;
    let mut read_head = disc.len() - 1;
    loop {
        while disc[write_head].is_some() {
            write_head += 1;
        }
        while disc[read_head].is_none() {
            read_head -= 1;
        }
        if write_head > read_head {
            break;
        }
        disc[write_head] = disc[read_head];
        disc[read_head] = None;
    }
}
impl Puzzle {
    fn process(&mut self) -> usize {
        let mut disc = self.create_disc();
        compact_disc(&mut disc);
        disc.iter()
            .enumerate()
            .filter(|(_, id)| id.is_some())
            .map(|(index, id)| index * id.unwrap())
            .sum::<usize>()
    }
    fn create_disc(&self) -> Vec<Option<usize>> {
        let mut disc = Vec::with_capacity(self.digits.iter().sum());
        let mut space = false;
        let mut id = 0;
        for digit in &self.digits {
            if space {
                for _ in 0..*digit {
                    disc.push(None);
                }
            } else {
                for _ in 0..*digit {
                    disc.push(Some(id));
                }
                id += 1;
            }
            space = !space;
        }
        disc
    }
}

fn main() {
    let mut puzzle = include_str!("09.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 6320029754031);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut puzzle = include_str!("09_test.txt").parse::<Puzzle>().unwrap();
        let out = puzzle.process();
        assert_eq!(out, 1928);
    }
    #[test]
    fn check_input_length() {
        assert!(include_str!("09_test.txt").chars().count() % 2 == 1);
        assert!(include_str!("09.txt").chars().count() % 2 == 1);
    }
}
