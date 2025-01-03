use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

struct State {
    grid: HashMap<Point2<i32>, u32>,
    bbox: BBox2<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            grid: HashMap::new(),
            bbox: BBox2::default(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for (y, row) in input
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
    {
        for (x, col) in row.trim().char_indices() {
            match col {
                '.' => {}
                c => {
                    let p = Point2::new(x.try_into().unwrap(), y.try_into().unwrap());
                    state.grid.insert(p, c.to_digit(10).unwrap());
                    state.bbox.update(&p);
                }
            }
        }
    }

    state
}

fn get_neighbors(state: &State, at: &Point2<i32>) -> Vec<Point2<i32>> {
    let v = state.grid.get(at).unwrap();

    let steps = [
        Direction::North.step(at),
        Direction::East.step(at),
        Direction::South.step(at),
        Direction::West.step(at),
    ];

    let mut neighbors = Vec::new();
    for step in steps {
        if let Some(step_v) = state.grid.get(&step) {
            if step_v > v && step_v - v == 1 {
                neighbors.push(step);
            }
        }
    }

    neighbors
}

fn get_trailheads(state: &State) -> HashSet<(Point2<i32>, Point2<i32>)> {
    let mut starts = Vec::new();
    let mut ends = Vec::new();
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            if let Some(v) = state.grid.get(&p) {
                match v {
                    0 => starts.push(p),
                    9 => ends.push(p),
                    _ => {}
                }
            }
        }
    }

    let mut pairs = HashSet::new();
    for start in starts {
        let mut visited = HashSet::new();
        let mut unvisited = vec![start];
        while let Some(at) = unvisited.pop() {
            if visited.contains(&at) {
                continue;
            }
            visited.insert(at);
            for n in get_neighbors(state, &at) {
                if ends.contains(&n) {
                    pairs.insert((start, n));
                } else {
                    unvisited.push(n);
                }
            }
        }
    }

    pairs
}

fn solve(parsed: &State) -> usize {
    let mut scores = HashMap::new();
    for (start, _) in get_trailheads(parsed) {
        scores
            .entry(start)
            .and_modify(|score| *score += 1)
            .or_insert(1);
    }
    scores.values().sum()
}

fn main() {
    let input = load_input(2024, 10);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10a_example1() {
        let input = "
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let trailheads = get_trailheads(&parsed);
        assert_eq!(
            HashSet::from([
                (Point2::new(3, 0), Point2::new(0, 6)),
                (Point2::new(3, 0), Point2::new(6, 6)),
            ]),
            trailheads
        );

        assert_eq!(2, solve(&parsed));
    }

    #[test]
    fn day10a_example2() {
        let input = "
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let trailheads = get_trailheads(&parsed);
        assert_eq!(
            HashSet::from([
                (Point2::new(3, 0), Point2::new(0, 6)),
                (Point2::new(3, 0), Point2::new(5, 1)),
                (Point2::new(3, 0), Point2::new(4, 4)),
                (Point2::new(3, 0), Point2::new(6, 0)),
            ]),
            trailheads
        );

        assert_eq!(4, solve(&parsed));
    }

    #[test]
    fn day10a_example3() {
        let input = "
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let trailheads = get_trailheads(&parsed);
        assert_eq!(
            HashSet::from([
                (Point2::new(1, 0), Point2::new(3, 5)),
                (Point2::new(5, 6), Point2::new(3, 5)),
                (Point2::new(5, 6), Point2::new(4, 0)),
            ]),
            trailheads
        );

        assert_eq!(3, solve(&parsed));
    }

    #[test]
    fn day10a_example4() {
        let input = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let mut scores = HashMap::new();
        for (start, _) in get_trailheads(&parsed) {
            scores
                .entry(start)
                .and_modify(|score| *score += 1)
                .or_insert(1);
        }
        let mut scores = scores.into_values().collect::<Vec<_>>();
        scores.sort();
        assert_eq!(
            // ...they have scores of 5, 6, 5, 3, 1, 3, 5, 3, and 5.
            vec![1, 3, 3, 3, 5, 5, 5, 5, 6],
            scores
        );

        assert_eq!(36, solve(&parsed));
    }
}
