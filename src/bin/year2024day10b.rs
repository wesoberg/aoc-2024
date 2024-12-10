use std::collections::HashMap;

use aoc_2024_rs::*;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Point2 {
    x: i32,
    y: i32,
}

impl Point2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn min() -> Self {
        Self::new(i32::MIN, i32::MIN)
    }

    fn max() -> Self {
        Self::new(i32::MAX, i32::MAX)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct BBox2 {
    min: Point2,
    max: Point2,
}

impl BBox2 {
    #[allow(dead_code)]
    fn new(a: &Point2, b: &Point2) -> Self {
        Self {
            min: Point2::new(a.x.min(b.x), a.y.min(b.y)),
            max: Point2::new(a.x.max(b.x), a.y.max(b.y)),
        }
    }

    fn default() -> Self {
        Self {
            min: Point2::max(),
            max: Point2::min(),
        }
    }

    fn update(&mut self, p: &Point2) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
    }

    #[allow(dead_code)]
    fn contains(&self, p: &Point2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(&self, p: &Point2) -> Point2 {
        match self {
            Self::North => Point2::new(p.x, p.y - 1),
            Self::East => Point2::new(p.x + 1, p.y),
            Self::South => Point2::new(p.x, p.y + 1),
            Self::West => Point2::new(p.x - 1, p.y),
        }
    }
}

struct State {
    grid: HashMap<Point2, u32>,
    bbox: BBox2,
}

impl State {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            bbox: BBox2::default(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for (y, row) in input
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
    {
        for (x, col) in row.trim().char_indices() {
            match col {
                '.' => {}
                c => {
                    let p = Point2::new(x.try_into().unwrap(), y.try_into().unwrap());
                    state.grid.insert(p, c.to_digit(10).unwrap());
                    state.bbox.update(&p);
                }
            }
        }
    }

    state
}

fn get_neighbors(state: &State, at: &Point2) -> Vec<Point2> {
    let v = state.grid.get(at).unwrap();

    let steps = [
        Direction::North.step(at),
        Direction::East.step(at),
        Direction::South.step(at),
        Direction::West.step(at),
    ];

    let mut neighbors = Vec::new();
    for step in steps {
        if let Some(step_v) = state.grid.get(&step) {
            if step_v > v && step_v - v == 1 {
                neighbors.push(step);
            }
        }
    }

    neighbors
}

fn get_trailheads(state: &State) -> HashMap<Point2, usize> {
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            if let Some(v) = state.grid.get(&p) {
                match v {
                    0 => starts.push(p),
                    9 => ends.push(p),
                    _ => {}
                }
            }
        }
    }

    let mut pairs = HashMap::new();
    for start in starts {
        let mut unvisited = vec![start];
        while let Some(at) = unvisited.pop() {
            for n in get_neighbors(state, &at) {
                if ends.contains(&n) {
                    pairs
                        .entry(start)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                } else {
                    unvisited.push(n);
                }
            }
        }
    }

    pairs
}

fn solve(parsed: &State) -> usize {
    get_trailheads(parsed).values().sum()
}

fn main() {
    let input = load_input(2024, 10);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10b_example1() {
        let input = "
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let trailheads = get_trailheads(&parsed);
        assert_eq!(HashMap::from([(Point2::new(5, 0), 3)]), trailheads);

        assert_eq!(3, solve(&parsed));
    }

    #[test]
    fn day10b_example2() {
        let input = "
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let trailheads = get_trailheads(&parsed);
        assert_eq!(HashMap::from([(Point2::new(3, 0), 13)]), trailheads);

        assert_eq!(13, solve(&parsed));
    }

    #[test]
    fn day10b_example3() {
        let input = "
012345
123456
234567
345678
4.6789
56789.
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let trailheads = get_trailheads(&parsed);
        assert_eq!(HashMap::from([(Point2::new(0, 0), 227)]), trailheads);

        assert_eq!(227, solve(&parsed));
    }

    #[test]
    fn day10b_example4() {
        let input = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(81, solve(&parsed));
    }
}
