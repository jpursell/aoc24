use std::str::FromStr;

const BLINKS: usize = 25;

#[derive(Debug)]
struct Puzzle {
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
        let mut buffers = [Vec::with_capacity(size), Vec::with_capacity(size)];
        for stone in stones {
            buffers[0].push(stone);
        }
        Ok(Puzzle { buffers, blinks: 0 })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        while self.blinks < BLINKS {
            self.blink();
        }
        self.stone_count()
    }
    fn current_index(&self) -> usize {
        self.blinks % 2
    }
    fn next_index(&self) -> usize {
        (self.current_index() + 1) % 2
    }
    fn stone_count(&self) -> usize {
        self.buffers[self.current_index()].len()
    }
    fn blink(&mut self) {
        let current = &self.buffers[self.current_index()];
        let next = &mut self.buffers[self.current_index()];
        todo!()
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
