const INPUT: &str = include_str!("../../inputs/day_9/input");

fn main() {
    for (idx, part) in [part_1, part_2].iter().enumerate() {
        let res = part();
        println!("[D9 P{}] Answer: {res}", idx + 1);
    }
}

fn part_1() -> i64 {
    let points: Vec<_> = INPUT.lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();

            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();

            (x, y)
        })
        .collect();

    let mut max_area = 0;

    for (i, &(xi, yi)) in points.iter().enumerate() {
        for &(xj, yj) in points.iter().skip(i + 1) {
            let width = xj.abs_diff(xi) + 1;
            let height = yj.abs_diff(yi) + 1;
            let area = width * height;

            println!("point_i: ({xi};{yi}); point_j: ({xj};{yj}); area: {area}");

            max_area = max_area.max(area);
        }
    }

    max_area as i64
}

fn part_2() -> i64 {
    let mut res = 0;

    res
}
