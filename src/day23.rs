use std::{fs, env};
use std::io::{Error, ErrorKind};

use super::day18::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Inst {
    SET(Reg, Op),
    SUB(Reg, Op),
    MUL(Reg, Op),
    JNZ(Op, Op),
}

type Program = Vec<Inst>;

struct Machine {
    pc: Int,
    regs: [Int; Machine::REG_COUNT],
    mul_count: u32,
}

pub fn day23(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day23.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2());

    Ok(())
}

fn part1(input: &str) -> u32 {
    let program = parse_program(input).unwrap();
    let mut machine = Machine::new();
    machine.run(&program);
    machine.mul_count
}

fn part2() -> i64 {
    // Decompile the given assembly into Rust
    let b = 57 * 100 + 100000;
    let c = b + 17000;

    // Loop from b to c, counting non-primes
    let mut h = 0;
    for n in (b..=c).step_by(17) {
        if !is_prime(n) {
            h += 1;
        }
    }

    h
}

fn is_prime(n: i64) -> bool {
    let b = (n as f64).sqrt().floor() as i64;

    // Replace the given algorithm with a more efficient one
    for a in 2..=b {
        if n % a == 0 {
            return false;
        }
    }

    true
}

#[test]
fn test_day23_is_prime() {
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(6), false);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(8), false);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(10), false);
    assert_eq!(is_prime(11), true);
    assert_eq!(is_prime(12), false);
    assert_eq!(is_prime(13), true);
}

fn parse_program(input: &str) -> Result<Program, Error> {
    let mut program = Program::new();

    // Parse instructions line-by-line
    for line in input.trim().lines() {
        let mut tokens = line.trim().split_whitespace();

        // Read the instruction name
        let inst = tokens.next().ok_or_else(||
            Error::new(ErrorKind::InvalidData, "Expected instruction, found EOL"))?;

        // Parse the instruction and expected operands
        program.push(match inst {
            "set" => Inst::SET(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "sub" => Inst::SUB(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "mul" => Inst::MUL(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "jnz" => Inst::JNZ(parse_op(&mut tokens)?, parse_op(&mut tokens)?),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                format!("Expected instruction, found {}", inst))),
        })
    }

    Ok(program)
}

#[test]
fn test_day23_parse_program() {
    let input = "\
        set a -1\n\
        sub b -2\n\
        mul c -3\n\
        jnz d -4";
    let program = parse_program(input).unwrap();
    assert_eq!(program[0], Inst::SET(0, Op::Int(-1)));
    assert_eq!(program[1], Inst::SUB(1, Op::Int(-2)));
    assert_eq!(program[2], Inst::MUL(2, Op::Int(-3)));
    assert_eq!(program[3], Inst::JNZ(Op::Reg(3), Op::Int(-4)));
}

impl Machine {
    const REG_COUNT: usize = 8;

    fn new() -> Self {
        Self { pc: 0, regs: [0; Self::REG_COUNT], mul_count: 0 }
    }

    fn read(&self, op: Op) -> Int {
        match op {
            Op::Reg(reg) => self.regs[reg as usize],
            Op::Int(int) => int,
        }
    }

    fn rw(&mut self, reg: Reg) -> &mut Int {
        &mut self.regs[reg as usize]
    }

    fn run(&mut self, program: &Program) {
        let len = program.len() as Int;
        while self.pc >= 0 && self.pc < len {
            let inst = program[self.pc as usize];

            // Handle jump first
            if let Inst::JNZ(op1, op2) = inst {
                if self.read(op1) != 0 {
                    self.pc += self.read(op2);
                    continue;
                }
            }

            // Otherwise, increment PC by 1
            self.pc += 1;

            // Then handle remaining instructions
            match inst {
                Inst::SET(reg, op) => *self.rw(reg) = self.read(op),
                Inst::SUB(reg, op) => *self.rw(reg) -= self.read(op),
                Inst::MUL(reg, op) => {
                    *self.rw(reg) *= self.read(op);
                    self.mul_count += 1;
                },
                _ => (),
            }
        }
    }
}
