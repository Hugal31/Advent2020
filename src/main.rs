use anyhow::Result;
use chrono::prelude::*;

use advent2020::solve;

fn get_today_day() -> u32 {
    let today = Local::today();
    let first_december = NaiveDate::from_ymd(today.year(), 12, 1);
    let difference = today.naive_local() - first_december;
    (difference.num_days() + 1) as u32
}

fn main() -> Result<()> {
    let day = get_today_day();

    println!("Part 1 result is {}", solve(day, 1)?);
    println!("Part 2 result is {}", solve(day, 2)?);

    Ok(())
}
