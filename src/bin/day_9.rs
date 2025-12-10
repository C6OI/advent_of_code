use std::cmp::{max, min};

const INPUT: &str = include_str!("../../inputs/day_9/input");

fn main() {
    for (idx, part) in [part_1, part_2].iter().enumerate() {
        let res = part();
        println!("[D9 P{}] Answer: {res}", idx + 1);
    }
}

fn part_1() -> i64 {
    let points: Vec<_> = INPUT
        .lines()
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

            // println!("point_i: ({xi};{yi}); point_j: ({xj};{yj}); area: {area}");

            max_area = max_area.max(area);
        }
    }

    max_area as i64
}

fn part_2() -> i64 {
    let points: Vec<_> = INPUT
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();

            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();

            Point { x, y }
        })
        .collect();

    // build edges
    let mut edges = Vec::new();
    for i in 0..points.len() {
        let a = points[i];
        let b = points[(i + 1) % points.len()];
        edges.push(Edge::from_points(a, b));
    }

    // build all candidate rectangles (pairs of red tiles), sorted by descending area
    let mut rects: Vec<Rect> = Vec::new();
    for (i, &a) in points.iter().enumerate() {
        for &b in points.iter().skip(i + 1) {
            rects.push(Rect::from_points(a, b));
        }
    }
    rects.sort_by_key(|r| -r.area());

    // test
    for r in rects {
        if let Some(inner) = r.inner() {
            // check edges: none must intersect the inner rectangle
            let mut ok = true;
            for &edge in &edges {
                if edge.intersects_rect(&inner) {
                    ok = false;
                    break;
                }
            }
            if ok {
                return r.area();
            }
        } else {
            // too small to have inner area: border touches — but that’s fine
            // We might accept rectangles of minimal size (1‑tile thick in one dimension),
            // depending on problem definition. Optionally decide based on spec.
            return r.area();
        }
    }
    0
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Rect {
    x0: usize,
    y0: usize, // inclusive min corner
    x1: usize,
    y1: usize, // inclusive max corner
}

impl Rect {
    fn from_points(a: Point, b: Point) -> Self {
        Rect {
            x0: min(a.x, b.x),
            y0: min(a.y, b.y),
            x1: max(a.x, b.x),
            y1: max(a.y, b.y),
        }
    }
    fn area(&self) -> i64 {
        let w = (self.x1 - self.x0 + 1) as i64;
        let h = (self.y1 - self.y0 + 1) as i64;
        w * h
    }
    /// inner rect (excluding the border)
    fn inner(&self) -> Option<Rect> {
        if self.x1 - self.x0 < 2 || self.y1 - self.y0 < 2 {
            None
        } else {
            Some(Rect {
                x0: self.x0 + 1,
                y0: self.y0 + 1,
                x1: self.x1 - 1,
                y1: self.y1 - 1,
            })
        }
    }
}

/// A border edge between two consecutive red tiles.
/// Always horizontal or vertical.
#[derive(Debug, Clone, Copy)]
struct Edge {
    r: Rect, // a 1‑thick rectangle representing the edge
}

impl Edge {
    fn from_points(a: Point, b: Point) -> Self {
        // one of x or y must be equal
        if a.x == b.x {
            let y0 = min(a.y, b.y);
            let y1 = max(a.y, b.y);
            // vertical line at column x = a.x
            Edge {
                r: Rect {
                    x0: a.x,
                    x1: a.x,
                    y0,
                    y1,
                },
            }
        } else if a.y == b.y {
            let x0 = min(a.x, b.x);
            let x1 = max(a.x, b.x);
            // horizontal line at row y = a.y
            Edge {
                r: Rect {
                    y0: a.y,
                    y1: a.y,
                    x0,
                    x1,
                },
            }
        } else {
            panic!(
                "Non-straight border segment between red tiles: {:?} {:?}",
                a, b
            );
        }
    }
    /// returns true if this edge intersects (overlaps) the given rect
    fn intersects_rect(&self, other: &Rect) -> bool {
        // two rectangles overlap if their projections overlap
        !(self.r.x1 < other.x0
            || self.r.x0 > other.x1
            || self.r.y1 < other.y0
            || self.r.y0 > other.y1)
    }
}
