use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, PartialOrd, Ord)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[allow(dead_code)]
impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn rotate_left(&self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }

    fn step(&self, p: &Point2) -> Point2 {
        match self {
            Self::North => Point2::new(p.x, p.y - 1),
            Self::East => Point2::new(p.x + 1, p.y),
            Self::South => Point2::new(p.x, p.y + 1),
            Self::West => Point2::new(p.x - 1, p.y),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug, Clone)]
struct State {
    grid: HashMap<Point2, Tile>,
    bbox: BBox2,
    start_at: Point2,
    end_at: Point2,
}

impl State {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            bbox: BBox2::default(),
            start_at: Point2::min(),
            end_at: Point2::min(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.trim().char_indices() {
            let p = Point2::new(x.try_into().unwrap(), y.try_into().unwrap());
            let t = match ch {
                '#' => Tile::Wall,
                '.' => Tile::Open,
                'S' => {
                    state.start_at = p;
                    Tile::Open
                }
                'E' => {
                    state.end_at = p;
                    Tile::Open
                }
                _ => panic!("Unknow input char: {:?}", ch),
            };
            state.grid.insert(p, t);
            state.bbox.update(&p);
        }
    }

    state
}

fn count_steps(state: &State) -> Vec<(Point2, i32)> {
    let mut path = Vec::new();
    let mut visited = HashSet::new();

    let mut at = state.start_at;
    let mut counter = -1;
    loop {
        counter += 1;
        path.push((at, counter));

        if at == state.end_at {
            break;
        }

        visited.insert(at);

        for d in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let candidate = d.step(&at);
            if state.grid.get(&candidate) == Some(&Tile::Wall) {
                continue;
            }
            if visited.contains(&candidate) {
                continue;
            }
            at = candidate;
            break;
        }
    }

    path
}

/// Return a map of (seconds-saved, count).
fn find_cheats(state: &State) -> HashMap<i32, i32> {
    let start_at = state.start_at;
    let end_at = state.end_at;

    let mut state = state.clone();
    let forward_dists = count_steps(&state);
    let forward_dists_lookup: HashMap<Point2, i32> = forward_dists.into_iter().collect();
    state.start_at = end_at;
    state.end_at = start_at;
    let backward_dists = count_steps(&state);
    let backward_dists_lookup: HashMap<Point2, i32> = backward_dists.into_iter().collect();

    let mut saved = HashMap::new();
    for x in state.bbox.min.x..=state.bbox.max.x {
        for y in state.bbox.min.y..=state.bbox.max.y {
            let p = Point2::new(x, y);
            if state.grid.get(&p) == Some(&Tile::Wall) {
                continue;
            }

            for d in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                let s0 = d.step(&p);
                if !state.bbox.contains(&s0) {
                    continue;
                }
                if state.grid.get(&s0) != Some(&Tile::Wall) {
                    continue;
                }
                let s1 = d.step(&s0);
                if !state.bbox.contains(&s1) {
                    continue;
                }
                if state.grid.get(&s1) == Some(&Tile::Wall) {
                    continue;
                }

                let f = forward_dists_lookup.get(&p).unwrap();
                let b = backward_dists_lookup.get(&s1).unwrap();

                // It takes two steps to travel this opening!
                let cost = f + b + 2;
                let max_cost: i32 = forward_dists_lookup.len().try_into().unwrap();
                // Including start in the direction dists, so subtract one.
                let save = max_cost - 1 - cost;

                if save <= 0 {
                    continue;
                }
                saved.entry(save).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }

    saved
}

fn solve(parsed: &State, save_at_least: i32) -> i32 {
    // Mixed up what the keys and values were a few times here.
    find_cheats(parsed)
        .iter()
        .filter_map(|(k, v)| if *k >= save_at_least { Some(v) } else { None })
        .sum()
}

fn main() {
    let input = load_input(2024, 20);
    let parsed = parse_input(input);
    let answer = solve(&parsed, 100);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20a_example1() {
        let input = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(14, 14)),
            parsed.bbox
        );

        assert_eq!(Point2::new(1, 3), parsed.start_at);
        assert_eq!(Point2::new(5, 7), parsed.end_at);
        assert_eq!(Some(&Tile::Open), parsed.grid.get(&parsed.start_at));
        assert_eq!(Some(&Tile::Open), parsed.grid.get(&parsed.end_at));

        assert_eq!(Some(&Tile::Wall), parsed.grid.get(&parsed.bbox.min));
        assert_eq!(Some(&Tile::Wall), parsed.grid.get(&parsed.bbox.max));

        let steps = count_steps(&parsed);
        assert_eq!(84 + 1, steps.len());
        assert_eq!((parsed.start_at, 0), steps[0]);
        assert_eq!((Direction::North.step(&parsed.start_at), 1), steps[1]);
        assert_eq!((parsed.end_at, 84), steps[steps.len() - 1]);
        assert_eq!(
            (Direction::West.step(&parsed.end_at), 83),
            steps[steps.len() - 2]
        );

        let count_saved = HashMap::from([
            (2, 14),
            (4, 14),
            (6, 2),
            (8, 4),
            (10, 2),
            (12, 3),
            (20, 1),
            (36, 1),
            (38, 1),
            (40, 1),
            (64, 1),
        ]);
        assert_eq!(count_saved, find_cheats(&parsed));

        assert_eq!(0, solve(&parsed, i32::MAX));
        assert_eq!(count_saved.values().sum::<i32>(), solve(&parsed, 1));
    }
}

