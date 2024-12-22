use std::collections::{BinaryHeap, HashMap};

use aoc_2024_rs::*;

const DEBUG: bool = false;

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
}

struct State {
    grid: HashMap<Point2<i32>, Tile>,
    bbox: BBox2<i32>,
    start_at: Point2<i32>,
    start_face: Direction,
    end_at: Point2<i32>,
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

fn pprint_grid(state: &State, at: &Point2<i32>, face: &Direction) {
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
    position: Point2<i32>,
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
    let mut dist: HashMap<(Point2<i32>, Direction), i32> = HashMap::new();

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
