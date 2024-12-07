use rustc_hash::FxHashSet;

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
        Self::new(0, 0)
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
    fn new(min: Point2, max: Point2) -> Self {
        BBox2 { min, max }
    }

    fn default() -> Self {
        BBox2 {
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
    obstructions: FxHashSet<Point2>,
    bbox: BBox2,
    guard_at: Point2,
    guard_face: Direction,
}

impl State {
    fn new() -> Self {
        Self {
            obstructions: FxHashSet::default(),
            bbox: BBox2::default(),
            guard_at: Point2::new(0, 0),
            guard_face: Direction::North,
        }
    }

    fn get(&self, p: &Point2) -> Option<Tile> {
        if !self.bbox.contains(p) {
            None
        } else if self.obstructions.contains(&p) {
            Some(Tile::Obstruction)
        } else if *p == self.guard_at {
            Some(Tile::Guard(self.guard_face))
        } else {
            Some(Tile::Open)
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
            match Tile::from(col) {
                Tile::Guard(d) => {
                    state.guard_at = p;
                    state.guard_face = d;
                }
                Tile::Obstruction => {
                    state.obstructions.insert(p);
                }
                Tile::Open => {}
            };
            state.bbox.update(&p);
        }
    }

    state
}

#[allow(dead_code)]
fn pprint_grid(state: &State) {
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            print!("{}", state.get(&Point2::new(x, y)).unwrap());
        }
        println!();
    }
}

fn patrol(state: &State) -> (FxHashSet<(Point2, Direction)>, bool) {
    let mut guard_at = state.guard_at;
    let mut guard_face = state.guard_face;

    let mut visited = FxHashSet::default();
    visited.reserve((state.bbox.max.x * state.bbox.max.y).try_into().unwrap());
    visited.insert((guard_at, guard_face));

    loop {
        let next_at = guard_face.step(&guard_at);

        match state.get(&next_at) {
            Some(Tile::Obstruction) => {
                guard_face = guard_face.rotate_right();
            }
            Some(Tile::Open | Tile::Guard(_)) => {
                guard_at = next_at;
                if !visited.insert((guard_at, guard_face)) {
                    return (visited, true);
                }
            }
            None => break,
        }
    }

    (visited, false)
}

// TODO: Could probably use "vectors" instead of individual steps?
// Or maybe pre-compute jumps?
//
// TODO: Hashing is still the slowest part, even after switching to one of the supposedly fastest
// hashing algorithm crates out there. May want to try Vec<Vec<bool>> and equivalents, just to see
// how it performs in comparison. Maybe Vec<bool> (flattened with access formula) would be even
// better than that? Maybe even bitwise masks even better (can you do an arbitrary series of bytes
// in Rust? well there's probably a crate)? Could potentially do bytes and have enum flags in there
// for directions and such?

fn get_looping_obstacles(state: &State) -> FxHashSet<Point2> {
    let mut current = state.clone();
    let (visited, _) = patrol(state);

    let mut looping_obstacles = FxHashSet::default();

    for (p, _) in visited {
        if p == state.guard_at {
            continue;
        }
        current.obstructions.insert(p);
        let (_, looped) = patrol(&current);
        current.obstructions.remove(&p);
        if looped {
            looping_obstacles.insert(p);
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

        assert_eq!(Some(Tile::Open), parsed.get(&Point2::new(0, 0)));
        assert_eq!(Some(Tile::Obstruction), parsed.get(&Point2::new(4, 0)));
        assert_eq!(
            Some(Tile::Guard(Direction::North)),
            parsed.get(&Point2::new(4, 6))
        );
        assert_eq!(Some(Tile::Obstruction), parsed.get(&Point2::new(6, 9)));

        assert_eq!(Point2::new(4, 6), parsed.guard_at);
        assert_eq!(Direction::North, parsed.guard_face);

        assert_eq!(
            BBox2::new(Point2::new(0, 0), Point2::new(9, 9)),
            parsed.bbox
        );

        let expected_loops = vec![
            Point2::new(3, 6),
            Point2::new(6, 7),
            Point2::new(7, 7),
            Point2::new(1, 8),
            Point2::new(3, 8),
            Point2::new(7, 9),
        ]
        .iter()
        .cloned()
        .collect::<FxHashSet<Point2>>();
        let actual_loops = get_looping_obstacles(&parsed);
        assert_eq!(expected_loops.len(), actual_loops.len());
        assert_eq!(expected_loops, actual_loops);

        assert_eq!(6, solve(&parsed));

        assert_eq!(Point2::new(4, 6), parsed.guard_at);
        assert_eq!(Direction::North, parsed.guard_face);
    }
}
