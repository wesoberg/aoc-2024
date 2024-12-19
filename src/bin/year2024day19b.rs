use aoc_2024_rs::*;
use cached::proc_macro::cached;
use cached::UnboundCache;

#[derive(Debug, PartialEq, Eq)]
enum Color {
    // MTG Color Wheel?!
    W, // white
    U, // blue
    B, // black
    R, // red
    G, // green
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'w' => Color::W,
            'u' => Color::U,
            'b' => Color::B,
            'r' => Color::R,
            'g' => Color::G,
            _ => panic!("Unknown Color char: {:?}", value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    patterns: Vec<Vec<Color>>,
    designs: Vec<Vec<Color>>,
}

impl State {
    fn new() -> Self {
        Self {
            patterns: Vec::new(),
            designs: Vec::new(),
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
        if line.contains(',') {
            for chunk in line.split(",") {
                state
                    .patterns
                    .push(chunk.trim().chars().map(Color::from).collect());
            }
        } else {
            state.designs.push(line.chars().map(Color::from).collect());
        }
    }

    state
}

#[cached(
    ty = "UnboundCache<String, u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ format!("{:?}", design) }"#
)]
fn is_possible(patterns: &Vec<Vec<Color>>, design: &[Color]) -> u64 {
    let mut acc = 0;
    if design.is_empty() {
        return acc;
    }
    for pattern in patterns {
        if pattern.len() == design.len() && pattern == design {
            // Last bug was still early aborting here with a return.
            acc += 1;
        }
        if pattern.len() > design.len() {
            continue;
        }
        if !design.starts_with(pattern) {
            continue;
        }
        acc += is_possible(patterns, &design[pattern.len()..]);
    }

    acc
}

fn solve(parsed: &State) -> u64 {
    let mut accumulator = 0;
    for design in &parsed.designs {
        accumulator += is_possible(&parsed.patterns, design);
    }
    accumulator
}

fn main() {
    let input = load_input(2024, 19);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19b_example1() {
        let input = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            State {
                patterns: vec![
                    vec![Color::R],
                    vec![Color::W, Color::R],
                    vec![Color::B],
                    vec![Color::G],
                    vec![Color::B, Color::W, Color::U],
                    vec![Color::R, Color::B],
                    vec![Color::G, Color::B],
                    vec![Color::B, Color::R],
                ],
                designs: vec![
                    vec![Color::B, Color::R, Color::W, Color::R, Color::R,],
                    vec![Color::B, Color::G, Color::G, Color::R,],
                    vec![Color::G, Color::B, Color::B, Color::R,],
                    vec![Color::R, Color::R, Color::B, Color::G, Color::B, Color::R,],
                    vec![Color::U, Color::B, Color::W, Color::U,],
                    vec![Color::B, Color::W, Color::U, Color::R, Color::R, Color::G,],
                    vec![Color::B, Color::R, Color::G, Color::R,],
                    vec![Color::B, Color::B, Color::R, Color::G, Color::W, Color::B,],
                ],
            },
            parsed
        );

        // brwrr
        assert_eq!(2, is_possible(&parsed.patterns, &parsed.designs[0]));
        // bggr
        assert_eq!(1, is_possible(&parsed.patterns, &parsed.designs[1]));
        // gbbr
        assert_eq!(4, is_possible(&parsed.patterns, &parsed.designs[2]));
        // rrbgbr
        assert_eq!(6, is_possible(&parsed.patterns, &parsed.designs[3]));
        // ubwu
        assert_eq!(0, is_possible(&parsed.patterns, &parsed.designs[4]));
        // bwurrg
        assert_eq!(1, is_possible(&parsed.patterns, &parsed.designs[5]));
        // brgr
        assert_eq!(2, is_possible(&parsed.patterns, &parsed.designs[6]));
        // bbrgwb
        assert_eq!(0, is_possible(&parsed.patterns, &parsed.designs[7]));

        assert_eq!(16, solve(&parsed));
    }
}
