use std::collections::HashMap;

use aoc_2024_rs::*;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
enum Item {
    Mul(usize, usize),
    Do,
    Dont,
}

fn parse_input(input: String) -> Vec<Item> {
    // I thought this was the de facto crate for regex, but it doesn't support exposing the capture
    // indexes, and it doesn't support variadic capture groups (turns out that's only with
    // .extract() in particular though, which to be fair IS called out in the docs I didn't read
    // for that particular method). You can get indexes on matches, but then have to parse again?
    // Weird stuff! Spent an hour tinkering on this nonsense. :D

    let p = [
        r"(mul\([0-9]{1,3},[0-9]{1,3}\))",
        r"(do\(\))",
        r"(don't\(\))",
    ]
    .join("|");
    let re_all = Regex::new(&p).unwrap();
    let re_mul = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut map = HashMap::new();
    for m in re_all.find_iter(&input) {
        match m.as_str() {
            "do()" => {
                map.insert(m.start(), Item::Do);
            }
            "don't()" => {
                map.insert(m.start(), Item::Dont);
            }
            mul => {
                let (_, [a, b]) = re_mul.captures(mul).unwrap().extract();
                map.insert(m.start(), Item::Mul(a.parse().unwrap(), b.parse().unwrap()));
            }
        }
    }

    let mut keys: Vec<&usize> = map.keys().collect();
    keys.sort();

    keys.iter().map(|k| map.get(k).unwrap().clone()).collect()
}

fn solve(parsed: Vec<Item>) -> usize {
    let mut accumulator = 0;
    let mut enabled = true;
    for item in parsed {
        match item {
            Item::Mul(a, b) => {
                if enabled {
                    accumulator += a * b
                }
            }
            Item::Do => enabled = true,
            Item::Dont => enabled = false,
        }
    }
    accumulator
}

fn main() {
    let input = load_input(2024, 3);
    let parsed = parse_input(input);
    let answer = solve(parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03b_example1() {
        let input = "
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
        "
        .to_string();
        let expected_parse = vec![
            Item::Mul(2, 4),
            Item::Dont,
            Item::Mul(5, 5),
            Item::Mul(11, 8),
            Item::Do,
            Item::Mul(8, 5),
        ];
        let parsed = parse_input(input);
        assert_eq!(expected_parse, parsed);

        assert_eq!(48, solve(parsed));
    }
}
