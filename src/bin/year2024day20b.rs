use aoc_2024_rs::*;
use rustc_hash::{FxHashMap, FxHashSet};

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
    end_at: Point2<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            grid: FxHashMap::default(),
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

fn count_steps(state: &State) -> Vec<(Point2<i32>, i32)> {
    let mut path = Vec::new();
    let mut visited = FxHashSet::default();

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

fn get_neighbors(state: &State, at: &Point2<i32>, radius: i32) -> Vec<Point2<i32>> {
    let mut neighbors = Vec::new();

    for x in (at.x - radius)..=(at.x + radius) {
        for y in (at.y - radius)..=(at.y + radius) {
            let candidate = Point2::new(x, y);
            if at.manhattan_distance(&candidate) > radius {
                continue;
            }
            if !state.bbox.contains(&candidate) {
                continue;
            }
            neighbors.push(candidate);
        }
    }

    neighbors
}

/// Return a map of (seconds-saved, count).
fn find_cheats(state: &State, radius: i32) -> FxHashMap<i32, i32> {
    let start_at = state.start_at;
    let end_at = state.end_at;

    // Similar approach as in day 16. Get cost to each cell on the path both forward and backward,
    // then (start, node) and (node, end) sum to the total cost for any given point on the path.
    // This time, use Manhattan distance to fill in the jump cost.
    let mut state = state.clone();
    let forward_dists = count_steps(&state);
    let forward_dists_lookup: FxHashMap<Point2<i32>, i32> =
        forward_dists.clone().into_iter().collect();
    state.start_at = end_at;
    state.end_at = start_at;
    let backward_dists_lookup: FxHashMap<Point2<i32>, i32> =
        count_steps(&state).into_iter().collect();

    let max_cost: i32 = forward_dists_lookup.len().try_into().unwrap();

    let mut visited = FxHashSet::default();

    let mut saved = FxHashMap::default();
    for (p, forward_cost) in &forward_dists {
        visited.insert(p);
        for n in get_neighbors(&state, p, radius) {
            // Can't end the cheat on a wall.
            if state.grid.get(&n) == Some(&Tile::Wall) {
                continue;
            }
            // Don't want to end up on a previous point on the original path.
            if visited.contains(&n) {
                continue;
            }

            // Looking at (start, end) jumps uniquely, so "the same cheat" definition didn't need
            // to be explicitly checked or de-duped.

            let backward_cost = backward_dists_lookup.get(&n).unwrap();
            let traverse_cost = p.manhattan_distance(&n);
            let cost = forward_cost + backward_cost + traverse_cost;
            // Including start in the direction dists, so subtract one.
            let save = max_cost - 1 - cost;

            if save <= 0 {
                continue;
            }

            saved.entry(save).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    saved
}

fn solve(parsed: &State, save_at_least: i32) -> i32 {
    find_cheats(parsed, 20)
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
    fn day20b_example1() {
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

        // Mixed up what the keys and values were a few times here.
        let count_saved = FxHashMap::from_iter([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ]);
        assert_eq!(
            count_saved,
            find_cheats(&parsed, 20)
                .into_iter()
                .filter(|(k, _)| *k >= 50)
                .collect()
        );

        assert_eq!(0, solve(&parsed, i32::MAX));
        assert_eq!(count_saved.values().sum::<i32>(), solve(&parsed, 50));
    }
}
