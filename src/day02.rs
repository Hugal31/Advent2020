use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::{utils, Challenge};
use itertools::Itertools;

pub struct Day02;

impl Challenge for Day02 {
    const DAY_NUMBER: u32 = 02;

    type InputType = Vec<PasswordEntry>;
    type OutputType = usize;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(input.iter().filter(|p| p.is_valid_occurrences()).count())
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(input.iter().filter(|p| p.is_valid_positions()).count())
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        utils::parse_line_separated_list(content).into()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordEntry {
    password: String,
    policy: PasswordPolicy,
}

impl PasswordEntry {
    pub fn is_valid_occurrences(&self) -> bool {
        let n_letter = self
            .password
            .chars()
            .filter(|&c| c == self.policy.letter)
            .count();

        n_letter >= self.policy.range.0 && n_letter <= self.policy.range.1
    }

    pub fn is_valid_positions(&self) -> bool {
        let pos_1 = self
            .password
            .chars()
            .nth(self.policy.range.0 - 1)
            .map(|c| c == self.policy.letter)
            .unwrap_or(false);
        let pos_2 = self
            .password
            .chars()
            .nth(self.policy.range.1 - 1)
            .map(|c| c == self.policy.letter)
            .unwrap_or(false);

        pos_1 ^ pos_2
    }
}

impl FromStr for PasswordEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (policy_str, password_str): (&str, &str) = s
            .split(':')
            .map(str::trim)
            .collect_tuple()
            .ok_or(anyhow!("Invalid password entry"))?;

        let password = password_str.to_owned();
        let policy = policy_str.parse()?;

        Ok(PasswordEntry { password, policy })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasswordPolicy {
    letter: char,
    range: (usize, usize),
}

impl FromStr for PasswordPolicy {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (range_str, letter_str) = s
            .split(' ')
            .collect_tuple()
            .ok_or(anyhow!("Invalid policy"))?;

        let range_res: (
            Result<usize, std::num::ParseIntError>,
            Result<usize, std::num::ParseIntError>,
        ) = range_str
            .split('-')
            .map(str::parse)
            .collect_tuple()
            .ok_or(anyhow!("Could not parse range."))?;
        let range = (range_res.0?, range_res.1?);

        if !letter_str.len() == 1 {
            return Err(anyhow!("The letter must be of length 1"));
        }
        let letter = letter_str.chars().nth(0).unwrap();

        Ok(PasswordPolicy { letter, range })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Challenge as _;

    #[test]
    fn test_parse_policy() {
        assert_eq!(
            "2-3 a".parse::<PasswordPolicy>().unwrap(),
            PasswordPolicy {
                letter: 'a',
                range: (2, 3)
            }
        );
    }

    #[test]
    fn test_parse_entry() {
        assert_eq!(
            "2-3 a: abcdef".parse::<PasswordEntry>().unwrap(),
            PasswordEntry {
                password: "abcdef".to_string(),
                policy: PasswordPolicy {
                    letter: 'a',
                    range: (2, 3)
                }
            }
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Day02::solve1(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            )
            .unwrap(),
            2
        );
    }

    #[test]
    fn test_solve2() {
        assert_eq!(
            Day02::solve2(
                "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"
            )
            .unwrap(),
            1
        );
    }
}

crate::benchmark_challenge!(crate::day02::Day02);
