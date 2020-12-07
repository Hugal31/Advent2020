use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools as _;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::Challenge;

pub struct Day07;

type Rules = DiGraph<(), usize>;

impl Challenge for Day07 {
    const DAY_NUMBER: u32 = 7;

    type InputType = (Rules, HashMap<String, NodeIndex>);
    type OutputType = usize;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        use petgraph::visit::{Dfs, Reversed};

        let shiny_gold = input.1["shiny gold"];

        let rev = Reversed(&input.0);
        let mut dfs = Dfs::new(&rev, shiny_gold);
        let mut n = 0;
        while dfs.next(&rev).is_some() {
            n += 1;
        }

        Ok(n - 1)
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        let shiny_gold = input.1["shiny gold"];
        let mut cache = HashMap::new();

        Ok(get_number_of_bags(shiny_gold, &input.0, &mut cache) - 1)
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        let rules_v = content
            .lines()
            .map(parse_rule)
            .collect::<Result<Vec<_>>>()?;

        let mut rules = Rules::new();
        let mut indexes = HashMap::new();
        for rule in rules_v {
            let rule_index = *indexes.entry(rule.0).or_insert_with(|| rules.add_node(()));
            for edge in rule.1 {
                let contained_index = *indexes.entry(edge.0).or_insert_with(|| rules.add_node(()));
                rules.add_edge(rule_index, contained_index, edge.1);
            }
        }

        Ok((rules, indexes))
    }
}

fn get_number_of_bags(
    node: NodeIndex,
    rules: &Rules,
    cache: &mut HashMap<NodeIndex, usize>,
) -> usize {
    use petgraph::Direction;

    rules
        .neighbors_directed(node, Direction::Outgoing)
        .map(|next_node| {
            let mult = rules
                .edge_weight(rules.find_edge(node, next_node).unwrap())
                .unwrap();
            let bag_size = if cache.contains_key(&next_node) {
                cache[&next_node]
            } else {
                let n = get_number_of_bags(next_node, rules, cache);
                cache.insert(next_node, n);
                n
            };

            mult * bag_size
        })
        .sum::<usize>()
        + 1
}

fn parse_rule(rule: &str) -> Result<(String, Vec<(String, usize)>)> {
    let (name, contained) = rule
        .split(" bags contain ")
        .collect_tuple()
        .ok_or_else(|| anyhow!("could not parse rule"))?;

    let contained = if contained == "no other bags." {
        Vec::new()
    } else {
        contained
            .split(", ")
            .map(|c| {
                let (n_str, name) = c
                    .splitn(2, ' ')
                    .collect_tuple()
                    .ok_or_else(|| anyhow!("could not parse rule"))?;
                let n = n_str.parse()?;
                Ok((
                    name.trim_end_matches('.')
                        .trim_end_matches(" bags")
                        .trim_end_matches(" bag")
                        .to_owned(),
                    n,
                ))
            })
            .collect::<Result<Vec<(String, usize)>>>()?
    };

    Ok((name.to_owned(), contained))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EXAMPLE2: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn test_parse() {
        let (rules, indexes) = Day07::parse(EXAMPLE1).expect("should parse");
        assert!(indexes.contains_key::<str>("dark orange"));
        assert!(indexes.contains_key::<str>("bright white"));
        let light_red = indexes["light red"];
        let muted_yellow = indexes["muted yellow"];
        assert_eq!(
            rules
                .edge_weight(rules.find_edge(light_red, muted_yellow).unwrap())
                .copied(),
            Some(2)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day07::solve1(EXAMPLE1).unwrap(), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day07::solve2(EXAMPLE1).unwrap(), 32);
        assert_eq!(Day07::solve2(EXAMPLE2).unwrap(), 126);
    }
}

crate::benchmark_challenge!(crate::day07::Day07);
