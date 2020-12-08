use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::Challenge;
use itertools::Itertools;

pub struct Day08;

impl Challenge for Day08 {
    const DAY_NUMBER: u32 = 8;

    type InputType = AssemblyEmulator;
    type OutputType = i32;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        let mut ae = input.clone();
        let mut instructions = HashSet::new();

        while !instructions.contains(&ae.program_counter()) {
            instructions.insert(ae.program_counter());
            ae.step();
        }

        Ok(ae.acc())
    }

    // Naive brute-force solution
    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        for (instruction_idx, _) in
            input
                .instructions
                .iter()
                .enumerate()
                .filter(|(_, i)| match i.0 {
                    OpCode::Nop | OpCode::Jmp => true,
                    _ => false,
                })
        {
            if let Ok(fixed_ae) = try_fix(input, instruction_idx) {
                return Ok(fixed_ae.acc());
            }
        }

        Err(anyhow!("Could not find corrupted instruction"))
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        content.parse()
    }
}

fn try_fix(input: &AssemblyEmulator, instruction_idx: usize) -> Result<AssemblyEmulator> {
    let mut fixed_ae: Vec<Instruction> = input.instructions().into();
    let corrupted_instruction = input.instructions()[instruction_idx];

    fixed_ae[instruction_idx].0 = match corrupted_instruction.0 {
        OpCode::Nop => OpCode::Jmp,
        OpCode::Jmp => OpCode::Nop,
        _ => unreachable!(),
    };

    let mut fixed_ae = AssemblyEmulator::new(fixed_ae);
    let mut encounterd_instructions = HashSet::new();

    while fixed_ae.program_counter() != fixed_ae.instructions().len() {
        if encounterd_instructions.contains(&fixed_ae.program_counter()) {
            return Err(anyhow!("The fix doesn't work"));
        }

        encounterd_instructions.insert(fixed_ae.program_counter());
        fixed_ae.step();
    }

    Ok(fixed_ae)
}

#[derive(Clone, Debug)]
pub struct AssemblyEmulator {
    acc: i32,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl AssemblyEmulator {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            acc: 0,
            program_counter: 0,
            instructions,
        }
    }

    pub fn acc(&self) -> i32 {
        self.acc
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn step(&mut self) {
        if self.program_counter >= self.instructions.len() {
            panic!("Segmentation fault in the assembly emulator.")
        }

        match self.instructions[self.program_counter] {
            Instruction(OpCode::Nop, _) => self.program_counter += 1,
            Instruction(OpCode::Acc, i) => {
                self.acc += i;
                self.program_counter += 1
            }
            Instruction(OpCode::Jmp, i) => {
                self.program_counter = (self.program_counter as i32 + i) as usize
            }
        }
    }
}

impl FromStr for AssemblyEmulator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(crate::utils::parse_line_separated_list(s)?))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Instruction(OpCode, i32);

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opcode, param): (&str, &str) = s
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| anyhow!("Could not parse instruction"))?;
        Ok(Self(opcode.parse()?, param.parse()?))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OpCode {
    Nop,
    Acc,
    Jmp,
}

impl FromStr for OpCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(OpCode::Nop),
            "acc" => Ok(OpCode::Acc),
            "jmp" => Ok(OpCode::Jmp),
            _ => Err(anyhow!("Unknwon instruction {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parse() {
        let ae: AssemblyEmulator = EXAMPLE.parse().expect("Should parse");
        assert_eq!(ae.instructions().len(), 9);
        assert_eq!(ae.instructions()[0], Instruction(OpCode::Nop, 0));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day08::solve1(EXAMPLE).unwrap(), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day08::solve2(EXAMPLE).unwrap(), 8);
    }
}

crate::benchmark_challenge!(crate::day08::Day08);
