use aoc_2024_rs::*;

fn parse_input(input: String) -> Vec<Vec<usize>> {
    input
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(report: &[usize]) -> bool {
    let mut inc = false;
    let mut dec = false;
    let mut same = false;
    let mut out_of_range = false;
    for i in 1..report.len() {
        let a = report[i - 1];
        let b = report[i];
        if a < b {
            inc = true;
        }
        if a == b {
            same = true;
        }
        if a > b {
            dec = true;
        }
        let d = a.abs_diff(b);
        if !(1..=3).contains(&d) {
            out_of_range = true;
        }
    }
    (inc ^ dec) && !(same || out_of_range)
}

fn solve(parsed: Vec<Vec<usize>>) -> usize {
    let mut accumulator = 0;
    for report in parsed {
        if is_safe(&report) {
            accumulator += 1;
        }
    }
    accumulator
}

fn main() {
    let input = load_input(2024, 2);
    let parsed = parse_input(input);
    let answer = solve(parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day02a_example1() {
        let input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "
        .to_string();
        let expected_parse = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let parsed = parse_input(input);
        assert_eq!(expected_parse, parsed);

        let expected_safe = vec![true, false, false, false, false, true];
        assert_eq!(expected_parse.len(), expected_safe.len());
        for (parse, safe) in expected_parse.iter().zip(expected_safe) {
            assert_eq!(safe, is_safe(parse));
        }

        assert_eq!(2, solve(parsed));
    }
}
