use anyhow::{anyhow, Result};

use crate::Challenge;

pub struct Day10;

impl Challenge for Day10 {
    const DAY_NUMBER: u32 = 10;

    type InputType = Vec<u64>;
    type OutputType = u64;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        let input = {
            let mut i = input.clone();
            i.push(0);
            i.sort();
            i.push(i.last().unwrap() + 3);
            i
        };

        let mut one_diff = 0;
        let mut three_diff = 0;
        input.windows(2).for_each(|sa| match sa {
            &[socket, adapter] => {
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
        let input = {
            let mut i = input.clone();
            i.push(0);
            i.sort();
            i
        };

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

        Ok(combination_per_adapter[0])
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        crate::utils::parse_line_separated_list(content).map_err(Into::into)
    }
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
}

crate::benchmark_challenge!(crate::day10::Day10);
