use std::fs;
use std::env;
use std::io::{Error, ErrorKind};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};

extern crate crossbeam;

// ==== Operands and registers ====

pub(crate) type Reg = u8;
pub(crate) type Int = i64;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum Op {
    Reg(Reg),
    Int(Int),
}

pub(crate) fn parse_op<'a, I>(tokens: &mut I) -> Result<Op, Error>
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

pub(crate) fn parse_reg<'a, I>(tokens: &mut I) -> Result<Reg, Error>
    where I: Iterator<Item = &'a str> {

    // Parse an operand, expecting a register
    match parse_op(tokens)? {
        Op::Reg(reg) => Ok(reg),
        Op::Int(int) => Err(Error::new(ErrorKind::InvalidData,
            format!("Expected register, found {}", int))),
    }
}

// ==== Instructions and programs ====

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

// ==== Virtual machines ====

enum Message {
    Value(Int),
    Blocked(u32), // received count
    Terminated,
}

struct Machine {
    pc: Int,
    regs: [Int; 26],
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    send_count: u32,
    receive_count: u32,
}

impl Machine {
    const P: usize = ('p' as u8 - 'a' as u8) as usize;

    fn with_mpsc(sender: Sender<Message>, receiver: Receiver<Message>) -> Self {
        Self {
            pc: 0,
            regs: [0; 26],
            sender,
            receiver,
            send_count: 0,
            receive_count: 0,
        }
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
        self.send_count += 1;
        // See this link for why a temp is required here; this may be fixed in the future!
        // https://internals.rust-lang.org/t/accepting-nested-method-calls-with-an-mut-self-receiver
        let value = self.read(op);
        let _ = self.sender.send(Message::Value(value));
    }

    fn is_running(&mut self, program: &Program) -> bool {
        let len = program.len() as Int;
        self.pc >= 0 && self.pc < len
    }

    fn run_yielding(&mut self, program: &Program) -> Option<Reg> {
        while self.is_running(&program) {
            let inst = program[self.pc as usize];

            // Handle jump first
            if let Inst::JGZ(op1, op2) = inst {
                if self.read(op1) > 0 {
                    self.pc += self.read(op2);
                    continue;
                }
            }

            // Otherwise, increment PC by 1
            self.pc += 1;

            // Then handle remaining instructions
            match inst {
                Inst::SND(op) => self.send(op),
                Inst::SET(reg, op) => *self.rw(reg) = self.read(op),
                Inst::ADD(reg, op) => *self.rw(reg) += self.read(op),
                Inst::MUL(reg, op) => *self.rw(reg) *= self.read(op),
                Inst::MOD(reg, op) => *self.rw(reg) %= self.read(op),
                Inst::RCV(reg) => return Some(reg), // yield
                _ => (),
            }
        }

        None
    }

    fn run_threaded(&mut self, program: &Program, pid: Int) -> u32 {
        // Reset virtual machine
        self.pc = 0;
        self.send_count = 0;
        self.receive_count = 0;
        self.regs[Self::P] = pid;

        // Run until the first yield
        let mut reg = self.run_yielding(&program);

        // Continue to receive messages until we terminate or deadlock
        while self.is_running(&program) {
            // Receive a message, notifying our partner if we're blocked
            let message = match self.receiver.try_recv() {
                Ok(message) => message,
                Err(TryRecvError::Empty) => {
                    let _ = self.sender.send(Message::Blocked(self.receive_count));
                    self.receiver.recv().unwrap()
                },
                _ => break,
            };

            // Handle the message
            match message {
                Message::Value(val) => {
                    self.receive_count += 1;
                    *self.rw(reg.unwrap()) = val;
                    reg = self.run_yielding(&program);
                },
                Message::Blocked(received) => {
                    // Exit the loop if we're deadlocked
                    if received == self.send_count {
                        break;
                    }
                },
                Message::Terminated => {
                    break;
                }
            }
        }

        // Notify our partner that we're terminating
        let _ = self.sender.send(Message::Terminated);
        self.send_count
    }
}

// ==== Exercises and tests ====

fn part1(program: &Program) -> Option<Int> {
    let (tx, rx) = mpsc::channel();
    let (_, dummy) = mpsc::channel();
    let mut machine = Machine::with_mpsc(tx, dummy);

    if let None = machine.run_yielding(&program) {
        return None;
    }

    if let Some(Message::Value(last_snd)) = rx.try_iter().last() {
        return Some(last_snd);
    }

    None
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

    let program = parse_program(&input).unwrap();
    assert_eq!(part1(&program), Some(4));
}

fn part2(program: &Program) -> u32 {
    // Open a pair of channels
    let (tx0, rx1) = mpsc::channel();
    let (tx1, rx0) = mpsc::channel();

    // Use scoped threads so we gurantee the lifetime of the program reference
    crossbeam::scope(|scope| {
        let _handle0 = scope.spawn(move || Machine::with_mpsc(tx0, rx0).run_threaded(program, 0));
        let handle1 = scope.spawn(move || Machine::with_mpsc(tx1, rx1).run_threaded(program, 1));
        handle1.join().unwrap()
    })
}

#[test]
fn test_day18_part2() {
    let input =
        "snd 1
        snd 2
        snd p
        rcv a
        rcv b
        rcv c
        rcv d";

    let program = parse_program(&input).unwrap();
    assert_eq!(part2(&program), 3);
}

pub fn day18(args: &mut env::Args) -> Result<(), Error> {
    // Read from file in first arg or default to input.txt
    let input = {
        let name = args.next().unwrap_or("input/day18.txt".to_string());
        fs::read_to_string(name)?
    };

    let program = parse_program(&input)?;
    println!("Part 1: {}", part1(&program).unwrap());
    println!("Part 2: {}", part2(&program));

    Ok(())
}
