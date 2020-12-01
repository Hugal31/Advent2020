use anyhow::{anyhow, Result};
use itertools::Itertools as _;

use crate::{utils, Challenge};

pub struct Day01;

impl Challenge for Day01 {
    const DAY_NUMBER: u32 = 1;

    type InputType = Vec<u32>;
    type OutputType = u32;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        input
            .into_iter()
            .copied()
            .tuple_combinations()
            .find(|(a, b)| a + b == 2020)
            .map(|(a, b)| a * b)
            .ok_or_else(|| anyhow!("Could not find combination"))
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        input
            .into_iter()
            .copied()
            .tuple_combinations()
            .find(|(a, b, c)| a + b + c == 2020)
            .map(|(a, b, c)| a * b * c)
            .ok_or_else(|| anyhow!("Could not find combination"))
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        utils::parse_line_separated_list(content).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::Day01;
    use crate::Challenge as _;

    static NUMBERS: &[u32] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_part1() {
        assert_eq!(Day01::part1(&NUMBERS.to_owned()).unwrap(), 514579);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(Day01::part2(&NUMBERS.to_owned()).unwrap(), 241861950);
    }
}

crate::benchmark_challenge!(crate::day01::Day01);
