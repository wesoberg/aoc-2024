use std::collections::{BinaryHeap, HashMap};

use aoc_2024_rs::*;

const DEBUG: bool = false;

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

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
}

struct State {
    grid: HashMap<Point2, Tile>,
    bbox: BBox2,
    start_at: Point2,
    start_face: Direction,
    end_at: Point2,
}

impl State {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            bbox: BBox2::default(),
            start_at: Point2::min(),
            start_face: Direction::East,
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
                    state.start_face = Direction::East;
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

fn pprint_grid(state: &State, at: &Point2, face: &Direction) {
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            let c = if p == *at {
                match face {
                    Direction::North => '^',
                    Direction::East => '>',
                    Direction::South => 'v',
                    Direction::West => '<',
                }
            } else if p == state.start_at {
                'S'
            } else if p == state.end_at {
                'E'
            } else {
                match state.grid.get(&p) {
                    Some(t) => match t {
                        Tile::Wall => '#',
                        Tile::Open => '.',
                    },
                    None => unreachable!("Grid is malformed!"),
                }
            };
            print!("{}", c);
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    cost: i32,
    position: Point2,
    direction: Direction,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(state: &State) -> Option<i32> {
    let mut dist: HashMap<(Point2, Direction), i32> = HashMap::new();

    let mut heap = BinaryHeap::new();

    dist.insert((state.start_at, state.start_face), 0);
    heap.push(Node {
        cost: 0,
        position: state.start_at,
        direction: state.start_face,
    });

    while let Some(Node {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if DEBUG {
            pprint_grid(state, &position, &direction);
            println!("heap: {:?}", heap.len());
            pause();
        }

        if position == state.end_at {
            return Some(cost);
        }

        if cost > *dist.get(&(position, direction)).unwrap_or(&i32::MAX) {
            continue;
        }

        let neighbors = [
            // walk forward
            Node {
                cost: cost + 1,
                position: direction.step(&position),
                direction,
            },
            // turn left
            Node {
                cost: cost + 1000,
                position,
                direction: direction.rotate_left(),
            }, //turn right
            Node {
                cost: cost + 1000,
                position,
                direction: direction.rotate_right(),
            },
        ];
        for next in neighbors {
            if DEBUG {
                println!("considering neighbor: {:?}", next);
            }
            // I had put this check inside a DEBUG block, so of course the answer was only correct
            // in DEBUG mode. Oof, that took a while to see.
            if state.grid.get(&next.position) == Some(&Tile::Wall) {
                if DEBUG {
                    println!("next position would be a wall, skipping");
                }
                continue;
            }

            // We're already on this tile if we've rotated on it. Prune paths that would on the
            // next iteration immediately try to walk into a wall.
            if direction != next.direction
                && state.grid.get(&next.direction.step(&next.position)) == Some(&Tile::Wall)
            {
                if DEBUG {
                    println!("next position would be a rotation that steps into a wall, skipping");
                }
                continue;
            }

            if DEBUG {
                println!(
                    "next cost: {:?} (compared to current cost: {:?})",
                    next.cost,
                    dist.get(&(next.position, next.direction))
                        .unwrap_or(&i32::MAX)
                );
            }

            if next.cost
                < *dist
                    .get(&(next.position, next.direction))
                    .unwrap_or(&i32::MAX)
            {
                if DEBUG {
                    println!("found a cheaper cost");
                    println!(
                        "from: {:?}",
                        Node {
                            cost,
                            position,
                            direction
                        }
                    );
                    println!("  to: {:?}", next);
                    println!();
                }
                heap.push(next);
                dist.insert((next.position, next.direction), next.cost);
            }
        }
    }

    None
}

fn solve(parsed: &State) -> i32 {
    shortest_path(parsed).unwrap()
}

fn main() {
    let input = load_input(2024, 16);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16a_example1() {
        let input = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(14, 14)),
            parsed.bbox
        );
        assert_eq!(Point2::new(1, 13), parsed.start_at);
        assert_eq!(Direction::East, parsed.start_face);
        assert_eq!(Some(&Tile::Open), parsed.grid.get(&Point2::new(1, 13)));

        assert_eq!(Point2::new(13, 1), parsed.end_at);
        assert_eq!(Some(&Tile::Open), parsed.grid.get(&Point2::new(13, 1)));

        assert_eq!(Some(&Tile::Wall), parsed.grid.get(&Point2::new(0, 0)));
        assert_eq!(Some(&Tile::Wall), parsed.grid.get(&Point2::new(14, 14)));

        assert_eq!(7036, solve(&parsed));
    }

    #[test]
    fn day16a_example2() {
        let input = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(11048, solve(&parsed));
    }
}

