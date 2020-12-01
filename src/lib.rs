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
    let solvers = SOLVERS.get(day as usize - 1).context("day out of range")?;

    let content = read_file(day)?;

    match part {
        1 => solvers.0(&content),
        2 => solvers.1(&content),
        _ => panic!("Part must be 1 or 2, not {}", part),
    }
}

pub fn bench(day: u32, part: u8) -> Result<()> {
    let solvers = SOLVERS.get(day as usize - 1).context("day out of range")?;

    let content = read_file(day)?;

    match part {
        1 => bench_part("Part 1", solvers.0, &content),
        2 => bench_part("Part 2", solvers.1, &content),
        _ => panic!("Part must be 1 or 2, not {}", part),
    }
}

fn bench_part(name: &str, part: SolveFn, content: &str) -> Result<()> {
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
