use anyhow::{anyhow, Result};
use grid::Grid;

use crate::Challenge;
use std::convert::TryFrom;

pub struct Day11;

type Layout = Grid<Cell>;

impl Challenge for Day11 {
    const DAY_NUMBER: u32 = 11;

    type InputType = Layout;
    type OutputType = usize;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        unimplemented!()
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        unimplemented!()
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        let width = content
            .lines()
            .next()
            .ok_or_else(|| anyhow!("Empty grid"))?
            .len();
        let cells = content
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(<Cell as TryFrom<char>>::try_from)
            .collect::<Result<_>>()?;

        Ok(Grid::from_vec(cells, width))
    }
}

fn seating_step(layout: &Layout) -> Layout {
    // TODO
    unimplemented!()
}

fn get_occupied_neighbors(layout: &Layout, coords: (usize, usize)) -> usize {
    dbg!(layout.size());

    let mut total = 0;
    if coords.0 > 0 && layout[coords.1][coords.0 - 1] == Cell::Occupied {
        total += 1;
    }
    if coords.0 + 1 < layout.size().0 && layout[coords.1][coords.0 + 1] == Cell::Occupied {
        total += 1;
    }
    if coords.1 > 0 && layout[coords.1 - 1][coords.0] == Cell::Occupied {
        total += 1;
    }
    if coords.1 + 1 < layout.size().1 && layout[coords.1 + 1][coords.0] == Cell::Occupied {
        total += 1;
    }

    total
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Floor,
    Unoccupied,
    Occupied,
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Floor),
            'L' => Ok(Cell::Unoccupied),
            '#' => Ok(Cell::Occupied),
            _ => Err(anyhow!("Cannot parse seat: {}", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        assert_eq!(Day11::solve1(EXAMPLE).unwrap(), 37);
    }

    #[test]
    fn test_part2() {
        unimplemented!()
    }
}

crate::benchmark_challenge!(crate::day11::Day11);
