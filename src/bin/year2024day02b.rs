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

fn is_safe(report: &Vec<usize>, tolerance: usize) -> bool {
    let mut inc = false;
    let mut dec = false;

    // 0 1 2 3 4 5
    // i j T       # process pair, incr both
    //   i j j     # incr j again, process pair, incr both
    //     i i j   # incr i again, conditionally incr j, process pair
    //         i j # process pair, incr both

    let mut i = 0;
    let mut j = 1;
    loop {
        if i == tolerance {
            i += 1;
            if i == j {
                j += 1;
            }
        } else if j == tolerance {
            j += 1;
        }

        if j >= report.len() {
            break;
        }

        let a = report[i];
        let b = report[j];

        if a < b {
            if dec {
                return false;
            }
            inc = true;
        }
        if a > b {
            if inc {
                return false;
            }
            dec = true;
        }
        let d = a.abs_diff(b);
        if d < 1 || d > 3 {
            return false;
        }

        i += 1;
        j += 1;
    }

    inc ^ dec
}

fn is_safe_with_toleration(report: &Vec<usize>) -> bool {
    if is_safe(&report, usize::MAX) {
        return true;
    }
    for i in 0..report.len() {
        if is_safe(&report, i) {
            return true;
        }
    }
    return false;
}

fn solve(parsed: Vec<Vec<usize>>) -> usize {
    let mut accumulator = 0;
    for report in parsed {
        if is_safe_with_toleration(&report) {
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
    fn day02b_example1() {
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

        let expected_safe = vec![true, false, false, true, true, true];
        assert_eq!(expected_parse.len(), expected_safe.len());
        for (parse, safe) in expected_parse.iter().zip(expected_safe) {
            assert_eq!(safe, is_safe_with_toleration(parse));
        }

        assert_eq!(4, solve(parsed));
    }
}
