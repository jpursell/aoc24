use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug)]
struct Connection {
    nodes: [String; 2],
}

impl FromStr for Connection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once("-").unwrap();
        Ok(Connection {
            nodes: [a.into(), b.into()],
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    connections: Vec<Connection>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let connections = s
            .lines()
            .map(|line| line.parse::<Connection>().unwrap())
            .collect();
        Ok(Puzzle { connections })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        let mut connection_map: HashMap<&str, HashSet<&str>> = HashMap::new();
        for connection in &self.connections {
            let source: &str = &connection.nodes[0];
            let dest: &str = &connection.nodes[1];
            let mut insert_connection = |source, dest| match connection_map.entry(source) {
                Entry::Occupied(mut occupied_entry) => {
                    (*occupied_entry.get_mut()).insert(dest);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(HashSet::from([dest]));
                }
            };
            insert_connection(source, dest);
            insert_connection(dest, source);
        }
        let mut trios: HashSet<[&str; 3]> = HashSet::new();
        for (&node, connections) in &connection_map {
            for others in connections.iter().combinations(2) {
                if !connection_map[others[0]].contains(others[1]) {
                    continue;
                }
                let mut trio = [node, *others[0], *others[1]];
                trio.sort();
                if trios.contains(&trio) {
                    continue;
                }
                trios.insert(trio);
            }
        }
        trios
            .iter()
            .filter(|trio| trio.iter().any(|node| node.starts_with("t")))
            .count()
    }
}

fn main() {
    let mut puzzle = include_str!("23.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 1083);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("23_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process();
        assert_eq!(out, 7);
    }
}
