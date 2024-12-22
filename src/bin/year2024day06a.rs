use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

#[derive(Debug, PartialEq)]
enum Tile {
    Open,
    Obstruction,
    Guard(Direction),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Open,
            '#' => Self::Obstruction,
            '^' => Self::Guard(Direction::North),
            '>' => Self::Guard(Direction::East),
            'v' => Self::Guard(Direction::South),
            '<' => Self::Guard(Direction::West),
            other => panic!("Unknown tile char: '{}'", other),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "."),
            Self::Obstruction => write!(f, "#"),
            Self::Guard(Direction::North) => write!(f, "^"),
            Self::Guard(Direction::East) => write!(f, ">"),
            Self::Guard(Direction::South) => write!(f, "v"),
            Self::Guard(Direction::West) => write!(f, "<"),
        }
    }
}

#[derive(Debug)]
struct State {
    grid: HashMap<Point2<i32>, Tile>,
    bbox: BBox2<i32>,
    guard_at: Point2<i32>,
    guard_face: Direction,
}

impl State {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            bbox: BBox2::default(),
            guard_at: Point2::new(0, 0),
            guard_face: Direction::North,
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for (y, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        for (x, col) in line.trim().char_indices() {
            let p = Point2::new(x.try_into().unwrap(), y.try_into().unwrap());
            let t = match Tile::from(col) {
                Tile::Guard(d) => {
                    state.guard_at = p;
                    state.guard_face = d;
                    Tile::Open
                }
                other => other,
            };
            state.grid.insert(p, t);
            state.bbox.update(&p);
        }
    }

    state
}

#[allow(dead_code)]
fn pprint_grid(state: &State) {
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            print!("{}", state.grid.get(&Point2::new(x, y)).unwrap());
        }
        println!();
    }
}

fn patrol(state: &State) -> HashSet<Point2<i32>> {
    let mut guard_at = state.guard_at;
    let mut guard_face = state.guard_face;

    let mut visited = HashSet::new();
    visited.insert(guard_at);

    loop {
        let next_at = guard_face.step(&guard_at);

        match state.grid.get(&next_at) {
            Some(t) => match t {
                Tile::Open => {
                    guard_at = next_at;
                    visited.insert(guard_at);
                }
                Tile::Obstruction => {
                    guard_face = guard_face.rotate_right();
                }
                _ => {}
            },
            None => break,
        }
    }

    visited
}

fn solve(parsed: State) -> usize {
    patrol(&parsed).len()
}

fn main() {
    let input = load_input(2024, 6);
    let parsed = parse_input(input);
    let answer = solve(parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06a_example1() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        pprint_grid(&parsed);

        assert_eq!(&Tile::Open, parsed.grid.get(&Point2::new(0, 0)).unwrap());
        assert_eq!(
            &Tile::Obstruction,
            parsed.grid.get(&Point2::new(4, 0)).unwrap()
        );
        assert_eq!(&Tile::Open, parsed.grid.get(&Point2::new(4, 6)).unwrap());
        assert_eq!(
            &Tile::Obstruction,
            parsed.grid.get(&Point2::new(6, 9)).unwrap()
        );

        assert_eq!(Point2::new(4, 6), parsed.guard_at);
        assert_eq!(Direction::North, parsed.guard_face);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(9, 9)),
            parsed.bbox
        );

        assert_eq!(41, solve(parsed));
    }
}
