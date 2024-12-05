use std::{collections::BTreeMap, str::FromStr};

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
        Ok(Rule{left, right})
    }
}
#[derive(Debug)]
struct Update {
    pages: Vec<usize>,
}
impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages:Vec<&str> = s.split(",").collect();
        if pages.is_empty() {
            return Err(())
        }
        let pages = pages.iter().map(|p|p.parse::<usize>()).collect::<Vec<_>>();
        if pages.iter().any(|p|p.is_err()) {
            return Err(());
        }
        let pages: Vec<usize> = pages.iter().map(|p| *p.as_ref().unwrap()).collect();
        Ok(Update{pages})
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
        Ok(Puzzle{rules,updates})
    }
}

impl Puzzle {
    fn process(&self) -> usize {
        let mut ruleset = BTreeMap::new();
        for Rule{left, right} in &self.rules {
            if !ruleset.contains_key(&right) {
                ruleset.insert(right, vec![left]);
            } else {
                ruleset.get_mut(&right).unwrap().push(left);
            }
        }
        dbg!(&ruleset);
        0
    }
}

// fn process(tokens: ArrayView2<Token>) -> usize {
//     let mut out = 0;

//     let shape = tokens.shape();
//     for ((i, j), token) in tokens.indexed_iter() {
//         if i == 0 || j == 0 || i == shape[0] - 1 || j == shape[1] - 1 {
//             continue;
//         }
//         if token == &Token::A && check_location(tokens, i, j) {
//             out += 1;
//         }
//     }
//     out
// }

fn main() {
    let puzzle = include_str!("05_test.txt").parse::<Puzzle>().unwrap();
    dbg!(&puzzle);
    let out = puzzle.process();
    assert_eq!(out, 143);

    // let out = include_str!("04.txt");
    // let out = extract(out);
    // let out = process(out.view());
    // assert_eq!(out, 1815);
    // println!("{out}");
}
