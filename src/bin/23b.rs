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
    fn make_connection_map(&self) -> HashMap<&str, HashSet<&str>> {
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
        connection_map
    }
    fn process(&mut self, size: usize) -> Option<String> {
        let connection_map = self.make_connection_map();
        for (key, connections) in &connection_map {
            'combo: for combination in connections.iter().combinations(size - 1) {
                for connection in combination.iter().combinations(2) {
                    if !connection_map[**connection[0]].contains(**connection[1]) {
                        continue 'combo;
                    }
                }
                let mut nodes: Vec<&str> = Vec::with_capacity(size);
                nodes.push(key);
                combination.iter().map(|x| **x).for_each(|x| nodes.push(x));
                nodes.sort();
                return Some(nodes.join(","));
            }
        }
        None
    }
}

fn main() {
    let mut puzzle = include_str!("23.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process(13).unwrap();
    println!("{out}");
    assert_eq!(out, "as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("23_test.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process(4).unwrap();
        assert_eq!(out, "co,de,ka,ta");
    }
    #[test]
    fn test_connections() {
        let out = include_str!("23.txt").parse::<Puzzle>().unwrap();
        for (num, (_key, connections)) in out.make_connection_map().iter().enumerate() {
            if num % 10 == 0 {
                print!("{:03}: ", num);
            }
            print!("{}, ", connections.len());
            if num % 10 == 9 {
                println!()
            }
        }
        println!()
    }
}
