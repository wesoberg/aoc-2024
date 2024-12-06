use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

struct Manual {
    rules: Vec<(i32, i32)>,
    pages: Vec<Vec<i32>>,
}

impl Manual {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            pages: Vec::new(),
        }
    }
}

fn parse_input(input: String) -> Manual {
    let mut manual = Manual::new();

    for line in input.lines() {
        if line.contains("|") {
            let chunks: Vec<&str> = line.trim().split("|").collect();
            assert!(chunks.len() == 2);
            manual.rules.push((
                chunks.get(0).unwrap().parse().unwrap(),
                chunks.get(1).unwrap().parse().unwrap(),
            ));
        } else if line.contains(',') {
            let chunks: Vec<&str> = line.trim().split(",").collect();
            assert!(chunks.len() > 1);
            manual.pages.push(
                chunks
                    .iter()
                    .map(|i| i.parse().unwrap())
                    .collect::<Vec<i32>>(),
            );

            assert_eq!(
                manual.pages[manual.pages.len() - 1]
                    .iter()
                    .cloned()
                    .collect::<HashSet<i32>>()
                    .len(),
                manual.pages[manual.pages.len() - 1].len()
            );
        }
    }

    manual
}

// could try something like this:
// given the entries in the page,
// what rules apply?
// then build a tree of the rules that apply
// and topo sort them
// it's a correct page if it matches the sort,
// otherwise the sorted traversal (?) is the corrected version
//
// probably then want to assert that the set of page entries is the set of rules elements
// on initial parse above.

fn is_correct(rules: &Vec<(i32, i32)>, page: &Vec<i32>) -> bool {
    let mut indices = HashMap::new();
    for (i, c) in page.iter().enumerate() {
        indices.insert(c, i);
    }

    for (a, b) in rules {
        match (indices.get(a), indices.get(b)) {
            (Some(ia), Some(ib)) => {
                if ia > ib {
                    return false;
                }
            }
            _ => {}
        }
    }
    return true;
}

fn re_order(rules: &Vec<(i32, i32)>, page: &Vec<i32>) -> Vec<i32> {
    let mut ordered = page.clone();
    while !is_correct(rules, &ordered) {
        for (a, b) in rules {
            // NOTE: It's faster to seek every time than to mutate the indices HashMap with
            // indices.remove() and indices.insert() again.
            match (
                ordered.iter().position(|v| v == a),
                ordered.iter().position(|v| v == b),
            ) {
                (Some(ia), Some(ib)) => {
                    // NOTE: Forgot this if-condition and debugged for a few minutes because the
                    // loop went infinite.
                    if ia > ib {
                        ordered.swap(ia, ib);
                    }
                }
                _ => {}
            }
        }
    }

    ordered
}

fn solve(parsed: Manual) -> i32 {
    let mut corrected_pages = Vec::new();

    for page in parsed.pages {
        if is_correct(&parsed.rules, &page) {
            continue;
        }
        corrected_pages.push(re_order(&parsed.rules, &page));
    }

    corrected_pages
        .iter()
        .map(|page| page[page.len() / 2])
        .sum()
}

fn main() {
    let input = load_input(2024, 5);
    let parsed = parse_input(input);
    let answer = solve(parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05b_example1() {
        let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
        "
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(&(47, 53), parsed.rules.get(0).unwrap());
        assert_eq!(&(97, 13), parsed.rules.get(1).unwrap());
        assert_eq!(&(97, 61), parsed.rules.get(2).unwrap());
        assert_eq!(&(75, 13), parsed.rules.get(parsed.rules.len() - 2).unwrap());
        assert_eq!(&(53, 13), parsed.rules.get(parsed.rules.len() - 1).unwrap());

        assert_eq!(&vec![75, 47, 61, 53, 29], parsed.pages.get(0).unwrap());
        assert_eq!(
            &vec![97, 13, 75, 29, 47],
            parsed.pages.get(parsed.pages.len() - 1).unwrap()
        );

        let expected_correct = vec![true, true, true, false, false, false];
        for (page, expect) in parsed.pages.iter().zip(expected_correct) {
            assert_eq!(expect, is_correct(&parsed.rules, page));
        }

        assert_eq!(
            vec![97, 75, 47, 61, 53],
            re_order(&parsed.rules, &parsed.pages[3])
        );
        assert_eq!(vec![61, 29, 13], re_order(&parsed.rules, &parsed.pages[4]));
        assert_eq!(
            vec![97, 75, 47, 29, 13],
            re_order(&parsed.rules, &parsed.pages[5])
        );

        assert_eq!(123, solve(parsed));
    }
}
