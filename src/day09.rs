use anyhow::{anyhow, Result};

use crate::Challenge;
use itertools::Itertools;

pub struct Day09;

impl Challenge for Day09 {
    const DAY_NUMBER: u32 = 9;

    type InputType = Vec<u64>;
    type OutputType = u64;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        input
            .windows(26)
            .find(|&window| !can_form_number(&window[..25], window[25]))
            .map(|window| window[25])
            .ok_or_else(|| anyhow!("Could not find number"))
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        let number_to_compute = Self::part1(input)?;

        for i in 0..(input.len() - 1) {
            for j in (i + 1)..input.len() {
                let range_sum: u64 = (i..=j).map(|idx| input[idx]).sum();
                if range_sum == number_to_compute {
                    let min = (i..=j).map(|idx| input[idx]).min().unwrap();
                    let max = (i..=j).map(|idx| input[idx]).max().unwrap();
                    return Ok(min + max);
                } else if range_sum > number_to_compute {
                    break;
                }
            }
        }

        Err(anyhow!("Could not find weakness"))
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        crate::utils::parse_line_separated_list(content).map_err(Into::into)
    }
}

fn can_form_number(window: &[u64], number: u64) -> bool {
    window
        .iter()
        .tuple_combinations()
        .any(|(a, b)| a + b == number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let numbers = (1..=25).chain(vec![26, 49, 100, 50].into_iter()).collect();
        assert_eq!(Day09::part1(&numbers).unwrap(), 100);
    }

    #[test]
    fn test_part2() {
        let numbers = (1..=25).chain(vec![26, 49, 100, 50].into_iter()).collect();
        assert_eq!(Day09::part2(&numbers).unwrap(), 9 + 16);
    }
}

crate::benchmark_challenge!(crate::day09::Day09);
