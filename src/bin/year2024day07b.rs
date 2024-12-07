use std::collections::HashMap;

use aoc_2024_rs::*;
use itertools::{repeat_n, Itertools};

const DEBUG: bool = false;

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
            // I did look up this formula but was aware of it.
            Operator::Cat => lhs * (10u64.pow(rhs.ilog10() + 1)) + rhs,
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

struct Validate {
    ops_cache: HashMap<usize, Vec<Vec<Operator>>>,
}

impl Validate {
    fn new() -> Self {
        Self {
            ops_cache: HashMap::new(),
        }
    }

    fn is_possible(&mut self, equation: &Equation) -> bool {
        let ops_size = equation.parts.len() - 1;
        if !self.ops_cache.contains_key(&ops_size) {
            // Almost half the runtime is in this stuff, hence the caching here. Might be faster to
            // implement a non-generic version?

            let permutations = repeat_n(
                vec![Operator::Add, Operator::Mul, Operator::Cat].into_iter(),
                ops_size,
            )
            .multi_cartesian_product()
            .collect_vec();
            self.ops_cache.insert(ops_size, permutations);
        }

        if DEBUG {
            println!("{:?}", equation);
        }

        for ops in self.ops_cache.get(&ops_size).unwrap() {
            let mut lhs = None;
            let mut rhs = None;
            let mut op = None;

            let mut i = 0;
            let mut j = 0;

            if DEBUG {
                println!("  {:?}", ops);
            }

            while i < equation.parts.len() {
                if DEBUG {
                    println!("    {:?} {:?} {:?}", op, lhs, rhs);
                }

                // This was faster than something like "match (op, lhs, rhs)", presumably because
                // that variant had additional loop iterations?

                if lhs.is_none() {
                    lhs = Some(equation.parts[i]);
                    i += 1;
                }
                if rhs.is_none() {
                    rhs = Some(equation.parts[i]);
                    i += 1;
                }
                if op.is_none() {
                    op = Some(ops[j]);
                    j += 1;
                }
                lhs = Some(op.unwrap().apply(lhs.unwrap(), rhs.unwrap()));
                rhs = None;
                op = None;
            }

            if lhs.unwrap() == equation.value {
                return true;
            }
        }

        false
    }
}

fn solve(parsed: &Vec<Equation>) -> u64 {
    let mut validator = Validate::new();
    parsed
        .iter()
        .filter_map(|e| {
            if validator.is_possible(e) {
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
    fn day07a_example1() {
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
