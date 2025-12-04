const INPUT: &str = include_str!("../../inputs/day_4/input");

fn main() {
    let res = part_1();
    // let res = part_2();
    println!("Answer: {res}");
}

fn part_1() -> i64 {
    let mut res = 0;
    let grid: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line
            .chars()
            .map(|ch| if ch == '@' { Some(()) } else { None })
            .collect())
        .collect();

    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let check_neighbors = |x: isize, y: isize| -> u8 {
        let mut neighbors = 0;

        for y_delta in -1..=1 {
            let y = y + y_delta;
            if y < 0 || y >= height { continue; }
            let y = y as usize;

            for x_delta in -1..=1 {
                if y_delta == 0 && x_delta == 0 { continue; }

                let x = x + x_delta;
                if x < 0 || x >= width { continue; }
                let x = x as usize;

                if grid[y][x].is_some() {
                    neighbors += 1;
                }
            }
        }

        neighbors
    };

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_none() { continue; }

            let rolls_count = check_neighbors(x as isize, y as isize);

            println!("({x};{y}) rolls count: {rolls_count}");

            if rolls_count < 4 {
                res += 1;
            }
        }
    }

    res
}
