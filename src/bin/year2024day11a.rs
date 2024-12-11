use aoc_2024_rs::*;

fn parse_input(input: String) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|chunk| chunk.parse().unwrap())
        .collect()
}

fn count_stones(stone: u64, depth: u64) -> u64 {
    if depth == 0 {
        return 1;
    }

    if stone == 0 {
        return count_stones(1, depth - 1);
    }

    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        let lhs = stone / 10u64.pow(digits / 2);
        let rhs = stone % 10u64.pow(digits / 2);
        return count_stones(lhs, depth - 1) + count_stones(rhs, depth - 1);
    }

    count_stones(stone * 2024, depth - 1)
}

fn solve(parsed: &[u64], depth: u64) -> u64 {
    parsed.iter().map(|&p| count_stones(p, depth)).sum()
}

fn main() {
    let input = load_input(2024, 11);
    let parsed = parse_input(input);
    let answer = solve(&parsed, 25);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11a_example1() {
        let input = "
0 1 10 99 999
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(7, solve(&parsed, 1));
    }

    #[test]
    fn day11a_example2() {
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
