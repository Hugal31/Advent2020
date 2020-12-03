use anyhow::{anyhow, Result};
use grid::Grid;

use crate::Challenge;

pub struct Day03;

impl Challenge for Day03 {
    const DAY_NUMBER: u32 = 3;

    type InputType = Grid<bool>;
    type OutputType = usize;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(test_slope(input, slope_2(input)))
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(test_slope(input, slope_1(input))
            * test_slope(input, slope_2(input))
            * test_slope(input, slope_3(input))
            * test_slope(input, slope_4(input))
            * test_slope(input, slope_5(input)))
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        let vec2d: Vec<Vec<bool>> = content
            .lines()
            .map(|line| line.chars().map(is_tree).collect())
            .collect();

        let width = vec2d[0].len();

        if vec2d.iter().any(|r| r.len() != width) {
            return Err(anyhow!("All the row doesn't have the same size"));
        }

        let vec: Vec<bool> = vec2d.into_iter().flatten().collect();

        Ok(Grid::from_vec(vec, width))
    }
}

fn test_slope(grid: &Grid<bool>, slope: impl Iterator<Item = (usize, usize)>) -> usize {
    slope.filter(|(x, y)| grid[*y][x % grid.size().1]).count()
}

fn slope_1(grid: &Grid<bool>) -> impl Iterator<Item = (usize, usize)> {
    slope(1, 1, grid.size().0)
}

fn slope_2(grid: &Grid<bool>) -> impl Iterator<Item = (usize, usize)> {
    slope(3, 1, grid.size().0)
}

fn slope_3(grid: &Grid<bool>) -> impl Iterator<Item = (usize, usize)> {
    slope(5, 1, grid.size().0)
}

fn slope_4(grid: &Grid<bool>) -> impl Iterator<Item = (usize, usize)> {
    slope(7, 1, grid.size().0)
}

fn slope_5(grid: &Grid<bool>) -> impl Iterator<Item = (usize, usize)> {
    slope(1, 2, grid.size().0)
}

fn slope(step_x: usize, step_y: usize, height: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..).step_by(step_x).zip((0..height).step_by(step_y))
}

fn is_tree(c: char) -> bool {
    c == '#'
}

#[cfg(test)]
mod tests {
    use super::Day03;
    use crate::Challenge as _;

    const GRID: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_parse() {
        let grid_str = ".##
#.#";
        let grid = Day03::parse(grid_str).unwrap();
        assert_eq!(grid.size(), (2, 3));
        assert_eq!(grid[0][0], false);
        assert_eq!(grid[0][1], true);
        assert_eq!(grid[0][2], true);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day03::solve1(GRID).unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day03::solve2(GRID).unwrap(), 336);
    }
}

crate::benchmark_challenge!(crate::day03::Day03);
