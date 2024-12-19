use std::{
    env::args,
    fs::read_to_string,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum Instruction {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

impl From<u64> for Instruction {
    fn from(item: u64) -> Self {
        match item {
            0 => Instruction::adv,
            1 => Instruction::bxl,
            2 => Instruction::bst,
            3 => Instruction::jnz,
            4 => Instruction::bxc,
            5 => Instruction::out,
            6 => Instruction::bdv,
            7 => Instruction::cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RegisterKey {
    A,
    B,
    C,
}

#[derive(Debug, Clone)]
pub struct Register {
    reg: [u64; 3],
}

impl Index<RegisterKey> for Register {
    type Output = u64;

    fn index(&self, index: RegisterKey) -> &Self::Output {
        match index {
            RegisterKey::A => &self.reg[0],
            RegisterKey::B => &self.reg[1],
            RegisterKey::C => &self.reg[2],
        }
    }
}

impl IndexMut<RegisterKey> for Register {
    fn index_mut(&mut self, index: RegisterKey) -> &mut Self::Output {
        match index {
            RegisterKey::A => &mut self.reg[0],
            RegisterKey::B => &mut self.reg[1],
            RegisterKey::C => &mut self.reg[2],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pointer: usize,
    register: Register,
    instructions: Vec<u64>,
    output: Vec<u64>,
}

impl Program {
    fn next(&mut self) {
        self.pointer += 2;
    }

    fn instruction(&self) -> Instruction {
        self.instructions[self.pointer].into()
    }

    fn litteral_operand(&self) -> u64 {
        self.instructions[self.pointer + 1]
    }

    fn combo_operand(&self) -> u64 {
        match self.instructions[self.pointer + 1] {
            7 => unreachable!(),
            6 => self.register[RegisterKey::C],
            5 => self.register[RegisterKey::B],
            4 => self.register[RegisterKey::A],
            i => i,
        }
    }

    fn execute(&mut self) {
        let mut skip_next = false;
        match self.instruction() {
            Instruction::adv => self.register[RegisterKey::A] /= 1 << self.combo_operand(),
            Instruction::bxl => self.register[RegisterKey::B] ^= self.litteral_operand(),
            Instruction::bst => self.register[RegisterKey::B] = self.combo_operand() % 8,
            Instruction::jnz => {
                if self.register[RegisterKey::A] != 0 {
                    self.pointer = self.litteral_operand() as usize;
                    skip_next = true;
                }
            }
            Instruction::bxc => self.register[RegisterKey::B] ^= self.register[RegisterKey::C],
            Instruction::out => self.output.push(self.combo_operand() % 8),
            Instruction::bdv => self.register[RegisterKey::B] = self.register[RegisterKey::A] / (1 << self.combo_operand()),
            Instruction::cdv => self.register[RegisterKey::C] = self.register[RegisterKey::A] / (1 << self.combo_operand()),
        }
        if !skip_next {
            self.next();
        }
    }

    fn halt(&self) -> bool {
        self.pointer >= self.instructions.len()
    }

    pub fn run(&mut self) {
        while !self.halt() {
            self.execute();
        }
    }

    pub fn output(&self) -> String {
        self.output
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.split(':').nth(1).unwrap().trim());
        let a = lines.next().unwrap().parse().unwrap();
        let b = lines.next().unwrap().parse().unwrap();
        let c = lines.next().unwrap().parse().unwrap();
        let instructions = lines
            .next()
            .unwrap()
            .split(',')
            .map(|i| i.parse().unwrap())
            .collect();

        Ok(Program {
            pointer: 0,
            register: Register { reg: [a, b, c] },
            instructions,
            output: vec![],
        })
    }
}

fn answer_part2(program: &Program, answer: &mut [u64], depth: usize) -> bool {
    let len = answer.len();

    if depth == len {
        return false;
    }

    for d in 0..8 {
        answer[depth] = d;
        let mut p = program.clone();
        p.register[RegisterKey::A] = answer.iter().fold(0, |acc, &v| acc * 8 + v);
        p.run();
        if p.output.len() == p.instructions.len() && p.output.iter().rev().nth(depth).unwrap() == p.instructions.iter().rev().nth(depth).unwrap() {
            if p.output == p.instructions || answer_part2(&program, answer, depth + 1) {
                return true;
            }
        }
    }
    return false;
}

fn main() {
    let filename = args().nth(1).unwrap_or_else(|| "example".to_string());
    let program = Program::from_str(&read_to_string(filename).unwrap()).unwrap();

    let mut p1 = program.clone();
    p1.run();

    println!("Part 1: {}", p1.output());

    let mut part2 = vec![0; program.instructions.len()];
    answer_part2(&program, &mut part2, 0);

    println!("Part 2: {}", part2.iter().fold(0, |acc, &v| acc * 8 + v));

//    let mut p = program.clone(); p.register[RegisterKey::A] = [7, 4, 2, 1, 1, 1, 5, 0, 3, 6, 6, 0, 1, 6, 3, 3].iter().fold(0, |acc, &v| acc * 8 + v); p.run(); println!("3 => {}", p.output());
//    println!("Part 2: {}", [7, 4, 2, 1, 1, 1, 5, 0, 3, 6, 6, 0, 1, 6, 3, 3].iter().fold(0i64, |acc, &v| acc * 8 + v));
}

#[cfg(test)]
mod test {
    use super::{Program, Register, RegisterKey};

    #[test]
    fn test1() {
        let mut program = Program {
            pointer: 0,
            register: Register { reg: [0, 0, 9] },
            instructions: vec![2, 6],
            output: vec![],
        };

        program.run();

        assert!(program.register[RegisterKey::B] == 1);
    }

    #[test]
    fn test2() {
        let mut program = Program {
            pointer: 0,
            register: Register { reg: [10, 0, 0] },
            instructions: vec![5, 0, 5, 1, 5, 4],
            output: vec![],
        };

        program.run();

        assert!(program.output() == "0,1,2");
    }

    #[test]
    fn test3() {
        let mut program = Program {
            pointer: 0,
            register: Register { reg: [2024, 0, 0] },
            instructions: vec![0, 1, 5, 4, 3, 0],
            output: vec![],
        };

        program.run();

        assert!(program.output() == "4,2,5,6,7,7,7,7,3,1,0");
        assert!(program.register[RegisterKey::A] == 0);
    }

    #[test]
    fn test4() {
        let mut program = Program {
            pointer: 0,
            register: Register { reg: [0, 29, 0] },
            instructions: vec![1, 7],
            output: vec![],
        };

        program.run();

        assert!(program.register[RegisterKey::B] == 26);
    }

    #[test]
    fn test5() {
        let mut program = Program {
            pointer: 0,
            register: Register {
                reg: [0, 2024, 43690],
            },
            instructions: vec![4, 0],
            output: vec![],
        };

        program.run();

        assert!(program.register[RegisterKey::B] == 44354);
    }
}
