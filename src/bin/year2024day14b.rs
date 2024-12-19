use std::collections::HashMap;

use aoc_2024_rs::*;
use regex::Regex;

const DEBUG: bool = false;

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

    #[allow(dead_code)]
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
    let mut grid: HashMap<Point2, usize> =
        HashMap::with_capacity((state.bbox.max.x * state.bbox.max.y).try_into().unwrap());
    for (p, _) in &state.grid {
        grid.entry(*p).and_modify(|c| *c += 1).or_insert(1);
    }
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            if let Some(c) = grid.get(&Point2::new(x, y)) {
                print!("{}", c);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn step(p: &Point2, v: &Point2, bbox: &BBox2) -> Point2 {
    // Hilarious that I didn't want to "bother" making it modulo before, so had a lot more code
    // here to check all the min/max bounds and perform offset corrections.
    Point2::new(
        (p.x + v.x).rem_euclid(bbox.max.x + 1),
        (p.y + v.y).rem_euclid(bbox.max.y + 1),
    )
}

fn tick(state: &State) -> State {
    let mut next = State::new();
    next.bbox = state.bbox.clone();

    for (p, v) in &state.grid {
        next.grid.push((step(p, v, &next.bbox), *v));
    }

    next
}

fn longest_continuous_column(state: &State, x: i32) -> usize {
    let mut ys: Vec<i32> = state
        .grid
        .iter()
        .filter_map(|(p, _)| if p.x == x { Some(p.y) } else { None })
        .collect();
    ys.sort();

    let mut longest = 0;
    let mut candidate = 0;
    for i in 1..ys.len() {
        if ys[i - 1] + 1 == ys[i] {
            candidate += 1;
            if candidate > longest {
                longest = candidate;
            }
        } else {
            candidate = 0;
        }
    }

    longest
}

fn solve(parsed: &State) -> usize {
    // Stepping through 4000 something rounds, there do appear to be some patterns. First, it looks
    // like there's a border that is consistently at the same x positions (35 and 65) when the
    // image is vertical. The pattern flips diagonally every so often, stretching along y, then
    // along x, etc. After seeing the easter egg materialize, it is centered enough to try to do a
    // flood fill to be sure it is found again. Did that, but looking for the longest continuous
    // group along a given known x is much faster.
    //
    // I tried using a HashMap<Point2, Vec<Point2>> but cloning the Vec of tuples is just so much
    // faster. Setting a capacity whenever HashMap is used always helps...

    let mut step_seen = 0;
    let mut most_seen = 0;

    // Forgot about this trick and heard it was mentioned on the sub. Since the pathing is fixed,
    // and pathing loops around the space, the upper bound can be W*H, because states must repeat
    // after that. Does this only work if the dimensions are coprime?
    let upper_bound: usize = ((parsed.bbox.max.x + 1) * (parsed.bbox.max.y + 1))
        .try_into()
        .unwrap();

    let mut state: State = parsed.clone();
    for step in 1..upper_bound {
        state = tick(&state);

        let this_count = longest_continuous_column(&state, 65);
        if this_count > most_seen {
            step_seen = step;
            most_seen = this_count;
            if DEBUG {
                println!("Seconds: {:?}, Count: {:?}", step, most_seen);
                pprint_grid(&state);
                pause();
            }
        }
    }

    step_seen
}

fn main() {
    let input = load_input(2024, 14);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14b_example1() {
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
    }

    #[test]
    fn day14b_example2() {
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
