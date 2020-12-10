use anyhow::Result;

use crate::Challenge;
use itertools::Itertools;

pub struct Day10;

impl Challenge for Day10 {
    const DAY_NUMBER: u32 = 10;

    type InputType = Vec<u64>;
    type OutputType = u64;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        let input = {
            let mut v = prepare_input(input);
            v.push(v.last().unwrap() + 3);
            v
        };

        let mut one_diff = 0;
        let mut three_diff = 0;
        input.windows(2).for_each(|sa| match *sa {
            [socket, adapter] => {
                if adapter == socket + 1 {
                    one_diff += 1;
                } else if adapter == socket + 3 {
                    three_diff += 1;
                }
            }
            _ => unreachable!(),
        });

        Ok(one_diff * three_diff)
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        let input = prepare_input(input);

        Ok(part2_backtrack(&input))
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        crate::utils::parse_line_separated_list(content).map_err(Into::into)
    }
}

/// Sort input and prepend with a 0
fn prepare_input(input: &[u64]) -> Vec<u64> {
    let mut v = input.to_vec();
    v.push(0);
    v.sort_unstable();
    v
}

// The solution I found, by backtracking
fn part2_backtrack(input: &[u64]) -> u64 {
    let mut combination_per_adapter = Vec::new();
    combination_per_adapter.resize(input.len(), 1);

    // -2 because we skip the last two
    for i in (0..(input.len() - 2)).rev() {
        let current_socket = input[i];
        let number_of_suitable_adapters = input[i + 1..]
            .iter()
            .zip(&combination_per_adapter[i + 1..])
            .take_while(|(&adapter, _)| is_suitable_for(current_socket, adapter))
            .map(|(_, comb)| comb)
            .sum::<u64>();
        combination_per_adapter[i] = number_of_suitable_adapters;
    }

    combination_per_adapter[0]
}

#[allow(unused)]
const SUITE: &[u64] = &[1, 1, 2, 4, 7, 11, 16, 22];

// From Telly
#[allow(unused)]
fn part2_telly(input: &[u64]) -> u64 {
    // Diffs
    input
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        // Group
        .group_by(|f: &u64| *f)
        .into_iter()
        // Only use the group with 1 of difference
        .filter(|(key, _group)| *key == 1)
        // Convert to the number of possibilities
        .map(|(_key, group)| SUITE[group.count()])
        // And multiply all that
        .product()
}

fn is_suitable_for(socket: u64, adapter: u64) -> bool {
    adapter - socket <= 3
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const EXAMPLE2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_is_suitable_for() {
        assert!(is_suitable_for(2, 3));
        assert!(is_suitable_for(2, 4));
        assert!(is_suitable_for(2, 5));
        assert!(!is_suitable_for(2, 6));
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day10::solve1(EXAMPLE1).unwrap(), 7 * 5);
        assert_eq!(Day10::solve1(EXAMPLE2).unwrap(), 22 * 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day10::solve2(EXAMPLE1).unwrap(), 8);
        assert_eq!(Day10::solve2(EXAMPLE2).unwrap(), 19208);
    }

    #[test]
    fn test_part2_telly() {
        assert_eq!(
            part2_telly(&prepare_input(&Day10::parse(EXAMPLE1).unwrap())),
            8
        );
        assert_eq!(
            part2_telly(&prepare_input(&Day10::parse(EXAMPLE2).unwrap())),
            19208
        );
    }
}

crate::benchmark_challenge!(crate::day10::Day10);
