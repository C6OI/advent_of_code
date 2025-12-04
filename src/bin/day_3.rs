const INPUT: &str = include_str!("../../inputs/day_3/input");

fn main() {
    // let sum = part_1();
    let sum = part_2();
    println!("Answer: {sum}");
}

fn part_1() -> i64 {
    let mut sum: i64 = 0;

    for line in INPUT.lines() {
        let digits: Vec<_> = line.chars().map(|ch| ch.to_digit(10).unwrap()).collect();

        let (first_idx, first_digit) = digits
            .iter()
            .take(digits.len() - 1)
            .enumerate()
            .rev() // reverse because `max_by_key` returns the last max element
            .max_by_key(|(_, digit)| **digit)
            .unwrap();

        let second_digit = digits
            .iter()
            .skip(first_idx + 1)
            .max()
            .unwrap();

        let joltage = first_digit * 10 + second_digit;
        sum += joltage as i64;

        println!("Number: {line}; joltage: {joltage}; sum: {sum}")
    }

    sum
}

fn part_2() -> i64 {
    let mut sum: i64 = 0;

    for line in INPUT.lines() {
        let digits: Vec<_> = line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
        let mut joltage_digits = [0_u8; 12];

        let digits_count = digits.len();
        let mut last_digit_idx: isize = -1;

        for digit_num in (0..=11).rev() {
            println!("Digit {digit_num}: count = {digits_count}; last_idx = {last_digit_idx}");

            let (digit_idx, digit) = digits
                .iter()
                .enumerate()
                .take(digits_count - digit_num)
                .skip((last_digit_idx + 1) as usize)
                .rev()
                .max_by_key(|(_, digit)| **digit)
                .unwrap();

            joltage_digits[11 - digit_num] = *digit;
            last_digit_idx = digit_idx as isize;
        }

        let mut joltage: i64 = 0;

        for (idx, digit) in joltage_digits.iter().rev().enumerate() {
            joltage += *digit as i64 * 10_i64.pow(idx as u32);
        }

        sum += joltage;
        println!("Number: {line}; joltage: {joltage}; arr: {joltage_digits:?} sum: {sum}");
    }

    sum
}
