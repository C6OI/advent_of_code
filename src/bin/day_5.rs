use std::ops::RangeInclusive;

const INPUT: &str = include_str!("../../inputs/day_5/input");

fn main() {
    let res = part_1();
    // let res = part_2();
    println!("Answer: {res}");
}

fn part_1() -> i64 {
    let mut res = 0;
    let mut is_ranges = true;
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    INPUT.lines().for_each(|line| {
        if line.is_empty() {
            is_ranges = false;
            return;
        }

        if is_ranges {
            let (start, end) = line.split_once('-').unwrap();
            let (start, end) = (start.parse::<usize>().unwrap(), end.parse::<usize>().unwrap());

            ranges.push(RangeInclusive::new(start, end));
        } else {
            let id = line.parse::<usize>().unwrap();
            ids.push(id);
        }
    });

    for id in ids {
        if ranges.iter().any(|range| range.contains(&id)) {
            res += 1;
        }
    }

    res
}
