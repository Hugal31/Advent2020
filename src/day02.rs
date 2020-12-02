use std::str::FromStr;

use anyhow::{anyhow, Result};
use scan_fmt::{scan_fmt, scan_fmt_some};

use crate::{utils, Challenge};

pub struct Day02;

impl Challenge for Day02 {
    const DAY_NUMBER: u32 = 2;

    type InputType = Vec<PasswordEntry>;
    type OutputType = usize;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(input.iter().filter(|p| p.is_valid_occurrences()).count())
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(input.iter().filter(|p| p.is_valid_positions()).count())
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        utils::parse_line_separated_list(content)
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
        let (policy, password) = scan_fmt_some!(s, "{[^:]}: {}", PasswordPolicy, String);
        let policy = policy.ok_or_else(|| anyhow!("Could not parse policy"))?;
        let password = password.ok_or_else(|| anyhow!("Could not parse password"))?;

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
        let (range_beg, range_end, letter) = scan_fmt!(s, "{}-{} {}", usize, usize, char)?;

        Ok(PasswordPolicy {
            letter,
            range: (range_beg, range_end),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_part2() {
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
