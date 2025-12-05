use std::collections::HashSet;
use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/day_5/input");

fn main() {
    // let res = part_1();
    let res = part_2();
    println!("Answer: {res}");
}

fn part_1() -> i64 {
    let mut res = 0;
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    parse_input(&mut ranges, &mut Some(&mut ids));

    for id in ids {
        if ranges.iter().any(|range| range.contains(&id)) {
            res += 1;
        }
    }

    res
}

fn part_2() -> i64 {
    let mut ranges = Vec::new();
    parse_input(&mut ranges, &mut None);
    ranges.sort_by_key(|range| *range.start());

    let mut filtered_ranges: HashSet<RangeInclusive<usize>> = HashSet::new();
    let mut filtered_range: Option<RangeInclusive<usize>> = None;

    for range in &ranges {
        let Some(value) = filtered_range.as_ref() else {
            filtered_range = Some(range.clone());
            continue;
        };

        let contains_start = value.contains(range.start());
        let contains_end = value.contains(range.end());

        if contains_start && contains_end {
            continue;
        }

        if !contains_start && !contains_end {
            if value.start() > range.start() && value.end() < range.end() {
                filtered_range = Some(range.clone());
            } else {
                filtered_ranges.insert(value.clone());
                filtered_range = Some(range.clone());
            }

            continue;
        }

        if contains_start {
            filtered_range = Some(RangeInclusive::new(*value.start(), *range.end()));
        } else {
            filtered_range = Some(RangeInclusive::new(*range.start(), *value.end()));
        }
    }

    if let Some(value) = filtered_range {
        filtered_ranges.insert(value.clone());
    }

    println!(
        "Ranges filtered. {} => {}",
        ranges.len(),
        filtered_ranges.len()
    );

    filtered_ranges
        .into_iter()
        .map(|range| range.count())
        .sum::<usize>() as i64
}

fn parse_input(ranges: &mut Vec<RangeInclusive<usize>>, ids: &mut Option<&mut Vec<usize>>) {
    let mut is_ranges = true;

    for line in INPUT.lines() {
        if line.is_empty() {
            if ids.is_none() {
                return;
            }

            is_ranges = false;
            continue;
        }

        if is_ranges {
            let (start, end) = line.split_once('-').unwrap();
            let (start, end) = (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            );

            ranges.push(RangeInclusive::new(start, end));
        } else {
            let id = line.parse::<usize>().unwrap();
            ids.as_mut().unwrap().push(id);
        }
    }
}
