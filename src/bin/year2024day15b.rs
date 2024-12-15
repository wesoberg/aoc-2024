use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

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

    fn max() -> Self {
        Self::new(i32::MAX, i32::MAX)
    }

    #[allow(dead_code)]
    fn manhattan_distance(&self, other: &Point2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
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

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Open,
    Wall,
    Box,
}

#[derive(Debug, PartialEq, Clone)]
struct State {
    grid: HashMap<Point2, Tile>,
    bbox: BBox2,
    bot: Point2,
    movements: Vec<Direction>,
}

impl State {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            bbox: BBox2::default(),
            bot: Point2::min(),
            movements: Vec::new(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    let grid_chars = HashSet::from(['#', '.', 'O', '@']);
    let dir_chars = HashSet::from(['^', '>', 'v', '<']);

    let mut x = 0;
    let mut y = 0;

    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let line_chars = HashSet::from_iter(line.chars());
        if line_chars.is_subset(&grid_chars) {
            for ch in line.chars() {
                let p = Point2::new(x, y);
                let mut wide = true;
                if let Some(t) = match ch {
                    '#' => Some(Tile::Wall),
                    '.' => Some(Tile::Open),
                    'O' => {
                        wide = false;
                        Some(Tile::Box)
                    }
                    '@' => {
                        wide = false;
                        state.bot = p;
                        Some(Tile::Open)
                    }
                    _ => panic!("Unknown grid char: {:?}", ch),
                } {
                    state.grid.insert(p, t);
                    x += 1;
                    state
                        .grid
                        .insert(Point2::new(x, y), if wide { t } else { Tile::Open });
                }
                state.bbox.update(&p);
                x += 1;
            }
            x = 0;
            y += 1;
        } else if line_chars.is_subset(&dir_chars) {
            for ch in line.chars() {
                let d = match ch {
                    '^' => Direction::North,
                    '>' => Direction::East,
                    'v' => Direction::South,
                    '<' => Direction::West,
                    _ => panic!("Unknown movement char: {:?}", ch),
                };
                state.movements.push(d);
            }
        } else {
            panic!("Unparsed line: {:?}", line);
        }
    }

    state
}

fn is_left_box(state: &State, p: &Point2) -> bool {
    state.grid.get(p) == Some(&Tile::Box)
}

fn is_right_box(state: &State, p: &Point2) -> bool {
    state.grid.get(p) == Some(&Tile::Open) && is_left_box(state, &Direction::West.step(p))
}

#[allow(dead_code)]
fn pprint_grid(state: &State) {
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            let c = if p == state.bot {
                '@'
            } else {
                match state.grid.get(&p) {
                    Some(Tile::Open) => {
                        if is_right_box(state, &p) {
                            ']'
                        } else {
                            '.'
                        }
                    }
                    Some(Tile::Wall) => '#',
                    Some(Tile::Box) => '[',
                    None => unreachable!(),
                }
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
    println!(
        "{}",
        state
            .movements
            .iter()
            .map(|m| match m {
                Direction::North => "N".to_string(),
                Direction::East => "E".to_string(),
                Direction::South => "S".to_string(),
                Direction::West => "W".to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ")
    );
}

fn step_horizontally(state: &State, at: &Point2, d: &Direction) -> Option<Vec<Point2>> {
    let mut group = Vec::new();

    let mut step = d.step(at);
    while let Some(t) = state.grid.get(&step) {
        match t {
            Tile::Wall => {
                return None;
            }
            Tile::Box => {
                group.push(step);
                step = d.step(&step);
            }
            Tile::Open if is_right_box(state, &step) => {
                group.push(step);
                step = d.step(&step);
            }
            Tile::Open => {
                return Some(group);
            }
        }
    }

    Some(group)
}

fn step_vertically(state: &State, at: &Point2, d: &Direction) -> Option<Vec<Point2>> {
    let mut group = Vec::new();

    let step = d.step(at);
    match state.grid.get(&step) {
        Some(Tile::Wall) => {
            return None;
        }
        Some(Tile::Box) => {
            let side = Direction::East.step(&step);
            group.push(step);
            group.push(side);
            let lhs = step_vertically(state, &step, d);
            let rhs = step_vertically(state, &side, d);
            match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => {
                    group.extend(lhs);
                    group.extend(rhs);
                }
                _ => {
                    return None;
                }
            }
        }
        Some(Tile::Open) if is_right_box(state, &step) => {
            let side = Direction::West.step(&step);
            group.push(step);
            group.push(side);
            let lhs = step_vertically(state, &step, d);
            let rhs = step_vertically(state, &side, d);
            match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => {
                    group.extend(lhs);
                    group.extend(rhs);
                }
                _ => {
                    return None;
                }
            }
        }
        Some(Tile::Open) => {
            return Some(group);
        }
        None => unreachable!(),
    }

    Some(group)
}

fn run_bot(state: &mut State) {
    if DEBUG {
        println!("Initial state:");
        pprint_grid(state);
    }

    let movements = state.movements.clone();

    for (i, d) in movements.iter().enumerate() {
        if DEBUG {
            println!("Step {:?} of {:?}", i + 1, movements.len());
            pprint_grid(state);
            pause();

            state.movements.remove(0);
        }

        let group = match d {
            Direction::East | Direction::West => step_horizontally(state, &state.bot, d),
            Direction::North | Direction::South => step_vertically(state, &state.bot, d),
        };

        if let Some(mut group) = group {
            group.sort_by_key(|p| match d {
                Direction::North => (p.y, p.x),
                Direction::South => (-p.y, p.x),
                Direction::East => (-p.x, p.y),
                Direction::West => (p.x, p.y),
            });
            // TODO: Or fix the recursive aggregation.
            group.dedup();

            for p in group {
                let step = d.step(&p);
                // Exchange is always with an open space. This object is moving into an open space,
                // and it leaves an equivalent open space behind it.
                let old = state.grid.insert(p, Tile::Open).unwrap();
                state.grid.insert(step, old);
            }
            state.bot = d.step(&state.bot);
        }
    }

    if DEBUG {
        println!("Final state:");
        pprint_grid(state);
    }
}

//fn score_box(bbox: &BBox2, at: &Point2) -> i32 {
//    // Hours debugging this sort of nonsense because:
//    //
//    // "...distances are measured from the edge of the map
//    // to the closest edge of the box in question..."
//
//    let h0 = bbox.max.y - at.y;
//    let h1 = at.y - bbox.min.y;
//
//    let w0 = bbox.max.x - (at.x + 1);
//    let w1 = at.x - bbox.min.x;
//
//    let h = h0.min(h1);
//    let w = w0.min(w1);
//
//    100 * h + w
//}

fn solve(parsed: &State) -> i32 {
    let mut state = parsed.clone();
    run_bot(&mut state);

    let mut accumulator = 0;
    for (p, t) in state.grid {
        if t == Tile::Box {
            accumulator += 100 * p.y + p.x;
        }
    }

    accumulator
}

fn main() {
    let input = load_input(2024, 15);

    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15b_example1() {
        let input = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(7 * 2, 7)),
            parsed.bbox
        );
        assert_eq!(
            String::from("<^^>>>vv<v>>v<<").len(),
            parsed.movements.len()
        );

        assert_eq!(Point2::new(2 * 2, 2), parsed.bot);
        assert_eq!(
            Some(Tile::Open),
            parsed.grid.get(&Point2::new(2 * 2, 2)).cloned()
        );

        assert_eq!(
            Some(Tile::Open),
            parsed.grid.get(&Point2::new(1 * 2, 1)).cloned()
        );
        assert_eq!(
            Some(Tile::Box),
            parsed.grid.get(&Point2::new(4 * 2, 5)).cloned()
        );
        assert_eq!(
            Some(Tile::Open),
            parsed.grid.get(&Point2::new(5 * 2, 6)).cloned()
        );
        assert_eq!(
            Some(Tile::Wall),
            parsed.grid.get(&Point2::new(6 * 2, 7)).cloned()
        );
    }

    #[test]
    fn day15b_example2() {
        let input = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(18, 9)),
            parsed.bbox
        );

        // ####################
        // ##[].......[].[][]##
        // ##[]...........[].##
        // ##[]........[][][]##
        // ##[]......[]....[]##
        // ##..##......[]....##
        // ##..[]............##
        // ##..@......[].[][]##
        // ##......[][]..[]..##
        // ####################

        // Losing my mind? Any spotchecked scores look correct.
        // But the sum is half what it should be?
        // ...
        // Sigh, it's the original scoring function. So much lying this year.

        //assert_eq!(102, score_box(&parsed.bbox, &Point2::new(2, 1)));
        //assert_eq!(202, score_box(&parsed.bbox, &Point2::new(2, 2)));
        //assert_eq!(302, score_box(&parsed.bbox, &Point2::new(2, 3)));
        //assert_eq!(402, score_box(&parsed.bbox, &Point2::new(2, 4)));
        //assert_eq!(304, score_box(&parsed.bbox, &Point2::new(4, 6)));
        //assert_eq!(104, score_box(&parsed.bbox, &Point2::new(13, 8)));
        //assert_eq!(102, score_box(&parsed.bbox, &Point2::new(15, 1)));

        assert_eq!(9021, solve(&parsed));
    }

    #[test]
    fn day15_example3() {
        let input = "
#########
#..O@...#
#.......#
#########

<v
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(105, solve(&parsed));
    }

    #[test]
    fn day15_example4() {
        let input = "
    #######
    #...#.#
    #.....#
    #..OO@#
    #..O..#
    #.....#
    #######

    <vv<<^^<<^^
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        // Score taken from:
        // https://www.reddit.com/r/adventofcode/comments/1heoj7f/comment/m25w22f/
        assert_eq!(618, solve(&parsed));
    }
}
