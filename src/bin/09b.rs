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

/// Find first area on disc that can fit file of length size
/// Return None if nothing is available before end
fn find_first_space(disc: &[Option<usize>], size: usize, end: usize) -> Option<usize> {
    let mut start = None;
    let mut size_found = 0;
    for (i, block) in disc.iter().enumerate() {
        if i >= end {
            return None;
        }
        if block.is_some() {
            size_found = 0;
            start = None;
        } else {
            if start.is_none() {
                start = Some(i);
            }
            size_found += 1;
            if size_found >= size {
                return start;
            }
        }
    }
    None
}

fn compact_disc(disc: &mut [Option<usize>]) {
    let mut read_head = disc.len() - 1;
    let start_id = disc[read_head].unwrap();
    for id in (1..=start_id).rev() {
        while disc[read_head].is_none() || disc[read_head].unwrap() != id {
            assert!(read_head > 0);
            read_head -= 1;
        }
        let block_end = read_head + 1;
        while disc[read_head].is_some() && disc[read_head].unwrap() == id {
            assert!(read_head > 0);
            read_head -= 1;
        }
        let block_start = read_head + 1;
        let block_length = block_start - block_end;

        let gap = find_first_space(disc, block_length, block_start);
        if gap.is_none() {
            continue;
        }
        let gap = gap.unwrap();
        for i in 0..block_length {
            assert!(disc[i + block_start].is_some());
            disc[i + gap] = disc[i + block_start];
            disc[i + block_start] = None;
        }
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
