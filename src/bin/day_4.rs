const INPUT: &str = include_str!("../../inputs/day_4/input");

fn main() {
    // let res = part_1();
    let res = part_2();
    println!("Answer: {res}");
}

fn part_1() -> i64 {
    let mut res = 0;
    let grid = Grid::new();

    for (y, row) in grid.data.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_none() { continue; }

            let rolls_count = check_neighbors(&grid, x as isize, y as isize);

            println!("({x};{y}) rolls count: {rolls_count}");

            if rolls_count < 4 {
                res += 1;
            }
        }
    }

    res
}

fn part_2() -> i64 {
    let mut res = 0;
    let mut grid = Grid::new();

    let mut to_remove: Vec<(usize, usize)> = Vec::new();

    loop {
        for (y, row) in grid.data.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_none() { continue; }

                let rolls_count = check_neighbors(&grid, x as isize, y as isize);

                println!("({x};{y}) rolls count: {rolls_count}");

                if rolls_count < 4 {
                    to_remove.push((y, x));
                    res += 1;
                }
            }
        }

        if to_remove.is_empty() { break; }

        for (y, x) in &to_remove {
            grid.data[*y][*x].take();
        }

        to_remove.clear();
    }

    res
}

fn check_neighbors(grid: &Grid, x: isize, y: isize) -> u8 {
    let mut neighbors = 0;

    for y_delta in -1..=1 {
        let y = y + y_delta;
        if y < 0 || y >= grid.height { continue; }
        let y = y as usize;

        for x_delta in -1..=1 {
            if y_delta == 0 && x_delta == 0 { continue; }

            let x = x + x_delta;
            if x < 0 || x >= grid.width { continue; }
            let x = x as usize;

            if grid.data[y][x].is_some() {
                neighbors += 1;
            }
        }
    }

    neighbors
}

struct Grid {
    pub data: Vec<Vec<Option<()>>>,
    pub height: isize,
    pub width: isize,
}

impl Grid {
    pub fn new() -> Self {
        let data: Vec<Vec<_>> = INPUT
            .lines()
            .map(|line| line
                .chars()
                .map(|ch| if ch == '@' { Some(()) } else { None })
                .collect())
            .collect();

        let height = data.len() as isize;
        let width = data[0].len() as isize;

        Grid { data, height, width }
    }
}
