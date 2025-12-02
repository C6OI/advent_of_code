use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!("../../inputs/day_2/input");
    let mut sum = 0;

    INPUT.split(',').for_each(|range| {
        let (start, end) = range
            .split_once('-')
            .map(|(start, end)| {
                let start: i64 = start.parse().unwrap();
                let end: i64 = end.parse().unwrap();

                (start, end)
            })
            .unwrap();

        // sum += part_1(start, end);
        sum += part_2(start, end);
    });

    println!("Answer: {sum}")
}

fn part_1(start: i64, end: i64) -> i64 {
    let mut sum = 0;

    for id in start..=end {
        let as_string = id.to_string();
        let length = as_string.len();
        let mid = length / 2;

        if length - mid != mid {
            continue;
        }

        let (first, last) = as_string.split_at(mid);

        if first == last {
            sum += id;
            // println!("Duplicate: {id}; sum = {sum}");
        }
    }

    sum
}

fn part_2(start: i64, end: i64) -> i64 {
    const MAX_LENGTH: usize = 19; // i64::MAX digits count

    let mut sum = 0;
    let mut sizes_by_length = HashMap::with_capacity(MAX_LENGTH - 1);

    for id in start..=end {
        let as_string = id.to_string();
        let length = as_string.len();
        let chars: Vec<_> = as_string.chars().collect();

        let sizes: &Vec<_> = sizes_by_length
            .entry(length)
            .or_insert_with(|| (1..length).filter(|d| length % d == 0).collect());

        'sizes: for size in sizes {
            let mut prev_chunk = None;

            for chunk in chars.chunks(*size) {
                let chunk: String = chunk.iter().collect();

                if let Some(prev_chunk) = prev_chunk && chunk != prev_chunk {
                    continue 'sizes;
                }

                prev_chunk = Some(chunk);
            }

            sum += id;
            // println!("Duplicate: {id}; sum = {sum}");
            break;
        }
    }

    sum
}
