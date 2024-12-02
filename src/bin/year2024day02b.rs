use aoc_2024_rs::*;

fn parse_input(input: String) -> Vec<Vec<usize>> {
    let mut lines = Vec::new();
    for line in input.trim().lines() {
        if line.trim().is_empty() {
            continue;
        }
        lines.push(
            line.split_whitespace()
                .map(|i| i.parse().unwrap())
                .collect(),
        );
    }
    lines
}

fn is_safe(report: &Vec<usize>) -> bool {
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
        if d < 1 || d > 3 {
            out_of_range = true;
        }
    }
    //println!(
    //    "{:?} inc={}, dec={}, same={}, out_of_range={}",
    //    report, inc, dec, same, out_of_range
    //);
    (inc ^ dec) && !(same || out_of_range)
}

fn is_safe_with_toleration(report: Vec<usize>) -> bool {
    if is_safe(&report) {
        return true;
    }
    let mut candidate = Vec::new();
    for i in 0..report.len() {
        candidate.clear();
        for j in 0..report.len() {
            if i != j {
                candidate.push(report[j]);
            }
        }
        if is_safe(&candidate) {
            return true;
        }
    }
    return false;
}

fn solve(parsed: Vec<Vec<usize>>) -> usize {
    let mut accumulator = 0;
    for report in parsed {
        if is_safe_with_toleration(report) {
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
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        let parsed = parse_input(input);
        assert_eq!(expected, parsed);

        assert_eq!(4, solve(parsed));
    }
}
