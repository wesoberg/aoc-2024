use aoc_2024_rs::*;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Machine {
    a: Point2<i32>,
    b: Point2<i32>,
    p: Point2<i32>,
}

impl Machine {
    fn default() -> Self {
        Self {
            a: Point2::min(),
            b: Point2::min(),
            p: Point2::min(),
        }
    }
}

fn parse_input(input: String) -> Vec<Machine> {
    let mut machines = Vec::new();

    let re_a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let re_b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let re_p = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut machine = Machine::default();
    for line in input.trim().lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(caps) = re_a.captures(line) {
            machine.a.x = caps.get(1).unwrap().as_str().parse().unwrap();
            machine.a.y = caps.get(2).unwrap().as_str().parse().unwrap();
        } else if let Some(caps) = re_b.captures(line) {
            machine.b.x = caps.get(1).unwrap().as_str().parse().unwrap();
            machine.b.y = caps.get(2).unwrap().as_str().parse().unwrap();
        } else if let Some(caps) = re_p.captures(line) {
            machine.p.x = caps.get(1).unwrap().as_str().parse().unwrap();
            machine.p.y = caps.get(2).unwrap().as_str().parse().unwrap();
        } else {
            panic!("Unparsed line: {:?}", line);
        }
        if machine.a != Point2::min() && machine.b != Point2::min() && machine.p != Point2::min() {
            machines.push(machine);
            machine = Machine::default();
        }
    }

    machines
}

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
//
// The cheapest way to win the prize is by pushing the A button 80 times and the B button 40 times.
// This would line up the claw along the X axis (because 80*94 + 40*22 = 8400) and along the Y axis
// (because 80*34 + 40*67 = 5400). Doing this would cost 80*3 tokens for the A presses and 40*1 for
// the B presses, a total of 280 tokens.
//
// 80 * 94 + 40 * 22 = 8400
// 80 * 34 + 40 * 67 = 5400
//
// A * 94 + B * 22 = 8400
// A * 34 + B * 67 = 5400
//
// A * a.x + B * b.x = p.x
// A * a.y + B * b.y = p.y
//
// It looks like ChatGPT expanded these correctly below?

fn solve_system(m: &Machine) -> Option<(i32, i32)> {
    // Assuming a system of linear equations with one solution. Maybe it works.

    let b = (m.p.y * m.a.x - m.p.x * m.a.y) / (m.b.y * m.a.x - m.b.x * m.a.y);
    let a = (m.p.x - b * m.b.x) / m.a.x;

    if a * m.a.x + b * m.b.x == m.p.x && a * m.a.y + b * m.b.y == m.p.y {
        Some((a, b))
    } else {
        None
    }
}

fn solve(parsed: &[Machine]) -> i32 {
    parsed
        .iter()
        .map(|m| {
            if let Some((a, b)) = solve_system(m) {
                a * 3 + b
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let input = load_input(2024, 13);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13a_example1() {
        let input = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            vec![
                Machine {
                    a: Point2::new(94, 34),
                    b: Point2::new(22, 67),
                    p: Point2::new(8400, 5400)
                },
                Machine {
                    a: Point2::new(26, 66),
                    b: Point2::new(67, 21),
                    p: Point2::new(12748, 12176)
                },
                Machine {
                    a: Point2::new(17, 86),
                    b: Point2::new(84, 37),
                    p: Point2::new(7870, 6450)
                },
                Machine {
                    a: Point2::new(69, 23),
                    b: Point2::new(27, 71),
                    p: Point2::new(18641, 10279)
                }
            ],
            parsed
        );

        assert_eq!(Some((80, 40)), solve_system(&parsed[0]));
        assert_eq!(None, solve_system(&parsed[1]));
        assert_eq!(Some((38, 86)), solve_system(&parsed[2]));
        assert_eq!(None, solve_system(&parsed[3]));

        assert_eq!(480, solve(&parsed));
    }
}
