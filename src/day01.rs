use anyhow::{anyhow, Result};
use itertools::Itertools as _;

use crate::{utils, Challenge};

pub struct Day01;

impl Challenge for Day01 {
    const DAY_NUMBER: u32 = 1;

    type InputType = Vec<u32>;
    type OutputType = u32;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        find_combination(input, 2)
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        find_combination(input, 3)
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        utils::parse_space_separated_list(content).map_err(Into::into)
    }
}

fn find_combination(numbers: &[u32], size: usize) -> Result<u32> {
    for comb in numbers.iter().combinations(size) {
        if comb.iter().copied().sum::<u32>() == 2020 {
            return Ok(comb.iter().copied().product());
        }
    }

    Err(anyhow!("Could not find combination"))
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
