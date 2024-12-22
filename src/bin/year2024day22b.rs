use aoc_2024_rs::*;
use rustc_hash::{FxHashMap, FxHashSet};

fn parse_input(input: String) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|chunk| chunk.trim().parse().unwrap())
        .collect()
}

fn evolve(value: u64, rounds: u64) -> Vec<u64> {
    let prune = 16777216;

    let mut steps = vec![value];

    let mut v = value;
    for _ in 0..rounds {
        v = ((v * 64) ^ v).rem_euclid(prune);
        v = ((v / 32) ^ v).rem_euclid(prune);
        v = ((v * 2048) ^ v).rem_euclid(prune);
        steps.push(v);
    }

    steps
}

fn solve(parsed: &Vec<u64>, rounds: u64) -> u64 {
    // Implemented this in a new function at first, didn't update the call here, got answer WAY too
    // high (when using the previous part's scoring).

    // Accumulate prices across all runs, keyed by the 4-change sequence. The first time a 4-change
    // is seen in a paritcular evolution, add that resulting price to the accumulator, and then sum
    // with the subsequent prices in later evolutions (but still only the first time that 4-change
    // is seen per evolution). The highest accumulated price at the end is the answer.

    let mut accumulator: FxHashMap<(i64, i64, i64, i64), u64> = FxHashMap::default();
    // Why 21? Not sure what the relationship is here. This accumulator has 40951 entries at the
    // end. The average seen length is 1930 across all rounds. There are 2044 inputs. 40951 / 2044
    // is 20.03... I'm not sure if it really makes any difference to reserve or not here though.
    //
    // Since each 4-change sequence is anchored to an existing index in the changes, and the
    // changes are each rounds - 1 in length, the patholgical case of every 4-change being unique
    // would produce 2044 * (2000 - 1) entries (4M), much higher than the ~41k here. Reserving that
    // much is definitely overkill and slows down the runtime. So all this must just be an artifact
    // of how the inputs were created, and 21 isn't a "valid" coefficient.
    accumulator.reserve(parsed.len() * 21);

    let mut seen = FxHashSet::default();
    seen.reserve(parsed.len());

    for seed in parsed {
        let evolutions = evolve(*seed, rounds);
        let mut changes: Vec<(u64, i64)> = Vec::new();
        for i in 1..evolutions.len() {
            let a = evolutions[i - 1].rem_euclid(10);
            let b = evolutions[i].rem_euclid(10);
            let price = b;
            let change: i64 = i64::try_from(b).unwrap() - i64::try_from(a).unwrap();
            changes.push((price, change));
        }

        seen.clear();
        for i in 3..changes.len() {
            let (_, ac) = changes[i - 3];
            let (_, bc) = changes[i - 2];
            let (_, cc) = changes[i - 1];
            let (dp, dc) = changes[i];
            let seq = (ac, bc, cc, dc);
            // Only want the first time this 4-change is seen in this evolution.
            if !seen.insert(seq) {
                continue;
            }
            accumulator
                .entry(seq)
                .and_modify(|c| *c += dp)
                .or_insert(dp);
        }
    }

    let mut max_price = 0;
    for (_, price) in accumulator {
        if price > max_price {
            max_price = price;
        }
    }

    max_price
}

fn main() {
    let input = load_input(2024, 22);
    let parsed = parse_input(input);
    let answer = solve(&parsed, 2000);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day22b_example1() {
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
            assert_eq!(seq[i], *evolve(seq[i - 1], 1).iter().last().unwrap());
        }

        let seq = vec![
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ];
        for (start, end) in seq {
            assert_eq!(end, *evolve(start, 2000).iter().last().unwrap());
        }

        assert_eq!(2000 + 1, evolve(123, 2000).len());
    }

    #[test]
    fn day22b_example2() {
        let input = "
1
2
3
2024
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(vec![1, 2, 3, 2024], parsed);

        assert_eq!(7 + 7 + 9, solve(&parsed, 2000));
    }
}

