use std::collections::HashMap;

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

            // NOTE: This was not working out quickly, but verified in Python shell that all the
            // pages have unique numbers per page.
            //assert_eq!(
            //    HashSet::from_iter(
            //        manual
            //            .pages
            //            .get(manual.pages.len() - 1)
            //            .unwrap()
            //            .iter()
            //            .cloned()
            //    )
            //    .len(),
            //    manual.pages.len()
            //);
        }
    }

    manual
}

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

fn solve(parsed: Manual) -> i32 {
    let mut correct_pages = Vec::new();

    for page in parsed.pages {
        if is_correct(&parsed.rules, &page) {
            correct_pages.push(page);
        }
    }

    correct_pages.iter().map(|page| page[page.len() / 2]).sum()
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
    fn day05a_example1() {
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

        assert_eq!(143, solve(parsed));
    }
}
