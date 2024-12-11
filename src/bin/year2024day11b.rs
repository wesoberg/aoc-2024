use std::collections::HashMap;

use aoc_2024_rs::*;

fn parse_input(input: String) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|chunk| chunk.parse().unwrap())
        .collect()
}

fn count_stones(stones: &[u64], depth: u64) -> u64 {
    fn inner(stone: u64, depth: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
        if depth == 0 {
            return 1;
        }

        let key = (stone, depth);
        if cache.contains_key(&key) {
            return *cache.get(&key).unwrap();
        }

        if stone == 0 {
            let v = inner(1, depth - 1, cache);
            cache.insert(key, v);
            return v;
        }

        let digits = stone.ilog10() + 1;
        if digits % 2 == 0 {
            let lhs = stone / 10u64.pow(digits / 2);
            let rhs = stone % 10u64.pow(digits / 2);
            let v = inner(lhs, depth - 1, cache) + inner(rhs, depth - 1, cache);
            cache.insert(key, v);
            return v;
        }

        let v = inner(stone * 2024, depth - 1, cache);
        cache.insert(key, v);
        v
    }

    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&stone| inner(stone, depth, &mut cache))
        .sum()
}

fn solve(parsed: &[u64], depth: u64) -> u64 {
    count_stones(parsed, depth)
}

fn main() {
    let input = load_input(2024, 11);
    let parsed = parse_input(input);
    let answer = solve(&parsed, 75);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11b_example1() {
        let input = "
0 1 10 99 999
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(7, solve(&parsed, 1));
    }

    #[test]
    fn day11b_example2() {
        let input = "
125 17
            "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(22, solve(&parsed, 6));
        assert_eq!(55312, solve(&parsed, 25));
    }
}
