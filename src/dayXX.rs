use anyhow::{anyhow, Result};

use crate::Challenge;

pub struct DayXX;

impl Challenge for DayXX {
    const DAY_NUMBER: u32 = XX;

    type InputType = ();
    type OutputType = ();

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        unimplemented!()
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        unimplemented!()
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        unimplemented!()
    }

    #[test]
    fn test_part2() {
        unimplemented!()
    }
}

crate::benchmark_challenge!(crate::dayXX::DayXX);
