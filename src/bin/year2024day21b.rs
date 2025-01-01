use std::collections::HashMap;

use aoc_2024_rs::*;
use cached::proc_macro::cached;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Button {
    Digit(i32),
    Direction(Direction),
    Activate,
}

impl From<char> for Button {
    fn from(value: char) -> Self {
        match value {
            _ if value.is_ascii_digit() => {
                Button::Digit(value.to_digit(10).unwrap().try_into().unwrap())
            }
            'A' => Button::Activate,
            '^' => Button::Direction(Direction::North),
            '>' => Button::Direction(Direction::East),
            'v' => Button::Direction(Direction::South),
            '<' => Button::Direction(Direction::West),
            _ => panic!("Unknown Button char: {:?}", value),
        }
    }
}

fn parse_input(input: String) -> Vec<Vec<Button>> {
    input
        .split_whitespace()
        .map(|chunk| chunk.trim().chars().map(Button::from).collect())
        .collect()
}

// These two keypad() definitions below are considered different types with lazy_static.
// Also don't get code formatting inside the lazy_static block?

/// ```
/// +---+---+---+
/// | 7 | 8 | 9 |
/// +---+---+---+
/// | 4 | 5 | 6 |
/// +---+---+---+
/// | 1 | 2 | 3 |
/// +---+---+---+
///     | 0 | A |
///     +---+---+
/// ```
fn numeric_keypad() -> HashMap<Point2<i32>, Button> {
    HashMap::from([
        (Point2::new(2, 0), Button::Digit(9)),
        (Point2::new(1, 0), Button::Digit(8)),
        (Point2::new(0, 0), Button::Digit(7)),
        (Point2::new(2, 1), Button::Digit(6)),
        (Point2::new(1, 1), Button::Digit(5)),
        (Point2::new(0, 1), Button::Digit(4)),
        (Point2::new(2, 2), Button::Digit(3)),
        (Point2::new(1, 2), Button::Digit(2)),
        (Point2::new(0, 2), Button::Digit(1)),
        (Point2::new(2, 3), Button::Activate),
        (Point2::new(1, 3), Button::Digit(0)),
    ])
}

/// ```
///     +---+---+
///     | ^ | A |
/// +---+---+---+
/// | < | v | > |
/// +---+---+---+
/// ```
fn directional_keypad() -> HashMap<Point2<i32>, Button> {
    HashMap::from([
        (Point2::new(2, 0), Button::Activate),
        (Point2::new(1, 0), Button::Direction(Direction::North)),
        (Point2::new(2, 1), Button::Direction(Direction::East)),
        (Point2::new(1, 1), Button::Direction(Direction::South)),
        (Point2::new(0, 1), Button::Direction(Direction::West)),
    ])
}

fn get_position(keypad: &HashMap<Point2<i32>, Button>, button: &Button) -> Point2<i32> {
    for (p, b) in keypad {
        if button == b {
            return *p;
        }
    }
    panic!("Button not found on given keypad.");
}

type Point2AndDirection = (Point2<i32>, Direction);

fn get_paths(
    keypad: &HashMap<Point2<i32>, Button>,
    start: &Button,
    end: &Button,
) -> Vec<Vec<Point2AndDirection>> {
    fn inner(
        keypad: &HashMap<Point2<i32>, Button>,
        start: &Point2<i32>,
        end: &Point2<i32>,
        lineage: &[Point2AndDirection],
    ) -> Vec<Vec<Point2AndDirection>> {
        let mut paths = Vec::new();

        if start == end {
            if lineage.is_empty() {
                return vec![];
            }
            return vec![lineage.to_vec()];
        }

        for d in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            let p = d.step(start);
            // Every step should move closer to the goal.
            if p.manhattan_distance(end) != start.manhattan_distance(end) - 1 {
                continue;
            }
            // Can't walk off the grid-with-holes.
            if !keypad.contains_key(&p) {
                continue;
            }
            let mut lineage = lineage.to_owned();
            lineage.push((p, d));
            for path in inner(keypad, &p, end, &lineage) {
                paths.push(path);
            }
        }

        paths
    }

    inner(
        keypad,
        &get_position(keypad, start),
        &get_position(keypad, end),
        &[],
    )
}

// <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
// v<<A>>^A<A>AvA<^AA>A<vAAA>^A
// <A^A>^^AvvvA
// 029A
//
// 0(9):                  0           2                   9                 A
// 1(>):          <       A       ^   A     >        ^^   A        vvv      A
// 2(>):   v <<   A >>  ^ A   <   A > A  v  A   <  ^ AA > A   < v  AAA >  ^ A
// 3(>): <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A

#[cached(
    key = "String",
    convert = r#"{ format!("{:?} {:?}", buttons, depth) }"#
)]
fn get_min_seq(keypad: &HashMap<Point2<i32>, Button>, buttons: &[Button], depth: usize) -> usize {
    if depth == 0 {
        return buttons.len();
    }

    let mut seq = buttons.to_owned();
    seq.insert(0, Button::Activate);

    let mut size = 0;

    let mut pairs = seq.windows(2);
    while let Some([start, end]) = pairs.next() {
        size += if start == end {
            1
        } else {
            get_paths(keypad, start, end)
                .iter()
                .map(|path| {
                    get_min_seq(
                        &directional_keypad(),
                        &path
                            .iter()
                            .map(|(_, d)| Button::Direction(*d))
                            .chain(vec![Button::Activate])
                            .collect::<Vec<Button>>(),
                        depth - 1,
                    )
                })
                .min()
                .unwrap()
        };
    }

    size
}

fn solve(parsed: &Vec<Vec<Button>>, depth: usize) -> usize {
    let mut accumulator = 0;

    for buttons in parsed {
        let numeric_part: usize = buttons
            .iter()
            .filter_map(|b| match b {
                Button::Digit(d) => Some(d),
                _ => None,
            })
            .fold(0, |acc, d| acc * 10 + d)
            .try_into()
            .unwrap();
        let min_size = get_min_seq(&numeric_keypad(), buttons, depth);
        accumulator += min_size * numeric_part;
    }

    accumulator
}

fn main() {
    let input = load_input(2024, 21);
    let parsed = parse_input(input);
    let answer = solve(&parsed, 26);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day21a_parse() {
        let input = "
029A
980A
179A
456A
379A
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(5, parsed.len());
        assert_eq!(
            vec![
                Button::Digit(0),
                Button::Digit(2),
                Button::Digit(9),
                Button::Activate
            ],
            parsed[0]
        );
        assert_eq!(
            vec![
                Button::Digit(9),
                Button::Digit(8),
                Button::Digit(0),
                Button::Activate
            ],
            parsed[1]
        );
        assert_eq!(
            vec![
                Button::Digit(1),
                Button::Digit(7),
                Button::Digit(9),
                Button::Activate
            ],
            parsed[2]
        );
        assert_eq!(
            vec![
                Button::Digit(4),
                Button::Digit(5),
                Button::Digit(6),
                Button::Activate
            ],
            parsed[3]
        );

        assert_eq!(
            vec![
                Button::Digit(3),
                Button::Digit(7),
                Button::Digit(9),
                Button::Activate
            ],
            parsed[4]
        );

        // +---+---+---+
        // | 7 | 8 | 9 |
        // +---+---+---+
        // | 4 | 5 | 6 |
        // +---+---+---+
        // | 1 | 2 | 3 |
        // +---+---+---+
        //     | 0 | A |
        //     +---+---+

        assert_eq!(
            9,
            get_paths(&numeric_keypad(), &Button::Activate, &Button::Digit(7)).len()
        );

        assert_eq!(
            2,
            get_paths(&numeric_keypad(), &Button::Activate, &Button::Digit(2)).len()
        );

        assert_eq!(
            1,
            get_paths(&numeric_keypad(), &Button::Activate, &Button::Digit(9)).len()
        );

        assert_eq!(
            0,
            get_paths(&numeric_keypad(), &Button::Activate, &Button::Activate).len()
        );
    }

    #[test]
    fn day21a_example1() {
        let input = "
029A
980A
179A
456A
379A
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        // <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
        // v<<A>>^A<A>AvA<^AA>A<vAAA>^A
        // <A^A>^^AvvvA
        // 029A

        assert_eq!(
            "<A^A>^^AvvvA".len(),
            get_min_seq(&numeric_keypad(), &parsed[0], 1)
        );
        assert_eq!(
            "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len(),
            get_min_seq(&numeric_keypad(), &parsed[0], 2)
        );
        assert_eq!(
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len(),
            get_min_seq(&numeric_keypad(), &parsed[0], 3)
        );

        // 68 * 29, 60 * 980, 68 * 179, 64 * 456, and 64 * 379

        assert_eq!(68, get_min_seq(&numeric_keypad(), &parsed[0], 3));
        assert_eq!(60, get_min_seq(&numeric_keypad(), &parsed[1], 3));
        assert_eq!(68, get_min_seq(&numeric_keypad(), &parsed[2], 3));
        assert_eq!(64, get_min_seq(&numeric_keypad(), &parsed[3], 3));
        assert_eq!(64, get_min_seq(&numeric_keypad(), &parsed[4], 3));

        assert_eq!(126384, solve(&parsed, 3));
    }
}
