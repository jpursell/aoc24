use std::str::FromStr;

#[derive(Debug)]
struct Puzzle {
    secrets: Vec<Secret>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let initials = s
            .lines()
            .map(|line| Secret {
                value: line.parse::<usize>().unwrap(),
            })
            .collect();
        Ok(Puzzle { secrets: initials })
    }
}

#[derive(Debug)]
struct Secret {
    value: usize,
}
impl Secret {
    fn mix(&mut self, num: usize) {
        self.value ^= num;
    }
    fn prune(&mut self) {
        self.value %= 16777216;
    }
    fn evolve(&mut self) {
        self.mix(self.value * 64);
        self.prune();
        self.mix(self.value / 32);
        self.prune();
        self.mix(self.value * 2048);
        self.prune();
    }
}

impl Puzzle {
    fn process(&mut self, steps: usize) -> usize {
        for secret in &mut self.secrets {
            for _ in 0..steps {
                secret.evolve();
            }
        }
        self.secrets.iter().map(|x| x.value).sum()
    }
}

fn main() {
    let mut puzzle = include_str!("22.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(2000);
    println!("{out}");
    assert_eq!(out, 18525593556);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("22_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process(2000);
        assert_eq!(out, 37327623);
    }
    #[test]
    fn test_mix() {
        let mut secret = Secret { value: 42 };
        secret.mix(15);
        assert_eq!(secret.value, 37);
    }
    #[test]
    fn test_prune() {
        let mut secret = Secret { value: 100000000 };
        secret.prune();
        assert_eq!(secret.value, 16113920);
    }
    #[test]
    fn test_evolve() {
        let mut secret = Secret { value: 123 };
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for expected in expected {
            secret.evolve();
            assert_eq!(secret.value, expected);
        }
    }
}
