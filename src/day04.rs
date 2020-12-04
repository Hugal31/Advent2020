use std::collections::HashMap;

use anyhow::{anyhow, Result};
use itertools::Itertools as _;

use crate::Challenge;

pub struct Day04;

type Passport = HashMap<String, String>;

impl Challenge for Day04 {
    const DAY_NUMBER: u32 = 4;

    type InputType = Vec<Passport>;
    type OutputType = usize;

    fn part1(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(input
            .iter()
            .filter(|entry| has_required_fields(entry))
            .count())
    }

    fn part2(input: &Self::InputType) -> Result<Self::OutputType> {
        Ok(input
            .iter()
            .filter(|entry| passport_is_valid(entry))
            .count())
    }

    fn parse(content: &str) -> Result<Self::InputType> {
        content.split("\n\n").map(parse_entry).collect()
    }
}

fn parse_entry(entry: &str) -> Result<Passport> {
    entry
        .split_ascii_whitespace()
        .map(|s| {
            s.split(':')
                .map(str::to_string)
                .collect_tuple()
                .ok_or_else(|| anyhow!("Ill-formated entry: {}", s))
        })
        .collect()
}

fn has_required_fields(entry: &Passport) -> bool {
    REQUIRED_FIELDS
        .iter()
        .all(|field| entry.contains_key::<str>(field))
}

fn passport_is_valid(passport: &Passport) -> bool {
    has_required_fields(passport)
        && passport
            .iter()
            .all(|(field, value)| validate_field(field, value))
}

fn validate_field(field: &str, value: &str) -> bool {
    match field {
        "byr" => is_date_between(value, 1920, 2002),
        "iyr" => is_date_between(value, 2010, 2020),
        "eyr" => is_date_between(value, 2020, 2030),
        "hgt" => validate_height(value),
        "hcl" => validate_hair_color(value),
        "ecl" => VALID_EYES_COLORS.contains(&value),
        "pid" => validate_pid(value),
        "cid" => true,
        _ => false,
    }
}

fn is_date_between(s: &str, after: u32, before: u32) -> bool {
    let date = s.parse().unwrap_or(0);
    s.len() == 4 && s.chars().all(char::is_numeric) && date >= after && date <= before
}

fn validate_height(s: &str) -> bool {
    if !s.ends_with("cm") && !s.ends_with("in") {
        return false;
    }

    let height_str = &s[..s.len() - 2];
    let height = height_str.parse().unwrap_or(0);

    if s.ends_with("cm") {
        (150..=193).contains(&height)
    } else {
        (59..=76).contains(&height)
    }
}

fn validate_hair_color(color: &str) -> bool {
    color.starts_with('#')
        && color.len() == 7
        && (&color[1..])
            .chars()
            .all(|c| "0123456789abcdef".contains(c))
}

fn validate_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(char::is_numeric)
}

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

const VALID_EYES_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LIST1: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const TEST_LIST2: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    #[test]
    fn test_parse() {
        let entries = Day04::parse(TEST_LIST1).expect("Should parse");
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0]["ecl"], "gry");
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day04::solve1(TEST_LIST1).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day04::solve2(TEST_LIST2).unwrap(), 4);
    }
}

crate::benchmark_challenge!(crate::day04::Day04);
