use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn step(&self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Self::North => (x, y - 1),
            Self::East => (x + 1, y),
            Self::South => (x, y + 1),
            Self::West => (x - 1, y),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Clone)]
struct State {
    grid: HashMap<(i32, i32), Tile>,
    bbox: ((i32, i32), (i32, i32)),
    guard_at: (i32, i32),
    guard_face: Direction,
}

impl State {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            bbox: ((i32::MAX, i32::MAX), (0i32, 0i32)),
            guard_at: (0, 0),
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
            let t = match Tile::from(col) {
                Tile::Guard(d) => {
                    state.guard_at = (x.try_into().unwrap(), y.try_into().unwrap());
                    state.guard_face = d;
                    Tile::Open
                }
                other => other,
            };
            state
                .grid
                .insert((x.try_into().unwrap(), y.try_into().unwrap()), t);
            state.bbox.0 .0 = state.bbox.0 .0.min(x.try_into().unwrap());
            state.bbox.0 .1 = state.bbox.0 .1.min(y.try_into().unwrap());
            state.bbox.1 .0 = state.bbox.1 .0.max(x.try_into().unwrap());
            state.bbox.1 .1 = state.bbox.1 .1.max(y.try_into().unwrap());
        }
    }

    state
}

#[allow(dead_code)]
fn pprint_grid(state: &State) {
    for y in state.bbox.0 .1..=state.bbox.1 .1 {
        for x in state.bbox.0 .0..=state.bbox.1 .0 {
            let t = if (x, y) == state.guard_at {
                &Tile::Guard(state.guard_face)
            } else {
                state.grid.get(&(x, y)).unwrap()
            };
            print!("{}", t);
        }
        println!();
    }
}

fn patrol(state: &State) -> (HashSet<((i32, i32), Direction)>, bool) {
    let mut guard_at = state.guard_at;
    let mut guard_face = state.guard_face;

    let mut visited =
        HashSet::with_capacity((state.bbox.1 .0 * state.bbox.1 .1).try_into().unwrap());
    visited.insert((guard_at, guard_face));

    loop {
        let next_at = guard_face.step(guard_at.0, guard_at.1);

        match state.grid.get(&next_at) {
            Some(t) => match t {
                Tile::Open => {
                    guard_at = next_at;
                    let key = (guard_at, guard_face);
                    if !visited.insert(key) {
                        return (visited, true);
                    }
                }
                Tile::Obstruction => {
                    guard_face = guard_face.rotate_right();
                }
                _ => {}
            },
            None => break,
        }
    }

    (visited, false)
}

// TODO: Could probably use "vectors" instead of individual steps.

fn get_looping_obstacles(state: &State) -> HashSet<(i32, i32)> {
    let mut current = state.clone();
    let (visited, _) = patrol(state);

    let mut looping_obstacles = HashSet::new();

    for ((x, y), _) in visited {
        if (x, y) == state.guard_at {
            continue;
        }
        let old_t = current.grid.insert((x, y), Tile::Obstruction).unwrap();
        let (_, looped) = patrol(&current);
        current.grid.insert((x, y), old_t);
        if looped {
            looping_obstacles.insert((x, y));
        }
    }

    looping_obstacles
}

fn solve(parsed: &State) -> usize {
    get_looping_obstacles(parsed).len()
}

fn main() {
    let input = load_input(2024, 6);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06b_example1() {
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

        assert_eq!(&Tile::Open, parsed.grid.get(&(0, 0)).unwrap());
        assert_eq!(&Tile::Obstruction, parsed.grid.get(&(4, 0)).unwrap());
        assert_eq!(&Tile::Open, parsed.grid.get(&(4, 6)).unwrap());
        assert_eq!(&Tile::Obstruction, parsed.grid.get(&(6, 9)).unwrap());

        assert_eq!((4, 6), parsed.guard_at);
        assert_eq!(Direction::North, parsed.guard_face);

        assert_eq!(((0, 0,), (9, 9)), parsed.bbox);

        let expected_loops = HashSet::from([(3, 6), (6, 7), (7, 7), (1, 8), (3, 8), (7, 9)]);
        let actual_loops = get_looping_obstacles(&parsed);
        assert_eq!(expected_loops.len(), actual_loops.len());
        assert_eq!(expected_loops, actual_loops);

        assert_eq!(6, solve(&parsed));

        assert_eq!((4, 6), parsed.guard_at);
        assert_eq!(Direction::North, parsed.guard_face);
    }
}
