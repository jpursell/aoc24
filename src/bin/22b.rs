use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet, VecDeque},
    str::FromStr,
};

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
    fn make_sequence(&mut self, steps: usize) -> BTreeMap<[i8; 4], i8> {
        let mut out = BTreeMap::new();
        let mut prev = (self.value % 10) as i8;
        let mut deltas = VecDeque::new();
        for _ in 0..steps {
            self.evolve();
            let current = (self.value % 10) as i8;
            deltas.push_back(current - prev);
            if deltas.len() > 4 {
                deltas.pop_front();
            }
            prev = current;
            if deltas.len() < 4 {
                continue;
            }
            match out.entry([deltas[0], deltas[1], deltas[2], deltas[3]]) {
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(current);
                }
                Entry::Occupied(_occupied_entry) => (),
            }
        }
        out
    }
}

impl Puzzle {
    fn process(&mut self, steps: usize) -> usize {
        let sequences: Vec<_> = self
            .secrets
            .iter_mut()
            .map(|s| s.make_sequence(steps))
            .collect();
        let mut keys = BTreeSet::new();
        for seq in &sequences {
            for key in seq.keys() {
                keys.insert(key);
            }
        }
        let mut best_sum = 0;
        let mut best_key = None;
        for key in &keys {
            let sum = sequences
                .iter()
                .map(|seq| *seq.get(*key).unwrap_or(&0) as usize)
                .sum::<usize>();
            if sum > best_sum {
                best_sum = sum;
                best_key = Some(key);
            }
        }
        dbg!(best_key);
        best_sum
    }
}

fn main() {
    let mut puzzle = include_str!("22.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(2000);
    println!("{out}");
    // assert_eq!(out, );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("22_test_b.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process(2000);
        assert_eq!(out, 23);
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
    #[test]
    fn test_sequence() {
        let mut secret = Secret { value: 123 };
        assert_eq!(secret.make_sequence(10)[&[-1, -1, 0, 2]], 6);
    }
}
