use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Gate {
    inputs: [String; 2],
    output: String,
    operation: Operation,
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.inputs[0], self.operation, self.inputs[1], self.output
        )
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
    gates: Vec<Gate>,
    swapped: Vec<String>,
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
        let gates = lines[(empty_i + 1)..]
            .iter()
            .map(|x| x.parse::<Gate>().unwrap())
            .collect();
        Ok(Puzzle {
            gates,
            swapped: Vec::new(),
        })
    }
}

impl Puzzle {
    fn process(&mut self) -> String {
        self.swapped.sort();
        self.swapped.join(",")
    }
    fn perform_swap(&mut self, a: &str, b: &str) {
        self.swapped.push(a.to_string());
        self.swapped.push(b.to_string());
        let mut i_a: Option<usize> = None;
        let mut i_b: Option<usize> = None;
        for (i, gate) in self.gates.iter().enumerate() {
            if gate.output == a {
                i_a = Some(i);
            }
            if gate.output == b {
                i_b = Some(i);
            }
        }
        self.gates[i_a.unwrap()].output = String::from(b);
        self.gates[i_b.unwrap()].output = String::from(a);
    }
    fn check(&self) {
        // let mut puzzle = include_str!("24.txt").parse::<Puzzle>().unwrap();

        let mut swapped: Vec<&str> = Vec::new();
        let mut gates: HashMap<String, &Gate> = HashMap::new();
        {
            // find sum_00 and carry_00
            let carry = String::from("carry_00");
            let sum = String::from("sum_00");
            for gate in &self.gates {
                if gate.inputs[0].contains("00") || gate.inputs[1].contains("00") {
                    match gate.operation {
                        Operation::Or => {
                            println!("### num input OR operator: {}", gate);
                            panic!();
                        }
                        Operation::And => match gates.entry(carry.clone()) {
                            Entry::Occupied(_occupied_entry) => panic!(),
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(gate);
                                println!("{} // {}", gate, &carry);
                            }
                        },
                        Operation::Xor => match gates.entry(sum.clone()) {
                            Entry::Occupied(_occupied_entry) => panic!(),
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(gate);
                                println!("{} // {}", gate, &sum);
                            }
                        },
                    }
                }
            }
            println!();
        }
        // find full adders
        for num in 1..=44 {
            let num_string = format!("{:02}", num);
            let carry_right = format!("carry_right_{}", num_string);
            let pre_sum = format!("pre_sum_{}", num_string);
            let input_carry = format!("carry_{:02}", num - 1);
            let mut input_carry_label = None;
            match gates.get(&input_carry) {
                Some(input_carry_gate) => {
                    input_carry_label = Some(&input_carry_gate.output);
                }
                None => {
                    println!("missing {}", input_carry);
                }
            }
            for gate in &self.gates {
                if gate.inputs[0].contains(&num_string) || gate.inputs[1].contains(&num_string) {
                    match gate.operation {
                        Operation::Or => {
                            println!("### num input OR operator: {}", gate);
                            panic!();
                        }
                        Operation::And => match gates.entry(carry_right.clone()) {
                            Entry::Occupied(_occupied_entry) => panic!(),
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(gate);
                                println!("{} // {}", gate, &carry_right);
                            }
                        },
                        Operation::Xor => match gates.entry(pre_sum.clone()) {
                            Entry::Occupied(_occupied_entry) => panic!(),
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(gate);
                                println!("{} // {}", gate, &pre_sum);
                            }
                        },
                    }
                }
            }
            assert!(gates.contains_key(&pre_sum));
            assert!(gates.contains_key(&carry_right));

            let pre_sum_label = &gates[&pre_sum].output;
            let carry_left = format!("carry_left_{}", num_string);
            let sum = format!("sum_{}", num_string);
            for gate in &self.gates {
                if &gate.inputs[0] == pre_sum_label || &gate.inputs[1] == pre_sum_label {
                    if &gate.inputs[0] != input_carry_label.unwrap()
                        && &gate.inputs[1] != input_carry_label.unwrap()
                    {
                        println!("### Problem detected with gate: {}", gate);
                        let labeled: HashSet<&Gate> = gates.values().map(|x| *x).collect();
                        for gate in &self.gates {
                            if !labeled.contains(gate) {
                                println!("{}", gate);
                            }
                        }
                        panic!();
                    }
                    match gate.operation {
                        Operation::And => {
                            // carry left
                            match gates.entry(carry_left.clone()) {
                                Entry::Occupied(_occupied_entry) => panic!(),
                                Entry::Vacant(vacant_entry) => {
                                    vacant_entry.insert(gate);
                                    println!("{} // {}", gate, &carry_left);
                                }
                            }
                        }
                        Operation::Xor => match gates.entry(sum.clone()) {
                            Entry::Occupied(_occupied_entry) => panic!(),
                            Entry::Vacant(vacant_entry) => {
                                vacant_entry.insert(gate);
                                println!("{} // {}", gate, &sum);
                                if !gate.output.contains(&num_string) {
                                    swapped.push(&gate.output);
                                }
                            }
                        },
                        _ => {
                            swapped.push(&gate.output);
                        }
                    }
                }
            }

            // extra check for sum output
            for gate in &self.gates {
                if gate.output.contains(&num_string) {
                    match gate.operation {
                        Operation::Xor => (),
                        _ => {
                            swapped.push(&gate.output);
                        }
                    }
                }
            }

            let carry = format!("carry_{}", num_string);
            let carry_left_label: &str = &gates[&carry_left].output;
            let carry_right_label: &str = &gates[&carry_right].output;
            for gate in &self.gates {
                if gate.inputs[0] == carry_left_label || gate.inputs[1] == carry_left_label {
                    assert!(
                        gate.inputs[0] == carry_right_label || gate.inputs[1] == carry_right_label
                    );
                    match gate.operation {
                        Operation::Or => {
                            // carry
                            match gates.entry(carry.clone()) {
                                Entry::Occupied(_occupied_entry) => panic!(),
                                Entry::Vacant(vacant_entry) => {
                                    vacant_entry.insert(gate);
                                    println!("{} // {}", gate, &carry);
                                }
                            }
                        }
                        _ => panic!(),
                    }
                }
            }
            println!();
        }
        for swapped_name in &swapped {
            println!("swapped: {}", swapped_name);
        }
    }
}

fn main() {
    let mut puzzle = include_str!("24.txt").parse::<Puzzle>().unwrap();
    puzzle.perform_swap("rts", "z07");
    puzzle.perform_swap("jpj", "z12");
    puzzle.perform_swap("kgj", "z26");
    puzzle.perform_swap("vvw", "chv");
    puzzle.check();
    let out = puzzle.process();
    println!("{}", out);
    assert_eq!(out, "chv,jpj,kgj,rts,vvw,z07,z12,z26");
}
