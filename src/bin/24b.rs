use std::{
    collections::{hash_map::Entry, HashMap}, fmt::Display, str::FromStr
};

#[derive(Debug)]
struct Gate {
    inputs: [String; 2],
    output: String,
    operation: Operation,
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}",self.inputs[0],self.operation, self.inputs[1], self.output)
    }
}

#[derive(Debug)]
struct InitialCondition {
    output: String,
    value: bool,
}

#[derive(Debug)]
enum Operation {
    And,
    Or,
    Xor,
}
impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::And => write!(f, "AND"),
            Operation::Or => write!(f, "OR"),
            Operation::Xor => write!(f, "XOR"),
        }
    }
}
impl FromStr for InitialCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (output, value) = s.split_once(": ").unwrap();
        let output = String::from(output);
        let value = match value {
            "0" => false,
            "1" => true,
            _ => panic!(),
        };
        Ok(InitialCondition { output, value })
    }
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, output) = s.split_once(" -> ").unwrap();
        let output = String::from(output);
        let (input0, right) = left.split_once(" ").unwrap();
        let (operation, input1) = right.split_once(" ").unwrap();
        let input0 = String::from(input0);
        let input1 = String::from(input1);
        let operation = match operation {
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            "AND" => Operation::And,
            _ => panic!(),
        };
        Ok(Gate {
            inputs: [input0, input1],
            operation,
            output,
        })
    }
}

#[derive(Debug)]
struct Puzzle {
    initial_conditions: Vec<InitialCondition>,
    gates: Vec<Gate>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let (empty_i, _) = lines
            .iter()
            .enumerate()
            .find(|(_i, x)| x.is_empty())
            .unwrap();
        let initial_conditions = lines[0..empty_i]
            .iter()
            .map(|line| line.parse::<InitialCondition>().unwrap())
            .collect();
        let gates = lines[(empty_i + 1)..]
            .iter()
            .map(|x| x.parse::<Gate>().unwrap())
            .collect();
        Ok(Puzzle {
            initial_conditions,
            gates,
        })
    }
}

impl Puzzle {
    fn process(&mut self) -> usize {
        let mut outputs: HashMap<&str, bool> =
            HashMap::from_iter(self.initial_conditions.iter().map(|ic| {
                let str: &str = &ic.output;
                (str, ic.value)
            }));
        let mut gates: HashMap<&str, &Gate> = HashMap::from_iter(self.gates.iter().map(|g| {
            let str: &str = &g.output;
            (str, g)
        }));
        while !gates.is_empty() {
            let mut new_outputs = Vec::new();
            for (output, gate) in &gates {
                let input0: &str = &gate.inputs[0];
                let input1: &str = &gate.inputs[1];
                if outputs.contains_key(input0) && outputs.contains_key(input1) {
                    let new_output_value = match gate.operation {
                        Operation::And => outputs[input0] && outputs[input1],
                        Operation::Or => outputs[input0] || outputs[input1],
                        Operation::Xor => outputs[input0] ^ outputs[input1],
                    };
                    new_outputs.push((*output, new_output_value));
                }
            }
            if new_outputs.is_empty() {
                break;
            }
            for (output, value) in new_outputs {
                gates.remove(output);
                match outputs.entry(output) {
                    Entry::Occupied(_) => panic!(),
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(value);
                    }
                }
            }
        }
        let mut out = 0;
        for (output, value) in outputs {
            if !output.starts_with("z") || !value {
                continue;
            }
            let (_, num) = output.split_once("z").unwrap();
            let num: u32 = num.parse().unwrap();
            out += 2_usize.pow(num);
        }
        out
    }
}

fn main() {
    let mut puzzle = include_str!("24.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    assert_eq!(out, 56620966442854);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut out = include_str!("24_test.txt").parse::<Puzzle>().unwrap();
        let out = out.process();
        assert_eq!(out, 2024);
    }
    #[test]
    fn show_gates_in_order() {
        let puzzle = include_str!("24.txt").parse::<Puzzle>().unwrap();
        for num in 0..26 {
            let num = format!("{:02}", num);
            for gate in &puzzle.gates {
                if gate.inputs[0].contains(&num) || gate.inputs[1].contains(&num) {
                    println!("{}", gate);
                }
            }
            for gate in &puzzle.gates {
                if gate.output.contains(&num){
                    println!("{}", gate);
                }
            }
            println!();
        }

    }
}
