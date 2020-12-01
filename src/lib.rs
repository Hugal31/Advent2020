#![cfg_attr(all(test, feature = "nightly"), feature(test))]

#[cfg(all(test, feature = "nightly"))]
extern crate test;

mod day01;
mod utils;

use std::fs::File;
use std::io::Read as _;

use anyhow::{Context as _, Result};

trait Challenge {
    fn solve1(&self, content: &str) -> Result<String>;
    fn solve2(&self, content: &str) -> Result<String>;
}

static CHALLENGES: &[&(dyn Challenge + Sync + Send)] = &[&day01::Day01];

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
