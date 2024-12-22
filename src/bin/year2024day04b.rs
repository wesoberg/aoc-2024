use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

fn parse_input(input: String) -> HashMap<Point2<i32>, char> {
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        for (x, col) in line.trim().char_indices() {
            grid.insert(
                Point2::new(x.try_into().unwrap(), y.try_into().unwrap()),
                col,
            );
        }
    }
    grid
}

fn get_bounds(grid: &HashMap<Point2<i32>, char>) -> BBox2<i32> {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for p in grid.keys() {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }

    BBox2::new(&Point2::new(min_x, min_y), &Point2::new(max_x, max_y))
}

#[allow(dead_code)]
fn pprint_grid(grid: &HashMap<Point2<i32>, char>) {
    let bbox = get_bounds(grid);
    for y in bbox.min.y..=bbox.max.y {
        for x in bbox.min.x..=bbox.max.x {
            print!("{}", grid.get(&Point2::new(x, y)).unwrap());
        }
        println!();
    }
}

fn get_word_vectors(grid: &HashMap<Point2<i32>, char>) -> Vec<Vec<(i32, i32)>> {
    let anchor_value = 'A';
    let neighbor_values = HashSet::from([Some(&'M'), Some(&'S')]);

    let forward_neighbor_steps = [(-1, -1), (1, 1)];
    let backward_neighbor_steps = [(1, -1), (-1, 1)];

    let mut hits = Vec::new();

    let bbox = get_bounds(grid);
    for y in bbox.min.y..=bbox.max.y {
        for x in bbox.min.x..=bbox.max.x {
            if *grid.get(&Point2::new(x, y)).unwrap() != anchor_value {
                continue;
            }

            let forward_coords: Vec<(i32, i32)> = forward_neighbor_steps
                .iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .collect();
            let backward_coords: Vec<(i32, i32)> = backward_neighbor_steps
                .iter()
                .map(|(dx, dy)| (x + dx, y + dy))
                .collect();

            let forward_candidate = forward_coords
                .iter()
                .map(|(nx, ny)| grid.get(&Point2::new(*nx, *ny)))
                .collect::<HashSet<Option<&char>>>();
            let backward_candidate = backward_coords
                .iter()
                .map(|(nx, ny)| grid.get(&Point2::new(*nx, *ny)))
                .collect::<HashSet<Option<&char>>>();

            if forward_candidate == neighbor_values && backward_candidate == neighbor_values {
                let mut hit = vec![(x, y)];
                hit.extend(forward_coords);
                hit.extend(backward_coords);
                hits.push(hit);
            }
        }
    }

    hits
}

fn solve(parsed: HashMap<Point2<i32>, char>) -> usize {
    get_word_vectors(&parsed).len()
}

fn main() {
    let input = load_input(2024, 4);
    let parsed = parse_input(input);
    let answer = solve(parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day04b_example1() {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
        "
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(9, solve(parsed));
    }
}
