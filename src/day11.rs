use std::convert::TryFrom;

use anyhow::{anyhow, Result};
use grid::Grid;

use crate::Challenge;
use itertools::Itertools;

pub struct Day11;

type Layout = Grid<Cell>;

impl Challenge for Day11 {
    const DAY_NUMBER: u32 = 11;

    type InputType = Layout;
    type OutputType = usize;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        let equilibrium = find_equilibrium(input, 4, get_occupied_neighbors);
        Ok(count_occupied(&equilibrium))
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        let equilibrium = find_equilibrium(input, 5, get_visible_neighbors);
        Ok(count_occupied(&equilibrium))
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        let width = content
            .lines()
            .next()
            .ok_or_else(|| anyhow!("Empty grid"))?
            .trim_end()
            .len();
        let cells = content
            .trim()
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(<Cell as TryFrom<char>>::try_from)
            .collect::<Result<_>>()?;

        Ok(Grid::from_vec(cells, width))
    }
}

fn find_equilibrium<F>(input: &Layout, max_neighbors: usize, neighbors_func: F) -> Layout
where
    F: Copy + Fn(&Layout, (usize, usize)) -> usize,
{
    let mut last_input = input.clone();
    let mut new_input = seating_step(&last_input, max_neighbors, neighbors_func);

    while last_input != new_input {
        last_input = new_input.clone();
        new_input = seating_step(&last_input, max_neighbors, neighbors_func);
    }

    new_input
}

fn seating_step<F>(layout: &Layout, max_neighbors: usize, neighbors_func: F) -> Layout
where
    F: Fn(&Layout, (usize, usize)) -> usize,
{
    let cells = (0..layout.rows())
        .cartesian_product(0..layout.cols())
        .map(|(y, x)| {
            let cell = layout[y][x];
            if cell != Cell::Floor {
                new_cell_state(cell, neighbors_func(layout, (x, y)), max_neighbors)
            } else {
                cell
            }
        })
        .collect();

    Layout::from_vec(cells, layout.cols())
}

fn new_cell_state(cell: Cell, occupied_neighbors: usize, max_neighbors: usize) -> Cell {
    match (cell, occupied_neighbors) {
        (Cell::Empty, 0) => Cell::Occupied,
        (Cell::Occupied, i) if i >= max_neighbors => Cell::Empty,
        (c, _) => c,
    }
}

const DIRS: &[(i8, i8)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_occupied_neighbors(layout: &Layout, coords: (usize, usize)) -> usize {
    DIRS.iter()
        .filter_map(|&offset| filter_map_in_bounds(layout, coords, offset))
        .filter(|&(neighbor_x, neighbor_y)| layout[neighbor_y][neighbor_x] == Cell::Occupied)
        .count()
}

fn get_visible_neighbors(layout: &Layout, coords: (usize, usize)) -> usize {
    DIRS.iter()
        .copied()
        // For each direction, tries multiple of that direction to find the first visible seat.
        .filter(|&offset| {
            (1..)
                // Multiply offset
                .map(|i| (offset.0 * i, offset.1 * i))
                // map_while offset is in bounds
                .map(|offset| filter_map_in_bounds(layout, coords, offset))
                .take_while(|&visible_coords| visible_coords.is_some())
                .map(|visible_coords| visible_coords.unwrap())
                .map(|(x, y)| layout[y][x])
                .find(|&cell| cell != Cell::Floor)
                == Some(Cell::Occupied)
        })
        .count()
}

fn filter_map_in_bounds(
    layout: &Layout,
    (x, y): (usize, usize),
    (offset_x, offset_y): (i8, i8),
) -> Option<(usize, usize)> {
    if is_in_bounds(layout, (x, y), (offset_x, offset_y)) {
        Some((
            (x as i64 + offset_x as i64) as usize,
            (y as i64 + offset_y as i64) as usize,
        ))
    } else {
        None
    }
}

fn is_in_bounds(layout: &Layout, coords: (usize, usize), offset: (i8, i8)) -> bool {
    match (coords, offset) {
        ((x, _), (offset_x, _)) if offset_x < 0 && (-offset_x) as usize > x => false,
        ((_, y), (_, offset_y)) if offset_y < 0 && (-offset_y) as usize > y => false,
        ((x, _), (offset_x, _)) if offset_x > 0 && x + offset_x as usize >= layout.cols() => false,
        ((_, y), (_, offset_y)) if offset_y > 0 && y + offset_y as usize >= layout.rows() => false,
        _ => true,
    }
}

fn count_occupied(layout: &Layout) -> usize {
    layout.iter().filter(|&&c| c == Cell::Occupied).count()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Floor),
            'L' => Ok(Cell::Empty),
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
        assert_eq!(Day11::solve2(EXAMPLE).unwrap(), 26);
    }
}

crate::benchmark_challenge!(crate::day11::Day11);
