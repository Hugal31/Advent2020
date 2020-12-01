#![cfg_attr(all(test, feature = "nightly"), feature(test))]

#[cfg(all(test, feature = "nightly"))]
extern crate test;

mod day01;
mod utils;

use std::fmt::Display;
use std::fs::File;
use std::io::Read as _;

use anyhow::{Context as _, Result};

pub trait Challenge {
    const DAY_NUMBER: u32;

    type InputType;
    type OutputType: Display;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType>;
    fn part2(input: &Self::InputType) -> Result<Self::OutputType>;
    fn parse(content: &str) -> Result<Self::InputType>;
}

trait ChallengeSolver {
    fn solve1(&self, content: &str) -> Result<String>;
    fn solve2(&self, content: &str) -> Result<String>;
}

struct ChallengeImpl<C: Challenge>(C);

impl<C: Challenge> ChallengeSolver for ChallengeImpl<C> {
    fn solve1(&self, content: &str) -> Result<String> {
        let input = C::parse(content)?;
        Ok(format!("{}", C::part1(&input)?))
    }

    fn solve2(&self, content: &str) -> Result<String> {
        let input = C::parse(content)?;
        Ok(format!("{}", C::part2(&input)?))
    }
}

static CHALLENGES: &[&(dyn ChallengeSolver + Sync + Send)] = &[&ChallengeImpl(day01::Day01)];

pub fn solve(day: u32, part: u8) -> Result<String> {
    let solvers = CHALLENGES
        .get(day as usize - 1)
        .context("day out of range")?;

    let content = read_file(day)?;

    match part {
        1 => solvers.solve1(&content),
        2 => solvers.solve2(&content),
        _ => panic!("Part must be 1 or 2, not {}", part),
    }
}

pub fn bench(day: u32, part: u8) -> Result<()> {
    let solvers = CHALLENGES
        .get(day as usize - 1)
        .context("day out of range")?;

    let content = read_file(day)?;

    match part {
        1 => bench_part("Part 1", |c| solvers.solve1(c), &content),
        2 => bench_part("Part 2", |c| solvers.solve2(c), &content),
        _ => panic!("Part must be 1 or 2, not {}", part),
    }
}

fn bench_part<F>(name: &str, part: F, content: &str) -> Result<()>
where
    F: FnOnce(&str) -> Result<String>,
{
    let start = std::time::Instant::now();
    (part)(content).map(|_| ())?;
    let elapsed = start.elapsed();
    println!("{}: {:?}", name, elapsed);
    Ok(())
}

fn read_file(day: u32) -> Result<String> {
    let file_name = format!("inputs/{:02}.txt", day);
    let mut file =
        File::open(&file_name).with_context(|| format!("while opening {}", file_name))?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

#[cfg(all(test, feature = "nightly"))]
mod benchmarks {
    use test::{black_box, Bencher};

    use super::Challenge;
    use crate::read_file;

    pub fn bench_challenge<C: Challenge>(bencher: &mut Bencher, part: u32) {
        let content = read_file(C::DAY_NUMBER).expect("Should be able to read input file");
        let input = C::parse(&content).expect("Should parse content");

        match part {
            1 => bencher.iter(|| C::part1(black_box(&input))),
            2 => bencher.iter(|| C::part2(black_box(&input))),
            _ => panic!("Part must be 1 or 2"),
        }
    }
}

#[macro_export]
macro_rules! benchmark_challenge {
    ($challenge:ty) => {
        #[cfg(all(test, feature = "nightly"))]
        mod benchmarks {
            use test::Bencher;

            use crate::benchmarks::bench_challenge;

            #[bench]
            fn bench_solve1(bench: &mut Bencher) {
                bench_challenge::<$challenge>(bench, 1)
            }

            #[bench]
            fn bench_solve2(bench: &mut Bencher) {
                bench_challenge::<$challenge>(bench, 2)
            }
        }
    };
}
