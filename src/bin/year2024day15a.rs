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

#[derive(Debug, PartialEq, Clone)]
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
        let line_chars = HashSet::from_iter(line.chars().into_iter());
        if line_chars.is_subset(&grid_chars) {
            for ch in line.chars() {
                let p = Point2::new(x, y);
                if let Some(t) = match ch {
                    '#' => Some(Tile::Wall),
                    '.' => Some(Tile::Open),
                    'O' => Some(Tile::Box),
                    '@' => {
                        state.bot = p;
                        // Amazing. Took so long to debug why one box refused to move. It was never
                        // in the grid, and I'm materializing the grid, so an empty cell only means
                        // out of bounds. Lost at least an hour to this.

                        // None
                        Some(Tile::Open)
                    }
                    _ => panic!("Unknown grid char: {:?}", ch),
                } {
                    state.grid.insert(p, t);
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

#[allow(dead_code)]
fn pprint_grid(state: &State) {
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            let c = if p == state.bot {
                '@'
            } else {
                match state.grid.get(&p) {
                    Some(Tile::Open) | None => '.',
                    Some(Tile::Wall) => '#',
                    Some(Tile::Box) => 'O',
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

fn find_step(state: &State, at: &Point2, d: &Direction) -> Option<Point2> {
    if DEBUG {
        println!("finding step from {:?} heading {:?}", at, d);

        let mut holes = Vec::new();
        for y in state.bbox.min.y..=state.bbox.max.y {
            for x in state.bbox.min.x..=state.bbox.max.x {
                let p = Point2::new(x, y);
                if !state.grid.contains_key(&p) {
                    holes.push(p);
                }
            }
        }
        println!(
            "Found these holes in the grid ({:?} of them): {:?}",
            holes.len(),
            holes
        );
    }

    let mut step = d.step(at);
    while let Some(t) = state.grid.get(&step) {
        if DEBUG {
            println!("checked {:?} and saw {:?}", step, t);
        }
        match t {
            Tile::Wall => {
                if DEBUG {
                    println!("found at final step {:?} :: {:?}", step, t);
                }
                return None;
            }
            Tile::Box => {
                step = d.step(&step);
                if DEBUG {
                    println!("going to check {:?} next", step);
                }
            }
            Tile::Open => {
                if DEBUG {
                    println!("found at final step {:?} :: {:?}", step, t);
                }
                return Some(step);
            }
        }
    }
    if DEBUG {
        println!("step left the grid: {:?}", step);
    }
    None
}

fn run_bot(state: &mut State) {
    let movements = state.movements.clone();

    for (i, d) in movements.iter().enumerate() {
        if DEBUG {
            println!("Step {:?} of {:?}", i, movements.len());
            pprint_grid(state);
            pause();
        }

        state.movements.remove(0);

        if let Some(end) = find_step(state, &state.bot, d) {
            if DEBUG {
                println!(
                    "found an end step from {:?} ({:?}) to {:?}",
                    state.bot, d, end
                );
            }
            // It's a direct step into an adjacent open tile.
            if state.bot.manhattan_distance(&end) == 1 {
                state.bot = end;
                continue;
            }

            // It's a box push, so tiles get swapped.
            let step = d.step(&state.bot);
            state.grid.insert(end, Tile::Box);
            state.grid.insert(step, Tile::Open);
            state.bot = step;
        } else {
            if DEBUG {
                println!("no end step found, skipping move");
            }
        }
    }
}

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
    fn day15a_example1() {
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
            BBox2::new(&Point2::new(0, 0), &Point2::new(7, 7)),
            parsed.bbox
        );
        assert_eq!(
            String::from("<^^>>>vv<v>>v<<").len(),
            parsed.movements.len()
        );

        assert_eq!(Point2::new(2, 2), parsed.bot);
        assert_eq!(
            Some(Tile::Open),
            parsed.grid.get(&Point2::new(2, 2)).cloned()
        );

        assert_eq!(
            Some(Tile::Open),
            parsed.grid.get(&Point2::new(1, 1)).cloned()
        );
        assert_eq!(
            Some(Tile::Box),
            parsed.grid.get(&Point2::new(4, 5)).cloned()
        );
        assert_eq!(
            Some(Tile::Open),
            parsed.grid.get(&Point2::new(5, 6)).cloned()
        );
        assert_eq!(
            Some(Tile::Wall),
            parsed.grid.get(&Point2::new(6, 7)).cloned()
        );

        assert_eq!(2028, solve(&parsed));
    }

    #[test]
    fn day15a_example2() {
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

        assert_eq!(10092, solve(&parsed));
    }

    #[test]
    fn day15_example3() {
        let input = "
#######
#...O..
#......
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(104, solve(&parsed));
    }
}
