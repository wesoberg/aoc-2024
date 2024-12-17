use aoc_2024_rs::*;

#[derive(Debug, PartialEq, Clone)]
struct State {
    a: i32,
    b: i32,
    c: i32,
    program: Vec<i32>,
    output: Vec<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            program: Vec::new(),
            output: Vec::new(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["Register", "A:", v] => {
                state.a = v.parse().unwrap();
            }
            ["Register", "B:", v] => {
                state.b = v.parse().unwrap();
            }
            ["Register", "C:", v] => {
                state.c = v.parse().unwrap();
            }
            ["Program:", vs] => {
                state.program = vs.split(',').map(|v| v.parse().unwrap()).collect();
            }
            _ => panic!("Unparsed line: {:?}", line),
        }
    }

    state
}

fn resolve_operand(state: &State, operand: i32, combo: bool) -> i32 {
    // Combo operands 0 through 3 represent literal values 0 through 3.
    // Combo operand 4 represents the value of register A.
    // Combo operand 5 represents the value of register B.
    // Combo operand 6 represents the value of register C.
    // Combo operand 7 is reserved and will not appear in valid programs.

    if !combo {
        return operand;
    }
    match operand {
        0 | 1 | 2 | 3 => operand,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        7 => panic!("Combo operand 7 is reserved!"),
        _ => panic!("Failed to resolve unknown operand: {:?}", operand),
    }
}

fn run(state: &mut State) {
    let mut ip = 0;
    while ip + 1 < state.program.len() {
        let opcode = state.program[ip];
        let operand = state.program[ip + 1];
        match opcode {
            // adv combo
            0 => {
                let r = resolve_operand(state, operand, true);
                state.a = state.a / 2i32.pow(r.try_into().unwrap());
                ip += 2;
            }
            // bxl literal
            1 => {
                let r = resolve_operand(state, operand, false);
                state.b = state.b ^ r;
                ip += 2;
            }
            // bst combo
            2 => {
                let r = resolve_operand(state, operand, true);
                state.b = r.rem_euclid(8);
                ip += 2;
            }
            // jnz literal
            3 => {
                if state.a != 0 {
                    let r = resolve_operand(state, operand, false);
                    ip = r.try_into().unwrap();
                } else {
                    ip += 2;
                }
            }
            // bxc none
            4 => {
                state.b = state.b ^ state.c;
                ip += 2;
            }
            // out combo
            5 => {
                let r = resolve_operand(state, operand, true);
                state.output.push(r.rem_euclid(8));
                ip += 2;
            }
            // bdv combo
            6 => {
                let r = resolve_operand(state, operand, true);
                state.b = state.a / 2i32.pow(r.try_into().unwrap());
                ip += 2;
            }
            // cdv combo
            7 => {
                let r = resolve_operand(state, operand, true);
                state.c = state.a / 2i32.pow(r.try_into().unwrap());
                ip += 2;
            }
            _ => panic!("Unknown opcode: {:?}", opcode),
        }
    }
}

fn solve(parsed: &State) -> String {
    let mut state: State = parsed.clone();
    run(&mut state);
    state
        .output
        .iter()
        .map(|o| o.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let input = load_input(2024, 17);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17a_example1() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let input = "
Register A: 0
Register B: 0
Register C: 9

Program: 2,6
        "
        .trim()
        .to_string();
        let mut state = parse_input(input);
        assert_eq!(
            State {
                a: 0,
                b: 0,
                c: 9,
                program: vec![2, 6],
                output: vec![],
            },
            state
        );
        run(&mut state);
        assert_eq!(1, state.b);

        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let input = "
Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4
        "
        .trim()
        .to_string();
        let mut state = parse_input(input);
        assert_eq!(
            State {
                a: 10,
                b: 0,
                c: 0,
                program: vec![5, 0, 5, 1, 5, 4],
                output: vec![],
            },
            state
        );
        run(&mut state);
        assert_eq!(state.output, vec![0, 1, 2]);

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0
        // and leave 0 in register A.
        let input = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
        "
        .trim()
        .to_string();
        let mut state = parse_input(input);
        run(&mut state);
        assert_eq!(state.a, 0);
        assert_eq!(state.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);

        // If register B contains 29, the program 1,7 would set register B to 26.
        let input = "
Register A: 0
Register B: 29
Register C: 0

Program: 1,7
        "
        .trim()
        .to_string();
        let mut state = parse_input(input);
        run(&mut state);
        assert_eq!(state.b, 26);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set
        // register B to 44354.
        let input = "
Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0
        "
        .trim()
        .to_string();
        let mut state = parse_input(input);
        run(&mut state);
        assert_eq!(state.b, 44354);
    }

    #[test]
    fn day17a_example2() {
        let input = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);
        assert_eq!(
            State {
                a: 729,
                b: 0,
                c: 0,
                program: vec![0, 1, 5, 4, 3, 0],
                output: vec![]
            },
            parsed
        );
        assert_eq!("4,6,3,5,6,3,5,2,1,0", solve(&parsed));
    }
}

