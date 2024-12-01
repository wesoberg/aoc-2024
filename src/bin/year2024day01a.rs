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

fn repair_pairs(pairs: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut col0: Vec<usize> = pairs.iter().map(|p| p.0).collect();
    col0.sort();
    let mut col1: Vec<usize> = pairs.iter().map(|p| p.1).collect();
    col1.sort();
    std::iter::zip(col0, col1).collect()
}

fn solve(parsed: Vec<(usize, usize)>) -> usize {
    let mut accumulated_distances = 0;
    for (a, b) in repair_pairs(parsed) {
        accumulated_distances += a.abs_diff(b);
    }
    accumulated_distances
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
    fn day01a_example1() {
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

        assert_eq!(11, solve(parsed));
    }
}
