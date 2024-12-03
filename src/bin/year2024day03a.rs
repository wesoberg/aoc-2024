use aoc_2024_rs::*;
use regex::Regex;

fn parse_input(input: String) -> Vec<(usize, usize)> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(&input)
        .map(|caps| {
            let a = caps.get(1).unwrap().as_str().parse().unwrap();
            let b = caps.get(2).unwrap().as_str().parse().unwrap();
            (a, b)
        })
        .collect()
}

fn solve(parsed: Vec<(usize, usize)>) -> usize {
    let mut accumulator = 0;
    for (a, b) in parsed {
        accumulator += a * b;
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
    fn day03a_example1() {
        let input = "
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
        "
        .to_string();
        let expected_parse = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        let parsed = parse_input(input);
        assert_eq!(expected_parse, parsed);

        assert_eq!(161, solve(parsed));
    }
}
