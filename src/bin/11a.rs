use std::str::FromStr;

const BLINKS: usize = 25;

#[derive(Debug)]
struct Puzzle {
    stones: Vec<usize>,
    buffers: [Vec<usize>; 2],
    blinks: usize,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stones = Vec::new();
        for num in s.split(" ") {
            stones.push(num.parse::<usize>().unwrap());
        }
        let size = stones.len() * 2_usize.pow(BLINKS as u32);
        let buffers = [Vec::with_capacity(size), Vec::with_capacity(size)];
        Ok(Puzzle { stones , buffers, blinks: 0})
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        0
    }
}

fn main() {
    let mut puzzle = include_str!("11.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut out = include_str!("11_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process();
        assert_eq!(out, 55312);
    }
}
