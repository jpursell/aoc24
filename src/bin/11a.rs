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

fn digit_count(num: &usize) -> usize {
    let mut digits = 0;
    let mut val = 1;
    while &val <= num {
        val *= 10;
        digits += 1;
    }
    digits
}

fn split_digits(num: &usize, ndigits: usize) -> (usize, usize) {
    let val = 10_usize.pow((ndigits / 2) as u32);
    let left = num / val;
    let right = num - left * val;
    (left, right)
}

impl Puzzle {
    fn process(&mut self) -> usize {
        while self.blinks < BLINKS {
            self.blink();
            self.blinks += 1;
        }
        self.stone_count()
    }
    fn current_index(&self) -> usize {
        self.blinks % 2
    }
    fn stone_count(&self) -> usize {
        self.buffers[self.current_index()].len()
    }
    fn blink(&mut self) {
        let current_index = self.current_index();
        let (b0, b1) = self.buffers.split_at_mut(1);
        let current;
        let next;
        if current_index == 0 {
            current = &b0[0];
            next = &mut b1[0];
        } else {
            current = &b1[0];
            next = &mut b0[0];
        }
        next.clear();
        for stone in current {
            if stone == &0 {
                next.push(1)
            } else {
                let digit_count = digit_count(stone);
                if digit_count % 2 == 0 {
                    let (left, right) = split_digits(stone, digit_count);
                    next.push(left);
                    next.push(right);
                } else {
                    next.push(stone * 2024);
                }
            }
        }
    }
}

fn main() {
    let mut puzzle = include_str!("11.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 203609);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut out = include_str!("11_test.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 55312);
    }

    #[test]
    fn test_digit_count_1() {
        let out = digit_count(&1);
        assert_eq!(out, 1);
    }
    #[test]
    fn test_digit_count_9() {
        let out = digit_count(&9);
        assert_eq!(out, 1);
    }
    #[test]
    fn test_digit_count_10() {
        let out = digit_count(&10);
        assert_eq!(out, 2);
    }
}
