use std::hash::Hash;

const INPUT: &str = include_str!("../../inputs/day_8/input");

fn main() {
    for (idx, part) in [part_1, part_2].iter().enumerate() {
        let res = part();
        println!("[D8 P{}] Answer: {res}", idx + 1);
    }
}

fn part_1() -> i64 {
    let junction_boxes: Vec<_> = INPUT.lines().map(Vector3::from_line).collect();
    let n = junction_boxes.len();

    // Create all pairs of distances
    let mut edges = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = junction_boxes[i].distance_sqr(&junction_boxes[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort by distance
    edges.sort_by_key(|&(dist, _, _)| dist);

    // Union-Find structure
    let mut uf = UnionFind::new(n);

    let mut successful_connects = 0;
    let target_connects = 1000; // 1000 for real input

    for &(_, i, j) in &edges {
        if successful_connects == target_connects {
            break;
        }

        successful_connects += 1;

        if uf.union(i, j) {

        }
    }

    // Get all set sizes and find the 3 largest
    let mut sizes = uf.all_set_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    // Multiply the three largest sizes
    let product: usize = sizes.iter().take(3).product();

    product as i64
}

fn part_2() -> i64 {
    let junction_boxes: Vec<_> = INPUT.lines().map(Vector3::from_line).collect();
    let n = junction_boxes.len();

    // Create all pairs of distances
    let mut edges = vec![];
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = junction_boxes[i].distance_sqr(&junction_boxes[j]);
            edges.push((dist, i, j));
        }
    }

    // Sort by distance
    edges.sort_by_key(|&(dist, _, _)| dist);

    // Union-Find structure
    let mut uf = UnionFind::new(n);

    let mut components = n; // Initially, each junction box is its own component
    let mut last_connection_indices = (0, 0); // Store indices of last connection

    // Keep connecting until we have only 1 component
    for &(_, i, j) in &edges {
        if components == 1 {
            break; // All connected
        }

        if uf.union(i, j) {
            components -= 1; // Successfully merged two components
            if components == 1 {
                // This is the connection that made everything connected
                last_connection_indices = (i, j);
            }
        }
    }

    // Get the two junction boxes from the last connection
    let box1 = &junction_boxes[last_connection_indices.0];
    let box2 = &junction_boxes[last_connection_indices.1];

    // Multiply their X coordinates
    (box1.x * box2.x) as i64
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Vector3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Vector3 {
    pub fn from_line(line: &str) -> Self {
        let mut parts = line.splitn(3, ',');

        Self {
            x: parts.next().unwrap().parse::<usize>().unwrap(),
            y: parts.next().unwrap().parse::<usize>().unwrap(),
            z: parts.next().unwrap().parse::<usize>().unwrap(),
        }
    }

    pub fn distance_sqr(&self, rhs: &Self) -> usize {
        let x_diff = rhs.x - self.x;
        let y_diff = rhs.y - self.y;
        let z_diff = rhs.z - self.z;

        let x_sqr = x_diff * x_diff;
        let y_sqr = y_diff * y_diff;
        let z_sqr = z_diff * z_diff;

        x_sqr + y_sqr + z_sqr
    }

    pub fn distance(&self, rhs: &Self) -> f64 {
        (self.distance_sqr(rhs) as f64).sqrt()
    }
}

// #[derive(Debug)]
// struct JunctionBox {
//     position: Vector3,
//     circuit: Option<Vec<Self>>,
// }
//
// impl JunctionBox {
//     pub fn from_line(line: &str) -> Self {
//         Self {
//             position: Vector3::from_line(line),
//             circuit: None
//         }
//     }
//
//     pub fn distance_sqr(&self, rhs: &Self) -> usize {
//         self.position.distance_sqr(&rhs.position)
//     }
//
//     pub fn distance(&self, rhs: &Self) -> f64 {
//         self.position.distance(&rhs.position)
//     }
// }
//
// impl PartialEq<Self> for JunctionBox {
//     fn eq(&self, other: &Self) -> bool {
//         self.position == other.position
//     }
// }
//
// impl Eq for JunctionBox { }
//
// impl Hash for JunctionBox {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.position.hash(state)
//     }
// }

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // each element is its own parent
            size: vec![1; n],         // each set starts with size 1
        }
    }

    // Find the root of x with path compression
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // Union the sets of x and y
    // Returns true if union was successful (they were separate)
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false; // already in the same set
        }

        // Union by size: attach smaller tree to larger
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }
        true
    }

    // Get the size of the set containing x
    fn set_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }

    // Get all unique set sizes
    fn all_set_sizes(&mut self) -> Vec<usize> {
        let mut seen = std::collections::HashSet::new();
        let mut sizes = vec![];
        for i in 0..self.parent.len() {
            let root = self.find(i);
            if seen.insert(root) {
                sizes.push(self.size[root]);
            }
        }
        sizes
    }
}
