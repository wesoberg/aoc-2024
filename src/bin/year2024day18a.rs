use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_2024_rs::*;

#[derive(Debug, Clone)]
struct State {
    obstacles: Vec<Point2<i32>>,
    grid: HashSet<Point2<i32>>,
    bbox: BBox2<i32>,
    start_at: Point2<i32>,
    end_at: Point2<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            obstacles: Vec::new(),
            grid: HashSet::new(),
            bbox: BBox2::default(),
            start_at: Point2::min(),
            end_at: Point2::min(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for chunk in input.split_whitespace() {
        match chunk.split(',').collect::<Vec<_>>().as_slice() {
            [a, b] => {
                let p = Point2::new(a.parse().unwrap(), b.parse().unwrap());
                state.obstacles.push(p);
                state.bbox.update(&p);
            }
            _ => panic!("Unparsed chunk: {:?}", chunk),
        }
    }

    state.start_at = state.bbox.min;
    state.end_at = state.bbox.max;

    state
}

#[allow(dead_code)]
fn pprint_grid(state: &State) -> String {
    let mut s = String::new();
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            let c = if state.grid.contains(&p) { '#' } else { '.' };
            s.push(c);
        }
        s.push('\n');
    }
    s.trim().to_string()
}

// Of course it is: https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    cost: i32,
    position: Point2<i32>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(state: &State) -> Option<i32> {
    let mut dist: HashMap<Point2<i32>, i32> = HashMap::new();

    let mut heap = BinaryHeap::new();

    dist.insert(state.start_at, 0);
    heap.push(Node {
        cost: 0,
        position: state.start_at,
    });

    while let Some(Node { cost, position }) = heap.pop() {
        if position == state.end_at {
            return Some(cost);
        }

        if cost > *dist.get(&position).unwrap_or(&i32::MAX) {
            continue;
        }

        for d in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let next = Node {
                cost: cost + 1,
                position: d.step(&position),
            };
            if !state.bbox.contains(&next.position) || state.grid.contains(&next.position) {
                continue;
            }

            if next.cost < *dist.get(&next.position).unwrap_or(&i32::MAX) {
                heap.push(next);
                dist.insert(next.position, next.cost);
            }
        }
    }

    None
}

fn tick(state: &mut State, count: usize) {
    // This is so goofy when consumers just need a slice. But this occassional (probably not really
    // hotspot) duplication of work does make downstream inclusion checks a lot faster.
    for (i, obstacle) in state.obstacles.iter().enumerate() {
        if i == count {
            break;
        }
        state.grid.insert(*obstacle);
    }
}

fn solve(parsed: &State, depth: usize) -> usize {
    let mut state = parsed.clone();
    tick(&mut state, depth);
    shortest_path(&state).unwrap().try_into().unwrap()
}

fn main() {
    let input = load_input(2024, 18);
    let parsed = parse_input(input);
    let answer = solve(&parsed, 1024);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18a_example1() {
        let input = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(6, 6)),
            parsed.bbox
        );
        assert_eq!(parsed.start_at, parsed.bbox.min);
        assert_eq!(parsed.end_at, parsed.bbox.max);

        assert_eq!(Point2::new(5, 4), parsed.obstacles[0]);
        assert_eq!(
            Point2::new(2, 0),
            parsed.obstacles[parsed.obstacles.len() - 1]
        );

        let mut state = parsed.clone();
        tick(&mut state, 12);
        assert_eq!(
            "
...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....
        "
            .trim()
            .to_string(),
            pprint_grid(&state)
        );

        let mut state = parsed.clone();
        assert_eq!(22, solve(&mut state, 12));
    }
}
