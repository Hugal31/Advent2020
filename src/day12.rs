use std::convert::TryFrom;

use anyhow::{anyhow, Result};

use crate::day12::InstructionType::Rotate;
use crate::Challenge;

pub struct Day12;

impl Challenge for Day12 {
    const DAY_NUMBER: u32 = 12;

    type InputType = Vec<Instruction>;
    type OutputType = u32;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        let mut state = State::default();

        input.iter().for_each(|i| i.apply(&mut state));

        Ok(state.coords.0.abs() as u32 + state.coords.1.abs() as u32)
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        unimplemented!()
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        crate::utils::parse_line_separated_list(content)
    }
}

#[derive(Debug)]
pub struct State {
    pub coords: (i32, i32),
    pub orientation: Direction,
}

impl Default for State {
    fn default() -> Self {
        Self {
            coords: (0, 0),
            orientation: Direction::East,
        }
    }
}

impl State {
    pub fn move_toward(&mut self, direction: Direction, amount: i32) {
        let offset = match direction {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        };
        self.coords.0 += offset.0 * amount;
        self.coords.1 += offset.1 * amount;
    }

    pub fn rotate(&mut self, rotation: Rotation, amount: i32) {
        self.orientation = self.orientation.rotated(rotation, amount);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    typ: InstructionType,
    amount: i32,
}

impl Instruction {
    pub fn apply(&self, state: &mut State) {
        match self.typ {
            InstructionType::Move(direction) => state.move_toward(direction, self.amount),
            InstructionType::Rotate(rotation) => state.rotate(rotation, self.amount),
            InstructionType::Forward => state.move_toward(state.orientation, self.amount),
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let typ = InstructionType::try_from(
            s.chars()
                .next()
                .ok_or_else(|| anyhow!("Empty instruction"))?,
        )?;
        let amount = s[1..].parse()?;

        Ok(Self { typ, amount })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InstructionType {
    Move(Direction),
    Rotate(Rotation),
    Forward,
}

impl TryFrom<char> for InstructionType {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            'N' => Ok(InstructionType::Move(Direction::North)),
            'S' => Ok(InstructionType::Move(Direction::South)),
            'E' => Ok(InstructionType::Move(Direction::East)),
            'W' => Ok(InstructionType::Move(Direction::West)),
            'L' => Ok(Rotate(Rotation::Left)),
            'R' => Ok(Rotate(Rotation::Right)),
            'F' => Ok(InstructionType::Forward),
            _ => Err(anyhow!("Unknown instruction type {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn rotated(&self, rotation: Rotation, amount: i32) -> Direction {
        let (rotation, amount) = if amount < 0 {
            (rotation.opposite(), amount.abs() as u32)
        } else {
            (rotation, amount as u32)
        };

        let amount = amount % 360;

        let (rotation, amount) = if amount > 180 {
            (rotation.opposite(), 360 - amount)
        } else {
            (rotation, amount)
        };

        match (self, rotation, amount) {
            (_, _, 0) => *self,
            (_, _, 180) => self.opposite(),
            (Direction::West, Rotation::Right, 90) | (Direction::East, Rotation::Left, 90) => {
                Direction::North
            }
            (Direction::North, Rotation::Right, 90) | (Direction::South, Rotation::Left, 90) => {
                Direction::East
            }
            (Direction::East, Rotation::Right, 90) | (Direction::West, Rotation::Left, 90) => {
                Direction::South
            }
            (Direction::South, Rotation::Right, 90) | (Direction::North, Rotation::Left, 90) => {
                Direction::West
            }
            _ => panic!("Unexpected rotation amount {}", amount),
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Rotation {
    Left,
    Right,
}

impl Rotation {
    pub fn opposite(&self) -> Rotation {
        match self {
            Rotation::Right => Rotation::Left,
            Rotation::Left => Rotation::Right,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        assert_eq!(Day12::solve1(EXAMPLE1).unwrap(), 25);
    }

    #[test]
    fn test_part2() {
        unimplemented!()
    }
}

crate::benchmark_challenge!(crate::day12::Day12);
