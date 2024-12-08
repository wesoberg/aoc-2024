use aoc_2024_rs::*;

#[derive(Debug, PartialEq)]
struct Equation {
    value: u64,
    parts: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator {
    fn apply(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
            Operator::Cat => {
                // Adding these simpler preconditions saved a little bit of time.
                // Another tip I saw on the subreddit.
                if rhs < 10 {
                    lhs * 10 + rhs
                } else if rhs < 100 {
                    lhs * 100 + rhs
                } else if rhs < 1_000 {
                    lhs * 1_000 + rhs
                } else {
                    // I did look up this formula but was aware of it.
                    lhs * (10u64.pow(rhs.ilog10() + 1)) + rhs
                }
            }
        }
    }
}

fn parse_input(input: String) -> Vec<Equation> {
    let mut equations = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        match line.split([':', ' ']).collect::<Vec<&str>>().as_slice() {
            [value, parts @ ..] => {
                equations.push(Equation {
                    value: value.trim().parse().unwrap(),
                    parts: parts
                        .iter()
                        .filter_map(|p| {
                            let candidate = p.trim();
                            if candidate.is_empty() {
                                None
                            } else {
                                Some(candidate.parse().unwrap())
                            }
                        })
                        .collect(),
                });
            }
            _ => panic!("Unparsed input line: {}", line),
        }
    }

    equations
}

fn is_possible(acc: u64, parts: &[u64], target: u64) -> bool {
    (acc == target && parts.is_empty())
        || ((acc <= target && !parts.is_empty())
            && (is_possible(Operator::Add.apply(acc, parts[0]), &parts[1..], target)
                || is_possible(Operator::Mul.apply(acc, parts[0]), &parts[1..], target)
                || is_possible(Operator::Cat.apply(acc, parts[0]), &parts[1..], target)))
}

fn solve(parsed: &[Equation]) -> u64 {
    parsed
        .iter()
        .filter_map(|e| {
            if is_possible(e.parts[0], &e.parts[1..], e.value) {
                Some(e.value)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = load_input(2024, 7);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day07b_example1() {
        let input = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(9, parsed.len());
        assert_eq!(
            Equation {
                value: 190,
                parts: vec![10, 19],
            },
            parsed[0]
        );
        assert_eq!(
            Equation {
                value: 292,
                parts: vec![11, 6, 16, 20],
            },
            parsed[parsed.len() - 1]
        );

        assert_eq!(156, Operator::Cat.apply(15, 6));
        assert_eq!(86, Operator::Cat.apply(8, 6));
        assert_eq!(178, Operator::Cat.apply(17, 8));

        assert_eq!(11387, solve(&parsed));
    }
}
