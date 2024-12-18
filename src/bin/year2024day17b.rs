use aoc_2024_rs::*;

const DEBUG: bool = false;

#[derive(Debug, PartialEq, Clone)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
    output: Vec<u64>,
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

    #[allow(dead_code)]
    fn render_output(&self) -> String {
        self.output
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<_>>()
            .join(",")
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

fn resolve_operand(state: &State, operand: u64, combo: bool) -> u64 {
    // Combo operands 0 through 3 represent literal values 0 through 3.
    // Combo operand 4 represents the value of register A.
    // Combo operand 5 represents the value of register B.
    // Combo operand 6 represents the value of register C.
    // Combo operand 7 is reserved and will not appear in valid programs.

    if !combo {
        return operand;
    }
    match operand {
        0..=3 => operand,
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

        if DEBUG {
            println!();
            println!(
                "Pointer: {:?}, Opcode: {:?}, Operand: {:?}",
                ip, opcode, operand
            );
            println!("{:<20}  {:<20}  {:<30}", "PROGRAM", "OUTPUTS", "REGISTERS");
            println!(
                "{:<20 }  {:<20 }  rA: {:>8 }, rB: {:>8 }, rC: {:>8 }",
                format!("{:?}", state.program),
                format!("{:?}", state.output),
                state.a,
                state.b,
                state.c
            );
        }

        match opcode {
            // adv combo
            0 => {
                let r = resolve_operand(state, operand, true);
                state.a /= 2u64.pow(r.try_into().unwrap());
                if DEBUG {
                    println!(
                        "opcode={:?}, ADV COMBO, r={:?}, a = a / (2 ** r) = {}",
                        opcode, r, state.a
                    );
                }
                ip += 2;
            }
            // bxl literal
            1 => {
                let r = resolve_operand(state, operand, false);
                state.b ^= r;
                if DEBUG {
                    println!(
                        "opcode={:?}, BXL LITERAL, r={:?}, b = b ^ r = {:?}",
                        opcode, r, state.b
                    );
                }
                ip += 2;
            }
            // bst combo
            2 => {
                let r = resolve_operand(state, operand, true);
                state.b = r.rem_euclid(8);
                if DEBUG {
                    println!(
                        "opcode={:?}, BST COMBO, r={:?}, b = r % 8 = {:?}",
                        opcode, r, state.b
                    );
                }
                ip += 2;
            }
            // jnz literal
            3 => {
                if state.a != 0 {
                    let r = resolve_operand(state, operand, false);
                    ip = r.try_into().unwrap();
                    if DEBUG {
                        println!("opcode={:?}, JNZ LITERAL, r={:?}, ip = {:?}", opcode, r, ip);
                    }
                } else {
                    if DEBUG {
                        println!("opcode={:?}, JNZ LITERAL, no-op", opcode);
                    }
                    ip += 2;
                }
            }
            // bxc none
            4 => {
                state.b ^= state.c;
                if DEBUG {
                    println!("opcode={:?}, BXC NONE, b = b ^ c = {:?}", opcode, state.b);
                }
                ip += 2;
            }
            // out combo
            5 => {
                let r = resolve_operand(state, operand, true);
                state.output.push(r.rem_euclid(8));
                if DEBUG {
                    println!(
                        "opcode={:?}, OUT COMBO, r={:?}, out += r % 8 = {:?}",
                        opcode,
                        r,
                        state.output.last().unwrap()
                    );
                }
                ip += 2;
            }
            // bdv combo
            6 => {
                let r = resolve_operand(state, operand, true);
                state.b = state.a / 2u64.pow(r.try_into().unwrap());
                if DEBUG {
                    println!(
                        "opcode={:?}, BVD COMBO, r={:?}, b = a / (2 ** r) = {:?}",
                        opcode, r, state.b
                    );
                }
                ip += 2;
            }
            // cdv combo
            7 => {
                let r = resolve_operand(state, operand, true);
                state.c = state.a / 2u64.pow(r.try_into().unwrap());
                if DEBUG {
                    println!(
                        "opcode={:?}, CDV COMBO, r={:?}, c = a / (2 ** r) = {:?}",
                        opcode, r, state.c
                    );
                }
                ip += 2;
            }
            _ => panic!("Unknown opcode: {:?}", opcode),
        }
    }
}

fn search(state: &State, candidate: u64) -> Option<u64> {
    // Printing out the instructions and what they are doing reveals a loop. Something like:
    //
    // do {
    // b = a % 8
    // b = b ^ 1
    // c = a / ( 2 ** b )
    // a = a / ( 2 ** 3 )
    // b = b ^ 4
    // b = b ^ c
    // out <- b % 8
    // } (while a != 0)
    //
    // Here the 'a' input is first read with modulo 8, meaning its value is only [0, 7], or the
    // last 3 bits. Then 'a' is divided by 2 ** 3 (which is 8), so this is divmod() in octal to
    // walk through the number from right to left, one octal digit at a time.
    //
    // Expanded form of this example which turned out useless aside from quick spot checks:
    // out <- ((((a % 8) ^ 1) ^ 4) ^ (a / (2 ** ((a % 8) ^ 1)))) % 8
    //
    // To solve this, for each [0, 7], see if it outputs the next program digit after running the
    // loop above. Then for each [0, 7] not rejected, repeat. Repeat... This builds up the final
    // 'a' from least to most significant digit.
    //
    // Side node, the bounds for the search space for 16 digit outputs from 'a' in this example:
    // lower =  35_184_372_088_832
    // upper = 281_474_976_710_655

    for next in 0..8 {
        let mut attempt = state.clone();
        attempt.a = (candidate << 3) + next;
        run(&mut attempt);

        if attempt.output.len() > attempt.program.len() {
            continue;
        }
        if attempt.output[0] != attempt.program[attempt.program.len() - attempt.output.len()] {
            continue;
        }

        if attempt.output == attempt.program {
            return Some((candidate << 3) + next);
        }
        if let Some(value) = search(state, (candidate << 3) + next) {
            return Some(value);
        }
    }

    None
}

fn solve(parsed: &State) -> u64 {
    search(parsed, 0).unwrap()
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
    fn day17b_example1() {
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
    fn day17b_example2() {
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
    }

    #[test]
    fn day17b_example3() {
        let input = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);
        assert_eq!(
            State {
                a: 2024,
                b: 0,
                c: 0,
                program: vec![0, 3, 5, 4, 3, 0],
                output: vec![]
            },
            parsed
        );
    }
}
