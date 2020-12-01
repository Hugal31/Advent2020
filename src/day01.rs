use anyhow::{anyhow, Result};
use itertools::Itertools as _;

use crate::utils;

pub fn solve1(content: &str) -> Result<String> {
    let numbers: Vec<u32> = utils::parse_space_separated_list(content)?;

    find_combination(&numbers, 2)
        .map(|n| format!("{}", n))
}

pub fn solve2(content: &str) -> Result<String> {
    let numbers: Vec<u32> = utils::parse_space_separated_list(content)?;

    find_combination(&numbers, 3)
        .map(|n| format!("{}", n))
}

fn find_combination(numbers: &[u32], size: usize) -> Result<u32> {
    for comb in numbers.iter().combinations(size) {
        if comb.iter().copied().sum::<u32>() == 2020 {
            return Ok(comb.iter().fold(1, |a, &&b| { a * b }));
        }
    }

    return Err(anyhow!("Could not find combination"));
}
