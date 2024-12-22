use aoc_2024_rs::*;
use rustc_hash::{FxHashMap, FxHashSet};

struct State {
    grid: FxHashMap<Point2<i32>, char>,
    bbox: BBox2<i32>,
}

impl State {
    fn new() -> Self {
        Self {
            grid: FxHashMap::default(),
            bbox: BBox2::default(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for (y, row) in input
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
    {
        for (x, col) in row.trim().char_indices() {
            let p = Point2::new(x.try_into().unwrap(), y.try_into().unwrap());
            state.grid.insert(p, col);
            state.bbox.update(&p);
        }
    }

    state
}

fn get_neighbors(bbox: &BBox2<i32>, at: &Point2<i32>) -> Vec<Point2<i32>> {
    let mut neighbors = Vec::new();
    for step in [
        Direction::North.step(at),
        Direction::East.step(at),
        Direction::South.step(at),
        Direction::West.step(at),
    ] {
        if bbox.contains(&step) {
            neighbors.push(step);
        }
    }
    neighbors
}

fn flood_fill(state: &State, start: &Point2<i32>) -> FxHashSet<Point2<i32>> {
    let mut region = FxHashSet::default();

    let color = state.grid.get(start).unwrap();
    let mut queue = Vec::new();
    queue.push(*start);
    while let Some(n) = queue.pop() {
        if region.contains(&n) {
            continue;
        }
        if state.grid.get(&n).unwrap() == color {
            region.insert(n);
            for neighbor in get_neighbors(&state.bbox, &n) {
                queue.push(neighbor);
            }
        }
    }

    region
}

fn get_regions(state: &State) -> Vec<FxHashSet<Point2<i32>>> {
    let mut regions: Vec<FxHashSet<Point2<i32>>> = Vec::new();

    for y in state.bbox.min.y..=state.bbox.max.y {
        for x in state.bbox.min.x..=state.bbox.max.x {
            let at = Point2::new(x, y);
            if regions.iter().any(|region| region.contains(&at)) {
                continue;
            }
            regions.push(flood_fill(state, &at));
        }
    }

    regions
}

fn get_dimensions(bbox: &BBox2<i32>, region: &FxHashSet<Point2<i32>>) -> (usize, usize) {
    let mut perimeter = 0;

    for point in region {
        let neighbors = get_neighbors(bbox, point)
            .iter()
            .filter(|&n| region.contains(n))
            .count();
        perimeter += 4 - neighbors;
    }

    (region.len(), perimeter)
}

fn solve(parsed: &State) -> usize {
    get_regions(parsed)
        .iter()
        .map(|region| {
            let (area, perimeter) = get_dimensions(&parsed.bbox, region);
            area * perimeter
        })
        .sum()
}

fn main() {
    let input = load_input(2024, 12);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12a_example1() {
        let input = "
AAAA
BBCD
BBCC
EEEC
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let region_a = FxHashSet::from_iter(
            [
                Point2::new(0, 0),
                Point2::new(1, 0),
                Point2::new(2, 0),
                Point2::new(3, 0),
            ]
            .into_iter(),
        );
        assert_eq!(region_a, flood_fill(&parsed, &Point2::new(0, 0)));

        let region_b = FxHashSet::from_iter(
            [
                Point2::new(0, 1),
                Point2::new(1, 1),
                Point2::new(0, 2),
                Point2::new(1, 2),
            ]
            .into_iter(),
        );
        assert_eq!(region_b, flood_fill(&parsed, &Point2::new(0, 1)));

        let region_c = FxHashSet::from_iter(
            [
                Point2::new(2, 1),
                Point2::new(2, 2),
                Point2::new(3, 2),
                Point2::new(3, 3),
            ]
            .into_iter(),
        );
        assert_eq!(region_c, flood_fill(&parsed, &Point2::new(2, 1)));

        let region_d = FxHashSet::from_iter([Point2::new(3, 1)].into_iter());
        assert_eq!(region_d, flood_fill(&parsed, &Point2::new(3, 1)));

        let region_e = FxHashSet::from_iter(
            [Point2::new(0, 3), Point2::new(1, 3), Point2::new(2, 3)].into_iter(),
        );
        assert_eq!(region_e, flood_fill(&parsed, &Point2::new(0, 3)));

        let regions = get_regions(&parsed);
        assert_eq!(5, regions.len());

        assert_eq!((4, 10), get_dimensions(&parsed.bbox, &region_a));
        assert_eq!((4, 8), get_dimensions(&parsed.bbox, &region_b));
        assert_eq!((4, 10), get_dimensions(&parsed.bbox, &region_c));
        assert_eq!((1, 4), get_dimensions(&parsed.bbox, &region_d));
        assert_eq!((3, 8), get_dimensions(&parsed.bbox, &region_e));

        assert_eq!(140, solve(&parsed));
    }

    #[test]
    fn day12a_example2() {
        let input = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(772, solve(&parsed));
    }

    #[test]
    fn day12a_example3() {
        let input = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(1930, solve(&parsed));
    }
}
