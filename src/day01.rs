use anyhow::{anyhow, Result};
use itertools::Itertools as _;

use crate::{Challenge, utils};

pub struct Day01;

impl Challenge for Day01 {
    fn solve1(&self, content: &str) -> Result<String> {
        let content = utils::parse_space_separated_list(content)?;
        solve1(&content).map(|r| format!("{}", r))
    }

    fn solve2(&self, content: &str) -> Result<String> {
        let content = utils::parse_space_separated_list(content)?;
        solve2(&content).map(|r| format!("{}", r))
    }
}

pub fn solve1(numbers: &[u32]) -> Result<u32> {
    find_combination(&numbers, 2)
}

pub fn solve2(numbers: &[u32]) -> Result<u32> {
    find_combination(&numbers, 3)
}

fn find_combination(numbers: &[u32], size: usize) -> Result<u32> {
    for comb in numbers.iter().combinations(size) {
        if comb.iter().copied().sum::<u32>() == 2020 {
            return Ok(comb.iter().fold(1, |a, &&b| { a * b }));
        }
    }

    return Err(anyhow!("Could not find combination"));
}

#[cfg(test)]
mod tests {
    use super::{solve1, solve2};

    static NUMBERS: &[u32] = &[1721, 979, 366, 299, 675, 1456];

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&NUMBERS).unwrap(), 514579);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&NUMBERS).unwrap(), 241861950);
    }
}
