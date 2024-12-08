use aoc_2024_rs::*;
use rustc_hash::{FxHashMap, FxHashSet};

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

/// Now you're thinking with portals!
#[derive(Debug, Clone)]
struct Navigator {
    transitions: FxHashMap<(Point2, Direction), (Point2, Direction)>,
    prev: FxHashMap<(Point2, Direction), (Point2, Direction)>,
}

impl Navigator {
    fn new() -> Self {
        Self {
            transitions: FxHashMap::default(),
            prev: FxHashMap::default(),
        }
    }

    /// Walk a straight line in the given direction and return the end point, which could either be
    /// on an edge (another step in this direction would take you out of bounds) or right before an
    /// obstruction (another step would be blocked).
    fn get_end(&self, state: &State, at: &Point2, d: &Direction) -> Point2 {
        let mut end = *at;
        loop {
            let step = d.step(&end);
            if !state.bbox.contains(&step) || state.obstructions.contains(&step) {
                break;
            }
            end = step;
        }
        end
    }

    /// These are the entrances and exits of the portals.
    /// Get the ((start, facing), (end, facing)) pairs around the given point, by walking out from
    /// each of the point's neighbors, assuming the given point is an obstruction that would
    /// require rotating out from. For example, if `p` is (0,0), the result could contain up to
    /// four pairs. One pair may be:
    /// * Traveling Noth to encounter `p` from its South, (0,-1), and walking East, (1,-1), (2,-1),
    ///   etc., until hitting an obstruction at (5,-1), resulting in (((0,-1),N),((4,-1),E)).
    fn get_transitions_around(
        &self,
        state: &State,
        obstruction: &Point2,
    ) -> Vec<((Point2, Direction), (Point2, Direction))> {
        let mut transitions = Vec::new();
        for d_edge in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let start = d_edge.step(obstruction);
            if !state.bbox.contains(&start) || state.obstructions.contains(&start) {
                continue;
            }
            let d_enter = d_edge.rotate_right().rotate_right();
            let d_exit = d_enter.rotate_right();
            let end = self.get_end(state, &start, &d_exit);
            transitions.push(((start, d_enter), (end, d_exit)));
        }
        transitions
    }

    /// Mutate state to build all transition points from scratch.
    fn rebuild_all_transitions(&mut self, state: &State) {
        self.prev.clear();
        self.transitions.clear();
        for p in &state.obstructions {
            for (start, end) in self.get_transitions_around(state, p) {
                self.transitions.insert(start, end);
            }
        }
        let end = self.get_end(state, &state.guard_at, &state.guard_face);
        self.transitions
            .insert((state.guard_at, state.guard_face), (end, state.guard_face));
    }

    /// Mutate state to recreate transition points around new this obstruction.
    fn add_obstruction(&mut self, state: &State, obstruction: &Point2) {
        // Removing an obstruction is a rewind to the previous state because I got tired of dealing
        // with bugs in the actual logic for incremental removal and transition rebuilding.
        self.prev = self.transitions.clone();

        if !state.obstructions.contains(obstruction) {
            panic!("Obstruction must already be in state!");
        }

        let mut to_remove: Vec<(Point2, Direction)> = Vec::new();
        let mut to_add: Vec<((Point2, Direction), (Point2, Direction))> = Vec::new();

        for ((start, d_enter), (end, d_exit)) in self.transitions.iter() {
            if BBox2::new(start, end).contains(obstruction) {
                // Recreate this intersected segment.
                to_remove.push((*start, *d_enter));
                let new_end = self.get_end(state, start, d_exit);
                to_add.push(((*start, *d_enter), (new_end, *d_exit)));
            }
        }
        // Add the new segments around this obstruction.
        for (start, end) in self.get_transitions_around(state, obstruction) {
            to_add.push((start, end));
        }

        for update in to_remove {
            self.transitions.remove(&update);
        }
        for update in to_add {
            self.transitions.insert(update.0, update.1);
        }
    }

    /// Mutate state to recreate transition points without this obstruction.
    fn remove_last_obstruction(&mut self) {
        self.transitions = self.prev.clone();
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
        } else if self.obstructions.contains(p) {
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

fn patrol(state: &mut State, nav: &mut Navigator) -> (FxHashSet<(Point2, Direction)>, bool) {
    let mut visited_jumps = FxHashSet::default();

    while let Some((at, face)) = nav.transitions.get(&(state.guard_at, state.guard_face)) {
        if !visited_jumps.insert((state.guard_at, state.guard_face)) {
            return (visited_jumps, true);
        }
        state.guard_at = *at;
        state.guard_face = *face;
    }

    (visited_jumps, false)
}

fn get_looping_obstructions(state: &mut State, nav: &mut Navigator) -> FxHashSet<Point2> {
    let start_at = state.guard_at;
    let start_face = state.guard_face;

    let (visited_jumps, _) = patrol(state, nav);
    let mut visited_tiles = FxHashSet::default();
    for (start, d_enter) in visited_jumps {
        let (end, _) = nav.transitions.get(&(start, d_enter)).unwrap();
        let bbox = BBox2::new(&start, end);
        for x in bbox.min.x..=bbox.max.x {
            for y in bbox.min.y..=bbox.max.y {
                visited_tiles.insert(Point2::new(x, y));
            }
        }
    }
    let mut visited_tiles: Vec<Point2> = visited_tiles.into_iter().collect();
    visited_tiles.sort_by_key(|p| (p.x, p.y));

    let mut looping_obstructions = FxHashSet::default();

    for p in visited_tiles {
        // Never put the obstruction on the original starting position!
        if p == start_at {
            continue;
        }

        state.guard_at = start_at;
        state.guard_face = start_face;

        state.obstructions.insert(p);
        nav.add_obstruction(state, &p);

        let (_, looped) = patrol(state, nav);

        state.guard_at = start_at;
        state.guard_face = start_face;

        state.obstructions.remove(&p);
        nav.remove_last_obstruction();

        if looped {
            looping_obstructions.insert(p);
        }
    }

    looping_obstructions
}

fn solve(parsed: &mut State) -> usize {
    let mut nav = Navigator::new();
    nav.rebuild_all_transitions(parsed);
    get_looping_obstructions(parsed, &mut nav).len()
}

fn main() {
    let input = load_input(2024, 6);

    let mut parsed = parse_input(input);
    let answer = solve(&mut parsed);
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
            BBox2::new(&Point2::new(0, 0), &Point2::new(9, 9)),
            parsed.bbox
        );

        let expected_loops = [
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
        let mut state = parsed.clone();
        let mut nav = Navigator::new();
        nav.rebuild_all_transitions(&state);
        let actual_loops = get_looping_obstructions(&mut state, &mut nav);
        assert_eq!(expected_loops.len(), actual_loops.len());
        assert_eq!(expected_loops, actual_loops);

        let mut state = parsed.clone();
        assert_eq!(6, solve(&mut state));

        assert_eq!(Point2::new(4, 6), parsed.guard_at);
        assert_eq!(Direction::North, parsed.guard_face);
    }
}
