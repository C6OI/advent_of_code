const INPUT: &str = include_str!("../../inputs/day_6/input");

fn main() {
    // let res = part_1();
    let res = part_2();
    println!("Answer: {res}");
}

fn part_1() -> i64 {
    let mut res = 0;
    let columns = parse_input();

    for column in columns {
        // println!("{:?}", column);
        let numbers = column
            .iter()
            .take(column.len() - 1)
            .map(|value| value.trim().parse::<i64>().unwrap());
        let action = column.last().unwrap();

        if action.trim() == "+" {
            res += numbers.sum::<i64>();
        } else if action.trim() == "*" {
            res += numbers.product::<i64>()
        } else {
            panic!("Invalid action '{action}'")
        }
    }

    res
}

fn part_2() -> i64 {
    let mut res = 0;
    let mut columns = parse_input();
    let mut numbers: Vec<(String, Vec<i64>)> = Vec::new();

    for (idx, column) in columns.iter_mut().enumerate() {
        let action = column.remove(column.len() - 1);
        let longest = column.iter().map(|val| val.len()).max().unwrap();

        for i in (0..longest).rev() {
            let mut number = String::with_capacity(4);

            for value in &*column {
                let Some(char) = value.chars().nth(i) else {
                    continue;
                };

                if !char.is_numeric() {
                    continue;
                }

                number.push(char)
            }

            if number.is_empty() {
                continue;
            }

            if idx >= numbers.len() {
                numbers.resize_with(idx + 1, || (action.clone(), Vec::new()));
            }
            numbers[idx].1.push(number.parse::<i64>().unwrap());
        }
    }

    println!("numbers: {numbers:?}");

    for (action, numbers) in numbers {
        res += if action.trim() == "+" {
            numbers.iter().sum::<i64>()
        } else if action.trim() == "*" {
            numbers.iter().product::<i64>()
        } else {
            panic!("Invalid action: {action}")
        };
    }

    res
}

fn parse_input() -> Vec<Vec<String>> {
    let mut columns: Vec<Vec<String>> = Vec::new();

    let actions = INPUT.lines().last().unwrap();
    let column_lengths = count_spaces_between(actions, &['*', '+']);

    println!("lengths: {column_lengths:?}");

    for line in INPUT.lines() {
        let mut chars = line.chars().peekable();
        let mut values = Vec::new();

        for &col_length in &column_lengths {
            let mut value = String::with_capacity(col_length);

            for _ in 0..col_length {
                let char = chars.next().unwrap();
                value.push(char);
            }

            if let Some(char) = chars.peek() && char.is_whitespace() {
                chars.next();
            }

            values.push(value);
        }

        for char in chars {
            values.last_mut().unwrap().push(char)
        }

        for (idx, value) in values.into_iter().enumerate() {
            if idx >= columns.len() {
                columns.resize_with(idx + 1, Vec::new);
            }
            columns[idx].push(value);
        }
    }

    println!("columns: {columns:?}");
    columns
}

fn count_spaces_between(s: &str, targets: &[char]) -> Vec<usize> {
    let chars: Vec<_> = s.chars().collect();
    let mut result = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if targets.contains(&c) {
            let mut count_spaces = 0;
            let mut j = i + 1;
            while j < chars.len() {
                if targets.contains(&chars[j]) {
                    break;
                }
                if chars[j] == ' ' {
                    count_spaces += 1;
                }
                j += 1;
            }

            if j >= chars.len() {
                count_spaces += 1;
            }

            result.push(count_spaces);
        }
        i += 1;
    }

    result
}
