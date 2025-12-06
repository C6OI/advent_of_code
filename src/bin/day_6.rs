use std::sync::Mutex;

const INPUT: &str = include_str!("../../inputs/day_6/input");

fn main() {
    let res = part_1();
    // let res = part_2();
    println!("Answer: {res}");
}

fn part_1() -> i64 {
    let mut res = 0;

    let mut columns: Vec<Mutex<Vec<String>>> = Vec::new();

    for line in INPUT.lines() {
        let values = line
            .split(' ')
            .filter(|str| !str.is_empty())
            .collect::<Vec<_>>();

        columns.reserve(values.len());

        for (idx, value) in values.iter().enumerate() {
            if let Some(column_vec) = columns.get(idx).as_mut() {
                column_vec.lock().unwrap().push(value.trim().to_string());
            } else {
                let mut column_vec = Vec::with_capacity(4);
                column_vec.push(value.trim().to_string());
                columns.push(Mutex::new(column_vec));
            }
        }
    }

    for column in columns {
        let column = column.lock().unwrap();
        println!("{:?}", column);
        let numbers = column.iter().take(column.len() - 1).map(|value| value.parse::<i64>().unwrap());
        let action = column.last().unwrap();

        if action == "+" {
            res += numbers.sum::<i64>();
        } else if action == "*" {
            res += numbers.product::<i64>()
        } else {
            panic!("Invalid action '{action}'")
        }
    }

    res
}

fn part_2() -> i64 {
    let mut res = 0;

    res
}
