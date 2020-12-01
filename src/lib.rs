mod day01;
mod utils;

use std::fs::File;
use std::io::Read as _;

use anyhow::{Context as _, Result};

type SolveFn = fn(&str) -> Result<String>;

static SOLVERS: &[(SolveFn, SolveFn)] = &[
    (day01::solve1, day01::solve2),
];

pub fn solve(day: u32, part: u8) -> Result<String> {
    let content = read_file(day)?;

    let solvers = SOLVERS.get(day as usize - 1).context("day out of range")?;

    match part {
        1 => solvers.0(&content),
        2 => solvers.1(&content),
        _ => panic!("Part must be 1 or 2, not {}", part),
    }
}

fn read_file(day: u32) -> Result<String> {
    let file_name = format!("inputs/{:02}.txt", day);
    let mut file =
        File::open(&file_name).with_context(|| format!("while opening {}", file_name))?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}
