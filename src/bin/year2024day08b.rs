use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

struct State {
    towers: HashMap<char, Vec<Point2<i32>>>,
    bbox: BBox2<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            towers: HashMap::new(),
            bbox: BBox2::default(),
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
            state.bbox.update(&p);
            match col {
                '.' => {
                    continue;
                }
                ch => {
                    state
                        .towers
                        .entry(ch)
                        .and_modify(|locs| locs.push(p))
                        .or_insert(vec![p]);
                }
            }
        }
    }

    state
}

fn get_antinodes(towers: &[Point2<i32>], bbox: &BBox2<i32>) -> Vec<Point2<i32>> {
    let mut antinodes = Vec::new();

    // .a.
    // ..a
    // rise=1, run=1
    //
    // ..a
    // .a.
    // rise=1, run=-1

    for i in 0..towers.len() {
        for j in (i + 1)..towers.len() {
            let a = towers[i];
            let b = towers[j];

            let rise = b.y - a.y;
            let run = b.x - a.x;

            let mut updated;
            for step in 1.. {
                updated = false;
                let ca = Point2::new(a.x - run * step, a.y - rise * step);
                if bbox.contains(&ca) {
                    antinodes.push(ca);
                    updated = true;
                }
                let cb = Point2::new(b.x + run * step, b.y + rise * step);
                if bbox.contains(&cb) {
                    antinodes.push(cb);
                    updated = true;
                }
                if !updated {
                    break;
                }
            }
        }
    }

    antinodes
}

fn solve(parsed: State) -> usize {
    let mut antinodes = HashSet::new();
    for (_, towers) in parsed.towers {
        for antinode in get_antinodes(&towers, &parsed.bbox) {
            antinodes.insert(antinode);
        }
        if towers.len() > 1 {
            for tower in towers {
                antinodes.insert(tower);
            }
        }
    }

    antinodes.len()
}

fn main() {
    let input = load_input(2024, 8);
    let parsed = parse_input(input);
    let answer = solve(parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08b_example1() {
        let input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(11, 11)),
            parsed.bbox
        );

        assert_eq!(
            &vec![
                Point2::new(8, 1),
                Point2::new(5, 2),
                Point2::new(7, 3),
                Point2::new(4, 4),
            ],
            parsed.towers.get(&'0').unwrap()
        );

        assert_eq!(
            &vec![Point2::new(6, 5), Point2::new(8, 8), Point2::new(9, 9)],
            parsed.towers.get(&'A').unwrap()
        );

        assert_eq!(34, solve(parsed));
    }

    #[test]
    fn day08b_example2() {
        let input = "
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
        "
        .trim()
        .to_string();
        let mut parsed = parse_input(input);

        parsed.towers.remove(&'#');

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(9, 9)),
            parsed.bbox
        );

        assert_eq!(9, solve(parsed));
    }
}
