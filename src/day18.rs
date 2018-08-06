use std::fs;
use std::env;
use std::io::{Error, ErrorKind};

type Reg = u8;
type Int = i64;

enum Op {
    Reg(Reg),
    Int(Int),
}

impl Op {
    fn read(&self, regs: &[Int; 26]) -> Int {
        match self {
            Op::Reg(reg) => regs[*reg as usize],
            Op::Int(int) => *int,
        }
    }
}

enum Inst {
    SND(Op),
    SET(Reg, Op),
    ADD(Reg, Op),
    MUL(Reg, Op),
    MOD(Reg, Op),
    RCV(Op),
    JGZ(Op, Op),
}

fn parse_op<'a, I>(tokens : &mut I) -> Result<Op, Error>
    where I: Iterator<Item = &'a str> {

    // Take a token
    let token = tokens.next().ok_or_else(||
        Error::new(ErrorKind::InvalidData, "Expected operand, found EOL"))?;

    // Try and parse a letter (register) first
    if token.len() == 1 {
        // (Ab)use from_str_radix to convert letters to numeric
        if let Ok(base36) = u8::from_str_radix(token, 36) {
            if base36 >= 10 {
                return Ok(Op::Reg(base36 - 10));
            }
        }
    }

    // Otherwise, try and parse as an integer
    if let Ok(int) = token.parse() {
        return Ok(Op::Int(int))
    }

    Err(Error::new(ErrorKind::InvalidData, format!("Expected operand, found {}", token)))
}

fn parse_reg<'a, I>(tokens : &mut I) -> Result<Reg, Error>
    where I: Iterator<Item = &'a str> {

    // Parse an operand, expecting a register
    match parse_op(tokens)? {
        Op::Reg(reg) => Ok(reg),
        Op::Int(int) => Err(Error::new(ErrorKind::InvalidData,
            format!("Expected register, found {}", int))),
    }
}

fn part1(input: &str) -> Result<Int, Error> {
    let mut program = Vec::new();

    // Parse instructions line-by-line
    for line in input.trim().lines() {
        let mut tokens = line.trim().split_whitespace();

        // Read the instruction name
        let inst = tokens.next().ok_or_else(||
            Error::new(ErrorKind::InvalidData, "Expected instruction, found EOL"))?;

        // Parse the instruction and expected operands
        program.push(match inst {
            "snd" => Inst::SND(parse_op(&mut tokens)?),
            "set" => Inst::SET(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "add" => Inst::ADD(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "mul" => Inst::MUL(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "mod" => Inst::MOD(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "rcv" => Inst::RCV(parse_op(&mut tokens)?),
            "jgz" => Inst::JGZ(parse_op(&mut tokens)?, parse_op(&mut tokens)?),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                format!("Expected instruction, found {}", inst))),
        })
    }

    // Make program immutable and get the length
    let program = program;
    let len = program.len() as Int;

    // Initialize registers
    let mut pc : Int = 0;
    let mut regs = [0; 26];
    let mut snd = 0;

    // Execute program
    while pc >= 0 && pc < len {
        match &program[pc as usize] {
            Inst::SND(op) => { snd = op.read(&regs);},
            Inst::SET(reg, op) => { regs[*reg as usize] = op.read(&regs); },
            Inst::ADD(reg, op) => { regs[*reg as usize] += op.read(&regs); },
            Inst::MUL(reg, op) => { regs[*reg as usize] *= op.read(&regs); },
            Inst::MOD(reg, op) => { regs[*reg as usize] %= op.read(&regs); },
            Inst::RCV(op) => {
                if op.read(&regs) != 0 {
                    return Ok(snd);
                }
            },
            Inst::JGZ(op1, op2) => {
                if op1.read(&regs) > 0 {
                    pc += op2.read(&regs);
                    continue;
                }
            }
        }

        pc += 1;
    }

    Err(Error::new(ErrorKind::Other, "Never reached RCV instruction"))
}

#[test]
fn test_day18_part1() {
    let input =
        "set a 1
        add a 2
        mul a a
        mod a 5
        snd a
        set a 0
        rcv a
        jgz a -1
        set a 1
        jgz a -2";
    assert_eq!(part1(&input).ok(), Some(4));
}

pub fn day18(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day18.txt".to_string());
        fs::read_to_string(name)?
    };

    println!("Part 1: {}", part1(&input)?);

    Ok(())
}
