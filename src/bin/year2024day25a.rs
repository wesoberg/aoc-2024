use aoc_2024_rs::*;

#[derive(Debug, PartialEq)]
struct State {
    locks: Vec<Vec<u32>>,
    keys: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl State {
    fn new() -> Self {
        Self {
            locks: Vec::new(),
            keys: Vec::new(),
            height: 0,
            width: 0,
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    let mut chunks: Vec<Vec<&str>> = Vec::new();
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            chunks.push(Vec::new());
            continue;
        }
        if chunks.is_empty() {
            chunks.push(Vec::new());
        }
        chunks.iter_mut().last().unwrap().push(line);
    }

    state.height = chunks[0].len() - 2;
    state.width = chunks[0][0].len();

    for chunk in &chunks {
        let mut heights = vec![0; chunk[0].len()];
        for line in chunk {
            for (x, col) in line.chars().enumerate() {
                heights[x] += if col == '#' { 1 } else { 0 };
            }
        }
        for height in &mut heights {
            *height -= 1;
        }
        if chunk[0].chars().all(|c| c == '#') {
            state.locks.push(heights);
        } else {
            state.keys.push(heights);
        }
    }

    state
}

fn can_fit(state: &State, lock: &[u32], key: &[u32]) -> bool {
    for x in 0..state.width {
        if key[x] + lock[x] > state.height.try_into().unwrap() {
            return false;
        }
    }
    true
}

fn solve(parsed: &State) -> usize {
    let mut accumulator = 0;
    for key in &parsed.keys {
        for lock in &parsed.locks {
            if can_fit(parsed, lock, key) {
                accumulator += 1;
            }
        }
    }
    accumulator
}

fn main() {
    let input = load_input(2024, 25);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25a_example1() {
        let input = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            State {
                locks: vec![vec![0, 5, 3, 4, 3], vec![1, 2, 0, 5, 3]],
                keys: vec![
                    vec![5, 0, 2, 1, 3],
                    vec![4, 3, 4, 0, 2],
                    vec![3, 0, 2, 0, 1],
                ],
                height: 5,
                width: 5
            },
            parsed
        );

        //Lock 0,5,3,4,3 and key 5,0,2,1,3: overlap in the last column.
        assert_eq!(false, can_fit(&parsed, &parsed.locks[0], &parsed.keys[0]));
        //Lock 0,5,3,4,3 and key 4,3,4,0,2: overlap in the second column.
        assert_eq!(false, can_fit(&parsed, &parsed.locks[0], &parsed.keys[1]));
        //Lock 0,5,3,4,3 and key 3,0,2,0,1: all columns fit!
        assert_eq!(true, can_fit(&parsed, &parsed.locks[0], &parsed.keys[2]));
        //Lock 1,2,0,5,3 and key 5,0,2,1,3: overlap in the first column.
        assert_eq!(false, can_fit(&parsed, &parsed.locks[1], &parsed.keys[0]));
        //Lock 1,2,0,5,3 and key 4,3,4,0,2: all columns fit!
        assert_eq!(true, can_fit(&parsed, &parsed.locks[1], &parsed.keys[1]));
        //Lock 1,2,0,5,3 and key 3,0,2,0,1: all columns fit!
        assert_eq!(true, can_fit(&parsed, &parsed.locks[1], &parsed.keys[2]));

        assert_eq!(3, solve(&parsed));
    }
}
