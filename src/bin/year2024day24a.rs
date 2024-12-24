use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2024_rs::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    And,
    Xor,
    Or,
}

impl Op {
    fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            Self::And => lhs && rhs,
            Self::Xor => lhs ^ rhs,
            Self::Or => lhs || rhs,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    values: HashMap<String, bool>,
    outputs: HashMap<String, (Op, String, String)>,
    wires: HashSet<String>,
}

impl State {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            outputs: HashMap::new(),
            wires: HashSet::new(),
        }
    }

    fn can_output(&self, wire: &String) -> Option<(Op, String, String)> {
        let (op, lhs, rhs) = self.outputs.get(wire).unwrap();
        if self.values.contains_key(lhs) && self.values.contains_key(rhs) {
            return Some((*op, lhs.to_string(), rhs.to_string()));
        }
        None
    }
}

fn parse_input(input: String) -> State {
    // All of the time for part 1 today was in bad assumptions about uniqueness of inputs along
    // various dimensions that took a while to debug one at a time.

    let mut state = State::new();

    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.contains(":") {
            let mut chunks = line.split_whitespace();
            let wire = chunks.next().unwrap().trim_matches(':');
            let value = match chunks.next().unwrap() {
                "0" => false,
                "1" => true,
                _ => panic!("Not a bool value."),
            };
            if state.values.insert(wire.to_string(), value).is_some() {
                panic!("Overwrote a value.");
            }
            state.wires.insert(wire.to_string());
        } else if line.contains("->") {
            let mut chunks = line.split_whitespace();
            let lhs = chunks.next().unwrap();
            let op = match chunks.next().unwrap() {
                "AND" => Op::And,
                "XOR" => Op::Xor,
                "OR" => Op::Or,
                other => panic!("Unknown op: {:?}", other),
            };
            let rhs = chunks.next().unwrap();
            chunks.next();
            let assign = chunks.next().unwrap();
            if let Some(prev) = state
                .outputs
                .insert(assign.to_string(), (op, lhs.to_string(), rhs.to_string()))
            {
                panic!("Overwrote output formula for {:?}: {:?}", assign, prev);
            }
            state.wires.insert(lhs.to_string());
            state.wires.insert(rhs.to_string());
            state.wires.insert(assign.to_string());
        } else {
            panic!("Unparsed line: {:?}", line);
        }
    }

    state
}

fn simulate(state: &State) -> State {
    let mut state = state.clone();

    let mut queue = VecDeque::new();
    let filled: HashSet<String> = state.values.keys().cloned().collect();
    let mut empty: Vec<&String> = state.wires.difference(&filled).collect();
    empty.sort();
    queue.extend(empty);

    while let Some(target_wire) = queue.pop_front() {
        if let Some((op, lhs, rhs)) = state.can_output(target_wire) {
            let r = state.values.insert(
                target_wire.to_string(),
                op.apply(
                    *state.values.get(&lhs).unwrap(),
                    *state.values.get(&rhs).unwrap(),
                ),
            );
            if r.is_some() {
                panic!("Value output for {:?} more than once!", target_wire);
            }
        } else {
            queue.push_back(target_wire);
        }
    }

    state
}

fn solve(parsed: &State) -> u128 {
    let mut z_outs: Vec<String> = parsed
        .wires
        .iter()
        .filter(|wire| wire.starts_with("z"))
        .cloned()
        .collect();
    z_outs.sort();

    let state = simulate(parsed);

    let mut accumulator: u128 = 0;
    for z_out in z_outs.iter().rev() {
        let z_val = state.values.get(z_out).unwrap();
        accumulator = (accumulator << 1) + if *z_val { 1 } else { 0 };
    }

    accumulator
}

fn main() {
    let input = load_input(2024, 24);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24a_example0() {
        assert_eq!(true, Op::And.apply(true, true));
        assert_eq!(false, Op::And.apply(false, true));
        assert_eq!(false, Op::And.apply(true, false));
        assert_eq!(false, Op::And.apply(false, false));

        assert_eq!(false, Op::Xor.apply(true, true));
        assert_eq!(true, Op::Xor.apply(false, true));
        assert_eq!(true, Op::Xor.apply(true, false));
        assert_eq!(false, Op::Xor.apply(false, false));

        assert_eq!(true, Op::Or.apply(true, true));
        assert_eq!(true, Op::Or.apply(false, true));
        assert_eq!(true, Op::Or.apply(true, false));
        assert_eq!(false, Op::Or.apply(false, false));
    }

    #[test]
    fn day24a_example1() {
        let input = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            State {
                values: HashMap::from([
                    ("x00".to_string(), true),
                    ("x01".to_string(), true),
                    ("x02".to_string(), true),
                    ("y00".to_string(), false),
                    ("y01".to_string(), true),
                    ("y02".to_string(), false),
                ]),
                outputs: HashMap::from([
                    (
                        "z00".to_string(),
                        (Op::And, "x00".to_string(), "y00".to_string())
                    ),
                    (
                        "z01".to_string(),
                        (Op::Xor, "x01".to_string(), "y01".to_string())
                    ),
                    (
                        "z02".to_string(),
                        (Op::Or, "x02".to_string(), "y02".to_string())
                    ),
                ]),
                wires: HashSet::from([
                    "x00".to_string(),
                    "x01".to_string(),
                    "x02".to_string(),
                    "y00".to_string(),
                    "y01".to_string(),
                    "y02".to_string(),
                    "z00".to_string(),
                    "z01".to_string(),
                    "z02".to_string(),
                ]),
            },
            parsed
        );

        assert_eq!(6, parsed.values.len());
        assert_eq!(3, parsed.outputs.len());
        assert_eq!(9, parsed.wires.len());

        assert_eq!(4, solve(&parsed));
    }

    #[test]
    fn day24a_example2() {
        let input = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        // Ah, there we go. How dastardly!
        // tgd XOR rvg -> z01
        // tgd XOR rvg -> z12
        //
        // Also have this in the input:
        // cbj AND fnf -> pnj
        // cbj XOR fnf -> z26

        assert_eq!(10, parsed.values.len());
        assert_eq!(36, parsed.outputs.len());
        assert_eq!(46, parsed.wires.len());

        let state = simulate(&parsed);

        assert_eq!(state.values.len(), state.wires.len());
        assert_eq!(46, state.values.len());

        assert_eq!(
            HashMap::from([
                // from initial values
                ("x00".to_string(), true),
                ("x01".to_string(), false),
                ("x02".to_string(), true),
                ("x03".to_string(), true),
                ("x04".to_string(), false),
                ("y00".to_string(), true),
                ("y01".to_string(), true),
                ("y02".to_string(), true),
                ("y03".to_string(), true),
                ("y04".to_string(), true),
                // calculated values after
                ("bfw".to_string(), true),
                ("bqk".to_string(), true),
                ("djm".to_string(), true),
                ("ffh".to_string(), false),
                ("fgs".to_string(), true),
                ("frj".to_string(), true),
                ("fst".to_string(), true),
                ("gnj".to_string(), true),
                ("hwm".to_string(), true),
                ("kjc".to_string(), false),
                ("kpj".to_string(), true),
                ("kwq".to_string(), false),
                ("mjb".to_string(), true),
                ("nrd".to_string(), true),
                ("ntg".to_string(), false),
                ("pbm".to_string(), true),
                ("psh".to_string(), true),
                ("qhw".to_string(), true),
                ("rvg".to_string(), false),
                ("tgd".to_string(), false),
                ("tnw".to_string(), true),
                ("vdt".to_string(), true),
                ("wpb".to_string(), false),
                ("z00".to_string(), false),
                ("z01".to_string(), false),
                ("z02".to_string(), false),
                ("z03".to_string(), true),
                ("z04".to_string(), false),
                ("z05".to_string(), true),
                ("z06".to_string(), true),
                ("z07".to_string(), true),
                ("z08".to_string(), true),
                ("z09".to_string(), true),
                ("z10".to_string(), true),
                ("z11".to_string(), false),
                ("z12".to_string(), false),
            ]),
            state.values
        );

        assert_eq!(2024, solve(&parsed));
    }
}
