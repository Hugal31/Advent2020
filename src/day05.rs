use std::convert::TryInto;
use std::str::FromStr;

use anyhow::{anyhow, Result};

use crate::Challenge;
use itertools::Itertools;

pub struct Day05;

impl Challenge for Day05 {
    const DAY_NUMBER: u32 = 5;

    type InputType = Vec<Ticket>;
    type OutputType = u32;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        input
            .iter()
            .map(|t| t.get_id())
            .max()
            .ok_or_else(|| anyhow!("There must be at least one ticket"))
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        input
            .iter()
            .map(|t| t.get_id())
            .sorted()
            .tuple_windows()
            .find(|&(a, b)| a + 2 == b)
            .map(|(a, _)| a + 1)
            .ok_or_else(|| anyhow!("Could not find the ticket"))
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        crate::utils::parse_line_separated_list(content)
    }
}

const ROW_NUMBER: u32 = 127;
const COLUMN_NUMBER: u32 = 7;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ticket {
    row: [LowHigh; 7],
    column: [LowHigh; 3],
}

impl Ticket {
    pub fn get_id(&self) -> u32 {
        self.row_id() * 8 + self.column_id()
    }

    fn row_id(&self) -> u32 {
        dichotomy(0, ROW_NUMBER, &self.row)
    }

    fn column_id(&self) -> u32 {
        dichotomy(0, COLUMN_NUMBER, &self.column)
    }
}

fn dichotomy(mut min: u32, mut max: u32, low_high: &[LowHigh]) -> u32 {
    for d in low_high {
        let half = (max - min) / 2 + min;

        match d {
            LowHigh::Low => max = half,
            LowHigh::High => min = half,
        }
    }

    max
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(anyhow!("The ticket length must be 10: {}", s));
        }

        let row = (&s[..7])
            .chars()
            .map(|c| parse_low_high(c, 'F', 'B'))
            .collect::<Result<Vec<LowHigh>>>()?
            .as_slice()
            .try_into()?;
        let column = (&s[7..])
            .chars()
            .map(|c| parse_low_high(c, 'L', 'R'))
            .collect::<Result<Vec<LowHigh>>>()?
            .as_slice()
            .try_into()?;

        Ok(Ticket { row, column })
    }
}

fn parse_low_high(c: char, low: char, high: char) -> Result<LowHigh> {
    match c {
        _ if c == low => Ok(LowHigh::Low),
        _ if c == high => Ok(LowHigh::High),
        _ => Err(anyhow!("Must be {} or {}, not {}", 'F', 'B', c)),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LowHigh {
    Low,
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use LowHigh::{High, Low};

        let tickets = Day05::parse("FBFBFFBRLR\nBBBBFFBRRL").expect("Should have parsed");
        assert_eq!(tickets.len(), 2);
        assert_eq!(
            tickets[0],
            Ticket {
                row: [Low, High, Low, High, Low, Low, High],
                column: [High, Low, High],
            }
        )
    }

    #[test]
    fn test_compute_id() {
        let ticket: Ticket = "BFFFBBFRRR".parse().unwrap();
        assert_eq!(ticket.row_id(), 70);
        assert_eq!(ticket.column_id(), 7);
        assert_eq!(ticket.get_id(), 567);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Day05::solve1("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL").unwrap(),
            820
        );
    }
}

crate::benchmark_challenge!(crate::day05::Day05);
