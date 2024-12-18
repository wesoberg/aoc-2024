use std::collections::HashMap;

use aoc_2024_rs::*;
use regex::Regex;

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

    #[allow(dead_code)]
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

    fn contains(&self, p: &Point2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    grid: Vec<(Point2, Point2)>,
    bbox: BBox2,
}

impl State {
    fn new() -> Self {
        Self {
            grid: Vec::new(),
            bbox: BBox2::default(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    let re_bot = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();

    for line in input.trim().lines() {
        if let Some(caps) = re_bot.captures(line) {
            let p = Point2::new(
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            );
            let v = Point2::new(
                caps.get(3).unwrap().as_str().parse().unwrap(),
                caps.get(4).unwrap().as_str().parse().unwrap(),
            );
            state.grid.push((p, v));
            state.bbox.update(&p);
        }
    }

    state
}

#[allow(dead_code)]
fn pprint_grid(state: &State) {
    let mut grid: HashMap<Point2, usize> = HashMap::new();
    for (p, _) in &state.grid {
        grid.entry(*p).and_modify(|c| *c += 1).or_insert(1);
    }
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            if let Some(c) = grid.get(&Point2::new(x, y)) {
                print!("{}", c);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn step(p: &Point2, v: &Point2, bbox: &BBox2) -> Point2 {
    let mut q = *p;
    q.x += v.x;
    q.y += v.y;

    if q.x < 0 {
        q.x += bbox.max.x + 1;
    } else if q.x > bbox.max.x {
        q.x -= bbox.max.x + 1;
    }

    if q.y < 0 {
        q.y += bbox.max.y + 1;
    } else if q.y > bbox.max.y {
        q.y -= bbox.max.y + 1;
    }

    q
}

fn tick(state: &State) -> State {
    let mut next = State::new();
    next.bbox = state.bbox.clone();

    for (p, v) in &state.grid {
        next.grid.push((step(p, v, &next.bbox), *v));
    }

    next
}

fn solve(parsed: &State, steps: usize) -> usize {
    let mut state: State = parsed.clone();
    for _ in 0..steps {
        state = tick(&state);
    }

    let mid = Point2::new(state.bbox.max.x / 2, state.bbox.max.y / 2);

    let q1 = BBox2::new(&Point2::new(0, 0), &Point2::new(mid.x - 1, mid.y - 1));
    let q2 = BBox2::new(
        &Point2::new(mid.x + 1, 0),
        &Point2::new(state.bbox.max.x, mid.y - 1),
    );
    let q3 = BBox2::new(
        &Point2::new(0, mid.y + 1),
        &Point2::new(mid.x - 1, state.bbox.max.y),
    );
    let q4 = BBox2::new(
        &Point2::new(mid.x + 1, mid.y + 1),
        &Point2::new(state.bbox.max.x, state.bbox.max.y),
    );

    let mut c1 = 0;
    let mut c2 = 0;
    let mut c3 = 0;
    let mut c4 = 0;
    for (p, _) in state.grid {
        if q1.contains(&p) {
            c1 += 1;
        } else if q2.contains(&p) {
            c2 += 1;
        } else if q3.contains(&p) {
            c3 += 1;
        } else if q4.contains(&p) {
            c4 += 1;
        }
    }

    c1 * c2 * c3 * c4
}

fn main() {
    let input = load_input(2024, 14);
    let parsed = parse_input(input);
    let answer = solve(&parsed, 100);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14a_example1() {
        let input = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(10, 6)),
            parsed.bbox
        );

        assert_eq!(
            (Point2::new(0, 4), Point2::new(3, -3)),
            *parsed.grid.get(0).unwrap()
        );
        assert_eq!(
            (Point2::new(6, 3), Point2::new(-1, -3)),
            *parsed.grid.get(1).unwrap()
        );
        assert_eq!(
            (Point2::new(9, 5), Point2::new(-3, -3)),
            *parsed.grid.get(11).unwrap()
        );

        assert_eq!(12, solve(&parsed, 100));
    }

    #[test]
    fn day14a_example2() {
        let input = "
p=2,4 v=2,-3
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            State {
                grid: vec![(Point2::new(2, 4), Point2::new(2, -3))],
                bbox: BBox2::new(&Point2::new(2, 4), &Point2::new(2, 4))
            },
            parsed
        );

        let mut state = parsed.clone();
        state.bbox.min = Point2::new(0, 0);
        state.bbox.max = Point2::new(10, 6);

        state = tick(&state);
        assert_eq!(vec![(Point2::new(4, 1), Point2::new(2, -3))], state.grid);
        state = tick(&state);
        assert_eq!(vec![(Point2::new(6, 5), Point2::new(2, -3))], state.grid);
        state = tick(&state);
        assert_eq!(vec![(Point2::new(8, 2), Point2::new(2, -3))], state.grid);
        state = tick(&state);
        assert_eq!(vec![(Point2::new(10, 6), Point2::new(2, -3))], state.grid);
        state = tick(&state);
        assert_eq!(vec![(Point2::new(1, 3), Point2::new(2, -3))], state.grid);
    }
}
