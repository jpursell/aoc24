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

fn create_sequence(num: usize) -> Vec<usize> {
    let mut sequence = Vec::with_capacity(num);
    sequence.push(1);
    let mut buffers = [Vec::new(), Vec::new()];
    #[derive(Debug)]
    enum Stone {
        Val(usize),
        Seq(usize),
    }
    buffers[0].push(Stone::Val(0));
    for blink in 0..num {
        let (b0, b1) = buffers.split_at_mut(1);
        let (current, next) = if blink % 2 == 0 {
            (&b0[0], &mut b1[0])
        } else {
            (&b1[0], &mut b0[0])
        };
        next.clear();
        for stone in current {
            match stone {
                Stone::Val(0) => {
                    next.push(Stone::Val(1));
                }
                Stone::Val(n) => {
                    let ndigits = digit_count(n);
                    if ndigits % 2 == 0 {
                        let (left, right) = split_digits(n, ndigits);
                        let mut handle_val = |val: usize| match val {
                            0 => next.push(Stone::Seq(0)),
                            1 => next.push(Stone::Seq(1)),
                            n => next.push(Stone::Val(n)),
                        };
                        handle_val(left);
                        handle_val(right);
                    } else {
                        next.push(Stone::Val(2024 * n));
                    }
                }
                Stone::Seq(n) => {
                    next.push(Stone::Seq(n + 1));
                }
            }
        }
        sequence.push(
            next.iter()
                .map(|s| match s {
                    Stone::Seq(n) => sequence[*n],
                    Stone::Val(_) => 1,
                })
                .sum(),
        );
    }
    sequence
}

impl Puzzle {
    fn process(&mut self, num_blinks: usize) -> usize {
        let sequence = create_sequence(num_blinks);
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

    #[test]
    fn test_sequence() {
        // 0
        // 1
        // 2024
        // 20 24
        // 2 0 2 4
        // 4048 1 4048 8096
        // 40 48 2024 40 48 80 96
        // 4 0 4 8 20 24 4 0 4 8 8 0 9 6
        assert_eq!(create_sequence(7), vec![1, 1, 1, 2, 4, 4, 7, 14]);
    }
    #[test]
    fn test_sequence_long() {
        dbg!(create_sequence(50));
    }
}
