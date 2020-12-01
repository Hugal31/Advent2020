use anyhow::Result;
use chrono::prelude::*;
use clap::{self, value_t};

use advent2020::{bench, solve};

fn get_today_day() -> u32 {
    let today = Local::today();
    let first_december = NaiveDate::from_ymd(today.year(), 12, 1);
    let difference = today.naive_local() - first_december;
    (difference.num_days() + 1) as u32
}

fn main() -> Result<()> {
    let matches = clap::App::new("Advent 2020")
        .author("Hugo Laloge")
        .arg(
            clap::Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .takes_value(true),
        )
        .arg(clap::Arg::with_name("bench").short("b").long("bench"))
        .get_matches();

    let day = value_t!(matches, "day", u32).unwrap_or(get_today_day());
    let do_bench = matches.is_present("bench");

    if do_bench {
        bench(day, 1)?;
        bench(day, 2)?;
    } else {
        println!("Part 1 result is {}", solve(day, 1)?);
        println!("Part 2 result is {}", solve(day, 2)?);
    }

    Ok(())
}
