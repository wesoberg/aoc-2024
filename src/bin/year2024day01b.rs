use std::collections::HashMap;

use aoc_2024_rs::*;

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let mut pairs = Vec::new();
    for line in input.trim().lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut chunks = line.split_whitespace();
        let a = chunks.next().unwrap().parse().unwrap();
        let b = chunks.next().unwrap().parse().unwrap();
        pairs.push((a, b));
    }
    pairs
}

fn count_occurrences(corpus: Vec<usize>) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    for item in corpus {
        counts
            .entry(item)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    counts
}

fn solve(parsed: Vec<(usize, usize)>) -> usize {
    let mut accumulator = 0;
    let col1_counts = count_occurrences(parsed.iter().map(|p| p.1).collect());
    for (col0, _) in parsed {
        accumulator += col0 * col1_counts.get(&col0).unwrap_or(&0);
    }
    accumulator
}

fn main() {
    let input = load_input(2024, 1);
    let parsed = parse_input(input);
    let answer = solve(parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01b_example1() {
        let input = "
3   4
4   3
2   5
1   3
3   9
3   3
        "
        .to_string();
        let expected = vec![(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];
        let parsed = parse_input(input);
        assert_eq!(expected, parsed);

        assert_eq!(31, solve(parsed));
    }
}
