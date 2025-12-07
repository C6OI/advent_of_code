use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../inputs/day_7/input");

fn main() {
    for (idx, part) in [part_1, part_2].iter().enumerate() {
        let res = part();
        println!("[D7 P{}] Answer: {res}", idx + 1);
    }
}

fn part_1() -> i64 {
    let mut res = 0;

    let width = INPUT.lines().next().unwrap().len();
    let mut beam_positions = vec![None; width];

    for line in INPUT.lines() {
        for (idx, char) in line.chars().enumerate() {
            match char {
                'S' => {
                    beam_positions = vec![None; width];
                    beam_positions[idx] = Some(());
                    break;
                }
                '^' => {
                    if beam_positions[idx].take().is_some() {
                        if idx > 0 {
                            beam_positions[idx - 1] = Some(());
                        }

                        if idx < width - 1 {
                            beam_positions[idx + 1] = Some(());
                        }

                        res += 1;
                    }
                }
                '.' => continue,
                _ => panic!("Invalid symbol '{char}'"),
            }
        }
    }

    res
}

fn part_2() -> i64 {
    let mut res = 0;
    let mut beam_position = None;

    let mut splits: HashMap<(usize, usize), i64> = HashMap::new();
    let mut splitters: HashSet<(usize, usize)> = HashSet::new();

    for (line_idx, line) in INPUT.lines().enumerate() {
        for (char_idx, char) in line.chars().enumerate() {
            match char {
                'S' if beam_position.is_none() => {
                    beam_position = Some(char_idx);
                    break;
                }
                '^' => {
                    if res == 0 && beam_position == Some(char_idx) {
                        splits.insert((char_idx - 1, line_idx), 1);
                        splits.insert((char_idx + 1, line_idx), 1);

                        res += 2;
                    }

                    splitters.insert((char_idx, line_idx));
                }
                '.' => continue,
                _ => panic!("Invalid symbol '{char}'"),
            }
        }
    }

    let lines_length = INPUT.lines().count();

    while let Some(((beam_index, line_index), &count)) = splits.clone().iter().next() {
        splits.remove(&(*beam_index, *line_index));

        for line_index in (line_index + 1)..lines_length {
            if !splitters.contains(&(*beam_index, line_index)) {
                continue;
            }

            splits
                .entry((beam_index - 1, line_index))
                .and_modify(|left_count| *left_count += count)
                .or_insert(count);

            splits
                .entry((beam_index + 1, line_index))
                .and_modify(|right_count| *right_count += count)
                .or_insert(count);

            res += count;
            break;
        }
    }

    res
}
