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

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        unimplemented!()
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        content.parse()
    }
}

#[derive(Clone, Debug)]
pub struct AssemblyEmulator {
    acc: i32,
    instructions: Vec<Instruction>,
    program_counter: usize,
}

impl AssemblyEmulator {
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

        if self.program_counter >= self.instructions.len() {
            panic!("Segmentation fault in the assembly emulator.")
        }
    }
}

impl FromStr for AssemblyEmulator {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            acc: 0,
            program_counter: 0,
            instructions: crate::utils::parse_line_separated_list(s)?,
        })
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
        unimplemented!()
    }
}

crate::benchmark_challenge!(crate::day08::Day08);
