use std::str::FromStr;

#[derive(Debug)]
struct Puzzle {
    stones: Vec<usize>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stones = Vec::new();
        for num in s.split(" ") {
            stones.push(num.parse::<usize>().unwrap());
        }
        Ok(Puzzle { stones })
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
    fn process(&mut self, num_blinks: usize) -> usize {
        0
    }
}

fn main() {
    let mut puzzle = include_str!("11.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(75);
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut out = include_str!("11_test.txt").parse::<Puzzle>().unwrap();
        let out = out.process(25);
        assert_eq!(out, 55312);
    }

    #[test]
    fn test_25() {
        let mut out = include_str!("11.txt").parse::<Puzzle>().unwrap();
        let out = out.process(25);
        assert_eq!(out, 203609);
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
