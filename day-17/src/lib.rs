pub mod part1;
pub mod part2;

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, newline, space1},
    combinator::{map_res, peek},
    multi::{count, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

pub type RegisterA = isize;
pub type RegisterB = isize;
pub type RegisterC = isize;
pub type Operand = isize;
pub type InstructionPointer = usize;
pub type Output = Option<isize>;

#[derive(Debug)]
pub struct Program {
    pub a: RegisterA,
    pub b: RegisterB,
    pub c: RegisterC,
    pub program: Vec<(Instruction, Operand)>,
    pub program_raw: Vec<isize>,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Computer {
    pub a: isize,
    pub b: isize,
    pub c: isize,
    pub instruction_pointer: usize,
    pub program_raw: Vec<isize>,
}

pub fn parse_opcode(input: &str) -> IResult<&str, Instruction> {
    map_res(digit1, |d: &str| match d {
        "0" => Ok(Instruction::Adv),
        "1" => Ok(Instruction::Bxl),
        "2" => Ok(Instruction::Bst),
        "3" => Ok(Instruction::Jnz),
        "4" => Ok(Instruction::Bxc),
        "5" => Ok(Instruction::Out),
        "6" => Ok(Instruction::Bdv),
        "7" => Ok(Instruction::Cdv),
        opcode => Err(format!("unexpected opcode: {}", opcode)),
    })(input)
}

fn parse_register(input: &str) -> IResult<&str, isize> {
    let (input, _) = tag("Register ")(input)?;
    let (input, _) = alpha1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, register) = map_res(digit1, |d: &str| d.parse::<isize>())(input)?;
    Ok((input, register))
}

fn parse_operand(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |d: &str| d.parse::<isize>())(input)
}

fn parse_program(input: &str) -> IResult<&str, Vec<(Instruction, Operand)>> {
    let (input, _) = tag("Program: ")(input)?;
    separated_list1(
        tag(","),
        separated_pair(parse_opcode, tag(","), parse_operand),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Program> {
    let (input, a) = parse_register(input)?;
    let (input, _) = multispace0(input)?;
    let (input, b) = parse_register(input)?;
    let (input, _) = multispace0(input)?;
    let (input, c) = parse_register(input)?;
    let (input, _) = multispace0(input)?;
    let (input, program) = peek(parse_program)(input)?;
    let (input, program_raw) = preceded(
        tag("Program: "),
        separated_list1(tag(","), map_res(digit1, |d: &str| d.parse::<isize>())),
    )(input)?;
    Ok((
        input,
        Program {
            a,
            b,
            c,
            program,
            program_raw,
        },
    ))
}

pub fn parse_input(input: &str) -> Program {
    parse(input).unwrap().1
}

pub fn run_instruction(
    computer: &mut Computer,
    instruction: Instruction,
    operand: Operand,
) -> Output {
    let combo_operand = match operand {
        4 => computer.a,
        5 => computer.b,
        6 => computer.c,
        _ => operand,
    };
    match instruction {
        Instruction::Adv => {
            let numerator = computer.a;
            let denominator = 2isize.pow(combo_operand as u32);
            computer.a = numerator / denominator;
            computer.instruction_pointer += 1;
            None
        }
        Instruction::Bxl => {
            computer.b ^= operand;
            computer.instruction_pointer += 1;
            None
        }
        Instruction::Bst => {
            computer.b = combo_operand % 8;
            computer.instruction_pointer += 1;
            None
        }
        Instruction::Jnz => {
            if computer.a != 0 {
                computer.instruction_pointer = operand as usize;
            } else {
                computer.instruction_pointer += 1;
            }
            None
        }
        Instruction::Bxc => {
            computer.b ^= computer.c;
            computer.instruction_pointer += 1;
            None
        }
        Instruction::Out => {
            computer.instruction_pointer += 1;
            Some(combo_operand % 8)
        }
        Instruction::Bdv => {
            let numerator = computer.a;
            let denominator = 2isize.pow(combo_operand as u32);
            computer.b = numerator / denominator;
            computer.instruction_pointer += 1;
            None
        }
        Instruction::Cdv => {
            let numerator = computer.a;
            let denominator = 2isize.pow(combo_operand as u32);
            computer.c = numerator / denominator;
            computer.instruction_pointer += 1;
            None
        }
    }
}

pub fn run_program(program: &Program) -> Vec<isize> {
    let mut computer = Computer {
        a: program.a,
        b: program.b,
        c: program.c,
        instruction_pointer: 0,
        program_raw: program.program_raw.clone(),
    };
    let mut output = vec![];
    while computer.instruction_pointer < program.program.len() {
        let (instruction, operand) = &program.program[computer.instruction_pointer];
        if let Some(o) = run_instruction(&mut computer, *instruction, *operand) {
            output.push(o);
        }
    }
    output
}
