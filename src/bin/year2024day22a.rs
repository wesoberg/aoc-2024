use aoc_2024_rs::*;

fn parse_input(input: String) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|chunk| chunk.trim().parse().unwrap())
        .collect()
}

fn evolve(value: u64, rounds: u64) -> u64 {
    let mut v = value;

    let prune = 16777216;

    for _ in 0..rounds {
        v = ((v * 64) ^ v).rem_euclid(prune);
        v = ((v / 32) ^ v).rem_euclid(prune);
        v = ((v * 2048) ^ v).rem_euclid(prune);
    }

    v
}

fn solve(parsed: &[u64]) -> u64 {
    parsed.iter().map(|start| evolve(*start, 2000)).sum()
}

fn main() {
    let input = load_input(2024, 22);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22a_example1() {
        let input = "
1
10
100
2024
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(vec![1, 10, 100, 2024], parsed);

        let seq = vec![
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];
        for i in 1..seq.len() {
            assert_eq!(seq[i], evolve(seq[i - 1], 1));
        }

        let seq = vec![
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ];
        for (start, end) in seq {
            assert_eq!(end, evolve(start, 2000));
        }

        assert_eq!(37327623, solve(&parsed));
    }
}
