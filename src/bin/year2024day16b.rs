use std::collections::BinaryHeap;

use aoc_2024_rs::*;
use rustc_hash::{FxHashMap, FxHashSet};

const DEBUG: bool = false;

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug, Clone)]
struct State {
    grid: FxHashMap<Point2<i32>, Tile>,
    bbox: BBox2<i32>,
    start_at: Point2<i32>,
    start_face: Direction,
    end_at: Point2<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            grid: FxHashMap::default(),
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

// Interestingly, Clippy only complained about "type_complexity" today? Should I feel proud or
// ashamed that earlier days haven't triggered this already?
type LowestCostAndDistHashMap = (i32, FxHashMap<(Point2<i32>, Direction), i32>);

fn shortest_paths(state: &State) -> Option<LowestCostAndDistHashMap> {
    let mut dist: FxHashMap<(Point2<i32>, Direction), i32> = FxHashMap::default();
    dist.reserve((state.bbox.max.x * state.bbox.max.y).try_into().unwrap());

    let mut heap = BinaryHeap::new();

    dist.insert((state.start_at, state.start_face), 0);
    heap.push(Node {
        cost: 0,
        position: state.start_at,
        direction: state.start_face,
    });

    let mut lowest_cost = None;

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
            // Need to hydrate dist costs for all tiles that will appear on a lowest cost path, so
            // keep going until a higher than lowest cost path is found to the end.
            if lowest_cost.is_none() {
                lowest_cost = Some(cost);
            } else if cost > lowest_cost.unwrap() {
                break;
            }
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
            if state.grid.get(&next.position) == Some(&Tile::Wall) {
                continue;
            }

            // We're already on this tile if we've rotated on it. Prune paths that would on the
            // next iteration immediately try to walk into a wall. Mostly because watching this
            // happen was annoying.
            if direction != next.direction
                && state.grid.get(&next.direction.step(&next.position)) == Some(&Tile::Wall)
            {
                continue;
            }

            // Crucially, instead of requiring strictly lower cost, also now accept less than or
            // equal costs. We want all of the tiles on any lowest cost paths to be hydrated in the
            // dist cost data.
            if next.cost
                <= *dist
                    .get(&(next.position, next.direction))
                    .unwrap_or(&i32::MAX)
            {
                heap.push(next);
                dist.insert((next.position, next.direction), next.cost);
            }
        }
    }

    if let Some(cost) = lowest_cost {
        return Some((cost, dist));
    }

    None
}

fn solve(parsed: &State) -> i32 {
    let mut state = parsed.clone();

    let start_at = state.start_at;
    let end_at = state.end_at;

    let (value, forward_dist) = shortest_paths(&state).unwrap();

    // Now it's time to try out this:
    // https://math.stackexchange.com/questions/998848/can-i-use-dijkstras-algorith-for-finding-all-shortest-paths
    //
    // In particular, a node on the lowest cost path should sum (start, node) and (node, end) costs
    // to equal the lowest cost from (start, end) that was actually discovered, exactly. So, run
    // the search once from the front, and up to all directions from the end (whichever directions
    // were seen entering the end with this global lowest cost are now each a reverse start).

    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut backward_dists = vec![];
    for end_d in directions
        .iter()
        .filter(|d| forward_dist.contains_key(&(end_at, **d)))
    {
        state.start_at = end_at;
        state.start_face = end_d.rotate_right().rotate_right();
        state.end_at = start_at;
        let (_, backward_dist) = shortest_paths(&state).unwrap();
        backward_dists.push(backward_dist);
    }

    // Probably don't have to try all points and all directions, but I kind of want to be done.
    // This should be a bunch of quick HashMap checks and some simple math, so it should be fast,
    // but... Well, it's faster with FxHash anyway.

    let mut best_path_points = FxHashSet::default();
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            for d in directions {
                let forward_candidate = (p, d);
                if !forward_dist.contains_key(&forward_candidate) {
                    continue;
                }
                let backward_candidate = (p, d.rotate_right().rotate_right());
                for backward_dist in &backward_dists {
                    if !backward_dist.contains_key(&backward_candidate) {
                        continue;
                    }
                    let f = forward_dist.get(&forward_candidate).unwrap();
                    let b = backward_dist.get(&backward_candidate).unwrap();
                    if f + b == value {
                        best_path_points.insert(p);
                    }
                }
            }
        }
    }

    best_path_points.len().try_into().unwrap()
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
    fn day16b_example1() {
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

        assert_eq!(45, solve(&parsed));
    }

    #[test]
    fn day16b_example2() {
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

        assert_eq!(64, solve(&parsed));
    }
}
