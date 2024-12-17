use std::fs;

use aoc2024::{get_input_file, MainResult};

fn main() -> MainResult {
    let file_path = get_input_file(17, false);
    let data = fs::read_to_string(&file_path)?;

    let (registers, program) = {
        let (registers, program) = data.split_once("\n\n").unwrap();
        let registers = registers
            .lines()
            .map(|r| {
                let (start, end) = r.split_once(":").unwrap();
                let name = start.chars().last().unwrap();
                let value = end.trim().parse::<u64>().unwrap();
                (name, value)
            })
            .collect::<Vec<_>>();
        let (_, program) = program.split_once(":").unwrap();
        let program = program
            .trim()
            .split(",")
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<Vec<_>>();
        (registers, program)
    };
    let mut runner = Runner::new(&[registers[0].1, registers[1].1, registers[2].1], program);
    runner.run();

    println!("output:\n{}", runner.output());

    Ok(())
}

struct Runner {
    program: Vec<u8>,
    registers: [u64; 3],
    output: Vec<u64>,

    ip: usize,
}

impl Runner {
    fn new(registers: &[u64; 3], program: Vec<u8>) -> Self {
        Self {
            program,
            registers: *registers,
            output: vec![],
            ip: 0,
        }
    }

    fn run(&mut self) {
        loop {
            let ip = self.execute_instruction();
            self.ip = ip;
            if ip >= self.program.len() {
                return;
            }
        }
    }

    fn execute_instruction(&mut self) -> usize {
        let instruction = Instruction::from(self.program[self.ip]);
        let operand = self.program[self.ip + 1];
        match instruction {
            Instruction::Adv => {
                let operand = self.as_combo(operand);
                let a = self.register_a_mut();
                *a = *a >> operand;
                self.ip + 2
            }
            Instruction::Bxl => {
                let b = self.register_b_mut();
                *b = *b ^ operand as u64;
                self.ip + 2
            }
            Instruction::Bst => {
                let operand = self.as_combo(operand);
                let b = self.register_b_mut();
                *b = operand & 0x07;
                self.ip + 2
            }
            Instruction::Jnz => {
                let a = self.register_a();
                if a > 0 {
                    operand as usize
                } else {
                    self.ip + 2
                }
            }
            Instruction::Bxc => {
                let c = self.register_c();
                let b = self.register_b_mut();
                *b = *b ^ c;
                self.ip + 2
            }
            Instruction::Out => {
                self.output.push(self.as_combo(operand) & 0x07);
                self.ip + 2
            }
            Instruction::Bdv => {
                let operand = self.as_combo(operand);
                let a = self.register_a();
                let b = self.register_b_mut();
                *b = a >> operand;
                self.ip + 2
            }
            Instruction::Cdv => {
                let operand = self.as_combo(operand);
                let a = self.register_a();
                let c = self.register_c_mut();
                *c = a >> operand;
                self.ip + 2
            }
        }
    }

    fn register_a_mut(&mut self) -> &mut u64 {
        &mut self.registers[0]
    }

    fn register_b_mut(&mut self) -> &mut u64 {
        &mut self.registers[1]
    }

    fn register_c_mut(&mut self) -> &mut u64 {
        &mut self.registers[2]
    }

    fn register_a(&self) -> u64 {
        self.registers[0]
    }

    fn register_b(&self) -> u64 {
        self.registers[1]
    }

    fn register_c(&self) -> u64 {
        self.registers[2]
    }

    fn as_combo(&self, operand: u8) -> u64 {
        match operand {
            op if op <= 3 => op as u64,
            4 => self.register_a(),
            5 => self.register_b(),
            6 => self.register_c(),
            _ => panic!("invalid combo operand"),
        }
    }

    fn output(&self) -> String {
        self.output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    const fn from(opcode: u8) -> Self {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode"),
        }
    }
}
