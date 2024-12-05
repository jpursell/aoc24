use std::{
    collections::{btree_map::Entry, BTreeMap, BTreeSet},
    str::FromStr,
};

#[derive(Debug)]
struct Rule {
    left: usize,
    right: usize,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_once("|");
        if split.is_none() {
            return Err(());
        }
        let (left, right) = split.unwrap();
        let left = left.parse::<usize>();
        if left.is_err() {
            return Err(());
        }
        let left = left.unwrap();
        let right = right.parse::<usize>();
        if right.is_err() {
            return Err(());
        }
        let right = right.unwrap();
        Ok(Rule { left, right })
    }
}
#[derive(Debug)]
struct Update {
    pages: Vec<usize>,
}
impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages: Vec<&str> = s.split(",").collect();
        if pages.is_empty() {
            return Err(());
        }
        let pages = pages.iter().map(|p| p.parse::<usize>()).collect::<Vec<_>>();
        if pages.iter().any(|p| p.is_err()) {
            return Err(());
        }
        let pages: Vec<usize> = pages.iter().map(|p| *p.as_ref().unwrap()).collect();
        Ok(Update { pages })
    }
}
impl Update {
    fn check(&self, ruleset: &Ruleset) -> bool {
        let mut seen = BTreeSet::new();
        for page in &self.pages {
            if let Some(rights) = ruleset.rules.get(page) {
                if seen.intersection(rights).count() > 0 {
                    return false;
                }
            }
            seen.insert(*page);
        }
        true
    }
    fn middle(&self) -> usize {
        self.pages[(self.pages.len() - 1) / 2]
    }
}
#[derive(Debug)]
struct Puzzle {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut get_rules = true;
        let mut rules = Vec::new();
        let mut updates = Vec::new();
        for line in s.lines() {
            if line.is_empty() {
                get_rules = false;
                continue;
            }
            if get_rules {
                rules.push(line.parse::<Rule>()?);
            } else {
                updates.push(line.parse::<Update>()?);
            }
        }
        Ok(Puzzle { rules, updates })
    }
}

#[derive(Debug)]
struct Ruleset {
    rules: BTreeMap<usize, BTreeSet<usize>>,
}
impl Ruleset {
    fn new(rules: &[Rule]) -> Self {
        let mut ruleset: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
        for Rule { left, right } in rules {
            match ruleset.entry(*left) {
                Entry::Vacant(e) => {
                    e.insert(BTreeSet::from([*right]));
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(*right);
                }
            }
        }
        Ruleset { rules: ruleset }
    }
}

impl Puzzle {
    fn process(&self) -> usize {
        let mut out = 0;
        let ruleset = Ruleset::new(&self.rules);
        for update in &self.updates {
            if update.check(&ruleset) {
                out += update.middle();
            }
        }
        out
    }
}

fn main() {
    let puzzle = include_str!("05_test.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    assert_eq!(out, 143);

    let puzzle = include_str!("05.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    assert_eq!(out, 6034);
    println!("{out}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_fail() {
        let rules = vec![Rule { left: 1, right: 2 }];
        let ruleset = Ruleset::new(&rules);
        let update = Update { pages: vec![2, 1] };
        assert!(!update.check(&ruleset));
    }
    #[test]
    fn test_check_pass() {
        let rules = vec![Rule { left: 1, right: 2 }];
        let ruleset = Ruleset::new(&rules);
        let update = Update { pages: vec![1, 2] };
        assert!(update.check(&ruleset));
    }
    #[test]
    fn test_check_fail2() {
        let rules = vec![Rule { left: 1, right: 2 }, Rule { left: 1, right: 3 }];
        let ruleset = Ruleset::new(&rules);
        let update = Update { pages: vec![3, 1] };
        assert!(!update.check(&ruleset));
    }
}
