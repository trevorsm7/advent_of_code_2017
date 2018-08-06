use std::fs;
use std::env;
use std::io::{Error, ErrorKind};
use std::collections::VecDeque;

type Reg = u8;
type Int = i64;

#[derive(Copy, Clone, PartialEq)]
enum Op {
    Reg(Reg),
    Int(Int),
}

#[derive(Copy, Clone, PartialEq)]
enum Inst {
    SND(Op),
    SET(Reg, Op),
    ADD(Reg, Op),
    MUL(Reg, Op),
    MOD(Reg, Op),
    RCV(Reg),
    JGZ(Op, Op),
}

type Program = Vec<Inst>;

fn parse_op<'a, I>(tokens: &mut I) -> Result<Op, Error>
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

fn parse_reg<'a, I>(tokens: &mut I) -> Result<Reg, Error>
    where I: Iterator<Item = &'a str> {

    // Parse an operand, expecting a register
    match parse_op(tokens)? {
        Op::Reg(reg) => Ok(reg),
        Op::Int(int) => Err(Error::new(ErrorKind::InvalidData,
            format!("Expected register, found {}", int))),
    }
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
            "snd" => Inst::SND(parse_op(&mut tokens)?),
            "set" => Inst::SET(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "add" => Inst::ADD(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "mul" => Inst::MUL(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "mod" => Inst::MOD(parse_reg(&mut tokens)?, parse_op(&mut tokens)?),
            "rcv" => Inst::RCV(parse_reg(&mut tokens)?),
            "jgz" => Inst::JGZ(parse_op(&mut tokens)?, parse_op(&mut tokens)?),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                format!("Expected instruction, found {}", inst))),
        })
    }

    Ok(program)
}

struct Machine {
    pc: Int,
    regs: [Int; 26],
    send: VecDeque<Int>,
}

impl Machine {
    const P: usize = ('p' as u8 - 'a' as u8) as usize;

    fn new() -> Self {
        Self {
            pc: 0,
            regs: [0; 26],
            send: VecDeque::new(),
        }
    }

    fn with_pid(pid: Int) -> Self {
        let mut machine = Self::new();
        machine.regs[Self::P] = pid;
        machine
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

    fn send(&mut self, op: Op) {
        // See this link for why a temp is required here; this may be fixed in the future!
        // https://internals.rust-lang.org/t/accepting-nested-method-calls-with-an-mut-self-receiver/4588
        let value = self.read(op);
        self.send.push_back(value);
    }

    fn run_yield(&mut self, program: &Program) -> Result<Option<Reg>, Error> {
        let len = program.len() as Int;

        // Execute while the pc is valid
        while self.pc >= 0 && self.pc < len {
            match &program[self.pc as usize] {
                Inst::SND(op) => self.send(*op),
                Inst::SET(reg, op) => *self.rw(*reg) = self.read(*op),
                Inst::ADD(reg, op) => *self.rw(*reg) += self.read(*op),
                Inst::MUL(reg, op) => *self.rw(*reg) *= self.read(*op),
                Inst::MOD(reg, op) => *self.rw(*reg) %= self.read(*op),
                Inst::RCV(reg) => return Ok(Some(*reg)), // yield, returning register
                Inst::JGZ(op1, op2) => {
                    if self.read(*op1) > 0 {
                        self.pc += self.read(*op2);
                        continue;
                    }
                }
            }

            self.pc += 1;
        }

        // Return none if the program terminates without yielding
        Ok(None)
    }
}

fn part1(program: &Program) -> Result<Int, Error> {
    let mut machine = Machine::new();

    if let None = machine.run_yield(&program)? {
        return Err(Error::new(ErrorKind::Other, "Never reached RCV instruction"));
    }

    if let Some(last_snd) = machine.send.back() {
        return Ok(*last_snd);
    }

    Err(Error::new(ErrorKind::Other, "Never reached a SND instruction"))
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

    let result = || -> Result<(), Error> {
        let program = parse_program(&input)?;
        assert_eq!(part1(&program)?, 4);
        Ok(())
    }();

    if let Err(e) = result {
        panic!(format!("{}", e));
    }
}

pub fn day18(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day18.txt".to_string());
        fs::read_to_string(name)?
    };

    let program = parse_program(&input)?;
    println!("Part 1: {}", part1(&program)?);

    Ok(())
}
