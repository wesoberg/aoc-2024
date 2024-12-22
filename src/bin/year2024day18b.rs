use aoc_2024_rs::*;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
struct State {
    // Having two containers that represent the same logical entities and that also both support
    // inclusion checks led to a lot of bugs. Not really happy with this.
    obstacles: Vec<Point2<i32>>,
    grid: FxHashSet<Point2<i32>>,
    bbox: BBox2<i32>,
    start_at: Point2<i32>,
    end_at: Point2<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            obstacles: Vec::new(),
            grid: FxHashSet::default(),
            bbox: BBox2::default(),
            start_at: Point2::min(),
            end_at: Point2::min(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for chunk in input.split_whitespace() {
        match chunk.split(',').collect::<Vec<_>>().as_slice() {
            [a, b] => {
                let p = Point2::new(a.parse().unwrap(), b.parse().unwrap());
                state.obstacles.push(p);
                state.bbox.update(&p);
            }
            _ => panic!("Unparsed chunk: {:?}", chunk),
        }
    }

    state.start_at = state.bbox.min;
    state.end_at = state.bbox.max;

    state
}

#[allow(dead_code)]
fn pprint_grid(state: &State) -> String {
    let mut s = String::new();
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            let c = if state.grid.contains(&p) { '#' } else { '.' };
            s.push(c);
        }
        s.push('\n');
    }
    s.trim().to_string()
}

#[allow(dead_code)]
fn pprint_grid_with_marks(state: &State, marks: &[Point2<i32>]) -> String {
    let mut s = String::new();
    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let p = Point2::new(x, y);
            let c = if marks.contains(&p) {
                'o'
            } else if state.grid.contains(&p) {
                '#'
            } else {
                '.'
            };
            s.push(c);
        }
        s.push('\n');
    }
    s.trim().to_string()
}

fn has_path(state: &State) -> bool {
    let mut visited = FxHashSet::default();
    let mut stack = vec![state.start_at];

    while let Some(p) = stack.pop() {
        if p == state.end_at {
            return true;
        }
        if !visited.insert(p) {
            continue;
        }
        if !state.bbox.contains(&p) || state.grid.contains(&p) {
            continue;
        }
        for d in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            stack.push(d.step(&p));
        }
    }

    false
}

fn tick(state: &mut State, count: usize) {
    for (i, obstacle) in state.obstacles.iter().enumerate() {
        if i == count {
            break;
        }
        if i >= state.grid.len() {
            state.grid.insert(*obstacle);
        }
    }
}

fn solve(parsed: &State, depth: usize) -> String {
    // Binary search for this first condition change.

    let mut left = depth;
    let mut right = parsed.obstacles.len();

    let mut state = parsed.clone();
    while left <= right {
        let m = (left + right) / 2;
        state = parsed.clone();
        tick(&mut state, m);
        if has_path(&state) {
            left = m + 1;
        } else {
            right = m - 1;
        }
    }

    let p = state.obstacles[left - 1];
    format!("{},{}", p.x, p.y)
}

fn main() {
    let input = load_input(2024, 18);

    let parsed = parse_input(input);
    let answer = solve(&parsed, 1024);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18b_example1() {
        let input = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            BBox2::new(&Point2::new(0, 0), &Point2::new(6, 6)),
            parsed.bbox
        );
        assert_eq!(parsed.start_at, parsed.bbox.min);
        assert_eq!(parsed.end_at, parsed.bbox.max);

        assert_eq!(Point2::new(5, 4), parsed.obstacles[0]);
        assert_eq!(
            Point2::new(2, 0),
            parsed.obstacles[parsed.obstacles.len() - 1]
        );

        let mut state = parsed.clone();
        tick(&mut state, 12);
        assert_eq!(
            "
...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....
        "
            .trim()
            .to_string(),
            pprint_grid(&state)
        );

        let mut state = parsed.clone();
        assert_eq!("6,1", solve(&mut state, 12));
    }
}
