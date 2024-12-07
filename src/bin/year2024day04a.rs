use std::collections::HashMap;

use aoc_2024_rs::*;

fn parse_input(input: String) -> HashMap<(i32, i32), char> {
    let mut grid = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        for (x, col) in line.trim().char_indices() {
            grid.insert((x.try_into().unwrap(), y.try_into().unwrap()), col);
        }
    }
    grid
}

fn get_bounds(grid: &HashMap<(i32, i32), char>) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for (x, y) in grid.keys() {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    (min_x, min_y, max_x, max_y)
}

#[allow(dead_code)]
fn pprint_grid(grid: &HashMap<(i32, i32), char>) {
    let (min_x, min_y, max_x, max_y) = get_bounds(grid);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", grid.get(&(x, y)).unwrap());
        }
        println!();
    }
}

fn get_steps(word_len: i32) -> Vec<Vec<(i32, i32)>> {
    vec![
        // n  : -y
        (0..word_len).map(|i| (0, -i)).collect(),
        // ne : +x, -y
        (0..word_len).map(|i| (i, -i)).collect(),
        // e  : +x
        (0..word_len).map(|i| (i, 0)).collect(),
        // se : +x, +y
        (0..word_len).map(|i| (i, i)).collect(),
        // s  : +y
        (0..word_len).map(|i| (0, i)).collect(),
        // sw : -x, +y
        (0..word_len).map(|i| (-i, i)).collect(),
        // w  : -x
        (0..word_len).map(|i| (-i, 0)).collect(),
        // nw : -x, -y
        (0..word_len).map(|i| (-i, -i)).collect(),
    ]
}

fn get_word_vectors(grid: &HashMap<(i32, i32), char>) -> Vec<Vec<(i32, i32)>> {
    let word = ['X', 'M', 'A', 'S'];
    let steps = get_steps(word.len().try_into().unwrap());

    let mut hits = Vec::new();

    let (min_x, min_y, max_x, max_y) = get_bounds(grid);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            for step in &steps {
                let mut failed = false;
                for (i, (dx, dy)) in step.iter().enumerate() {
                    match grid.get(&(x + dx, y + dy)) {
                        Some(c) => {
                            if c != word.get(i).unwrap() {
                                failed = true;
                                break;
                            }
                        }
                        None => {
                            failed = true;
                            break;
                        }
                    }
                }
                if !failed {
                    hits.push(step.to_vec());
                }
            }
        }
    }

    hits
}

fn solve(parsed: HashMap<(i32, i32), char>) -> usize {
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
    fn day04a_example1() {
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

        assert_eq!(18, solve(parsed));
    }
}
