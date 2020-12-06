use anyhow::Result;

use crate::Challenge;
use std::collections::HashSet;

pub struct Day06;

impl Challenge for Day06 {
    const DAY_NUMBER: u32 = 6;

    type InputType = Vec<Vec<HashSet<char>>>;
    type OutputType = usize;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(input
            .iter()
            .map(|group| {
                group[1..]
                    .iter()
                    .fold(group[0].clone(), |s, p| s.union(p).copied().collect())
                    .len()
            })
            .sum())
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(input
            .iter()
            .map(|group| {
                group[1..]
                    .iter()
                    .fold(group[0].clone(), |s, p| {
                        s.intersection(p).copied().collect()
                    })
                    .len()
            })
            .sum())
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        Ok(content
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|person| person.chars().collect())
                    .collect()
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let groups = Day06::parse(
            "abc
cde

ab",
        )
        .expect("Should have parsed");

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].len(), 2);
        assert!(groups[0][0].contains(&'a'));
    }

    const EXAMPLE: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part1() {
        assert_eq!(Day06::solve1(EXAMPLE).unwrap(), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day06::solve2(EXAMPLE).unwrap(), 6);
    }
}

crate::benchmark_challenge!(crate::day06::Day06);
