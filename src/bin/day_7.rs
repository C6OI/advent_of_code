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
            if char == 'S' {
                beam_positions = vec![None; width];
                beam_positions[idx] = Some(());
                break;
            }

            if char == '.' {
                continue;
            }

            if char == '^' && beam_positions[idx].take().is_some() {
                if idx > 0 {
                    beam_positions[idx - 1] = Some(());
                }

                if idx < width - 1 {
                    beam_positions[idx + 1] = Some(());
                }

                res += 1;
            }
        }
    }

    res
}

fn part_2() -> i64 {
    let mut res = 0;

    res
}
