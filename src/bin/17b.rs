use std::str::FromStr;

enum Instruction {
    Adv(usize),
    Bxl(usize),
    Bst(usize),
    Jnz(usize),
    Bxc(usize),
    Out(usize),
    Bdv(usize),
    Cdv(usize),
}

impl Instruction {
    fn new(opcode: usize, operand: usize) -> Self {
        match opcode {
            0 => Instruction::Adv(operand),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(operand),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc(operand),
            5 => Instruction::Out(operand),
            6 => Instruction::Bdv(operand),
            7 => Instruction::Cdv(operand),
            _ => panic!(),
        }
    }
}

struct Computer {
    registers: [usize; 3],
    instruction_pointer: usize,
}

impl Computer {
    fn run_instruction(&mut self, program: &[usize]) -> Option<usize> {
        let instruction = Instruction::new(
            program[self.instruction_pointer],
            program[self.instruction_pointer + 1],
        );
        self.instruction_pointer += 2;
        match instruction {
            Instruction::Adv(x) => self.adv(x),
            Instruction::Bxl(x) => self.bxl(x),
            Instruction::Bst(x) => self.bst(x),
            Instruction::Jnz(x) => self.jnz(x),
            Instruction::Bxc(x) => self.bxc(x),
            Instruction::Out(x) => self.out(x),
            Instruction::Bdv(x) => self.bdv(x),
            Instruction::Cdv(x) => self.cdv(x),
        }
    }
    fn resolve_combo(&self, combo: usize) -> usize {
        match combo {
            0..=3 => combo,
            4..=6 => self.registers[combo - 4],
            _ => panic!(),
        }
    }
    fn adv(&mut self, combo: usize) -> Option<usize> {
        let combo: usize = self.resolve_combo(combo);
        self.registers[0] /= 2_usize.pow(combo as u32);
        None
    }
    fn bxl(&mut self, literal: usize) -> Option<usize> {
        self.registers[1] ^= literal;
        None
    }
    fn bst(&mut self, combo: usize) -> Option<usize> {
        let combo: usize = self.resolve_combo(combo);
        self.registers[1] = combo % 8;
        None
    }
    fn jnz(&mut self, literal: usize) -> Option<usize> {
        if self.registers[0] != 0 {
            self.instruction_pointer = literal;
        }
        None
    }
    fn bxc(&mut self, _literal: usize) -> Option<usize> {
        self.registers[1] ^= self.registers[2];
        None
    }
    fn out(&mut self, combo: usize) -> Option<usize> {
        Some(self.resolve_combo(combo) % 8)
    }
    fn bdv(&mut self, combo: usize) -> Option<usize> {
        let combo: usize = self.resolve_combo(combo);
        self.registers[1] = self.registers[0] / 2_usize.pow(combo as u32);
        None
    }
    fn cdv(&mut self, combo: usize) -> Option<usize> {
        let combo: usize = self.resolve_combo(combo);
        self.registers[2] = self.registers[0] / 2_usize.pow(combo as u32);
        None
    }
    fn halted(&self, program: &[usize]) -> bool {
        self.instruction_pointer >= program.len()
    }
}

#[derive(Debug)]
struct Puzzle {
    registers: [usize; 3],
    program: Vec<usize>,
}

impl FromStr for Puzzle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let read_register = |line: &str| {
            let (_, line) = line.split_once(": ").unwrap();
            line.parse::<usize>().unwrap()
        };
        let registers = [
            read_register(lines[0]),
            read_register(lines[1]),
            read_register(lines[2]),
        ];
        let program = lines[4];
        let (_, program) = program.split_once(": ").unwrap();
        let program = program
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Ok(Puzzle { registers, program })
    }
}

impl Puzzle {
    fn try_reg_a(&self, reg_a: usize) -> bool {
        let mut computer = Computer {
            registers: [reg_a, self.registers[1], self.registers[2]],
            instruction_pointer: 0,
        };
        let mut expected = self.program.iter();
        while !computer.halted(&self.program) {
            if let Some(o) = computer.run_instruction(&self.program) {
                match expected.next() {
                    Some(next) => {
                        if next != &o {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                }
            }
        }
        expected.next().is_none()
    }
    fn process(&self) -> usize {
        let mut a = 0;
        while !self.try_reg_a(a) {
            a += 1;
        }
        a
    }
}

fn main() {
    let puzzle = include_str!("17.txt").parse::<Puzzle>().unwrap();
    let out = puzzle.process();
    println!("{out}");
    // assert_eq!(out, )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_b() {
        let out = include_str!("17_test_b.txt").parse::<Puzzle>().unwrap();
        dbg!(&out);
        let out = out.process();
        assert_eq!(out, 117440);
    }
    // If register C contains 9, the program 2,6 would set register B to 1.
    #[test]
    fn test_1_bst() {
        let mut computer = Computer {
            registers: [0, 0, 9],
            instruction_pointer: 0,
        };
        let program = vec![2, 6];
        computer.run_instruction(&program);
        assert_eq!(computer.registers[1], 1);
    }

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    #[test]
    fn test_2_out() {
        let mut computer = Computer {
            registers: [10, 0, 0],
            instruction_pointer: 0,
        };
        let program = vec![5, 0, 5, 1, 5, 4];
        assert_eq!(computer.run_instruction(&program), Some(0));
        assert_eq!(computer.run_instruction(&program), Some(1));
        assert_eq!(computer.run_instruction(&program), Some(2));
    }

    // If register B contains 29, the program 1,7 would set register B to 26.
    #[test]
    fn test_4_bxl() {
        let mut computer = Computer {
            registers: [0, 29, 0],
            instruction_pointer: 0,
        };
        let program = vec![1, 7];
        computer.run_instruction(&program);
        assert_eq!(computer.registers[1], 26);
    }
}
