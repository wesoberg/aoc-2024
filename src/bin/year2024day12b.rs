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

fn get_neighbors(at: &Point2<i32>) -> Vec<Point2<i32>> {
    vec![
        Direction::North.step(at),
        Direction::East.step(at),
        Direction::South.step(at),
        Direction::West.step(at),
    ]
}

fn get_bounded_neighbors(bbox: &BBox2<i32>, at: &Point2<i32>) -> Vec<Point2<i32>> {
    get_neighbors(at)
        .iter()
        .filter(|n| bbox.contains(n))
        .copied()
        .collect()
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
            for neighbor in get_bounded_neighbors(&state.bbox, &n) {
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

fn get_dimensions(region: &FxHashSet<Point2<i32>>) -> (usize, usize) {
    // Count all continuous segments along boundaries between this region and its non-this-region
    // neighbors in all four directions. For each Y row, check each X column's north and south
    // boundary. For each X column, check each Y row's east and west boundary. If a step from the
    // point in the region would be outside the region, record that coordinate and that direction.
    // The count of continuous series of X (for each invariant Y) (and vise versa) is the desired
    // perimeter.
    //
    // I have to assume there's a much more elegant way to implement this.

    let mut edges_x: FxHashMap<i32, FxHashMap<Direction, Vec<i32>>> = FxHashMap::default();
    let mut edges_y: FxHashMap<i32, FxHashMap<Direction, Vec<i32>>> = FxHashMap::default();
    for point in region {
        for direction in [Direction::North, Direction::South] {
            if region.contains(&direction.step(point)) {
                continue;
            }
            edges_y
                .entry(point.y)
                .and_modify(|ds| {
                    ds.entry(direction)
                        .and_modify(|ps| {
                            ps.push(point.x);
                            ps.sort();
                        })
                        .or_insert(vec![point.x]);
                })
                .or_insert(FxHashMap::from_iter(
                    [(direction, vec![point.x])].into_iter(),
                ));
        }

        for direction in [Direction::East, Direction::West] {
            if region.contains(&direction.step(point)) {
                continue;
            }
            edges_x
                .entry(point.x)
                .and_modify(|ds| {
                    ds.entry(direction)
                        .and_modify(|ps| {
                            ps.push(point.y);
                            ps.sort();
                        })
                        .or_insert(vec![point.y]);
                })
                .or_insert(FxHashMap::from_iter(
                    [(direction, vec![point.y])].into_iter(),
                ));
        }
    }

    let mut perimeter = 0;
    for (_y, ds) in edges_x {
        for (_d, ys) in ds {
            perimeter += 1;
            if ys.len() > 1 {
                for i in 1..ys.len() {
                    if ys[i] - ys[i - 1] > 1 {
                        perimeter += 1;
                    }
                }
            }
        }
    }
    for (_x, ds) in edges_y {
        for (_d, xs) in ds {
            perimeter += 1;
            if xs.len() > 1 {
                for i in 1..xs.len() {
                    if xs[i] - xs[i - 1] > 1 {
                        perimeter += 1;
                    }
                }
            }
        }
    }

    (region.len(), perimeter)
}

fn solve(parsed: &State) -> usize {
    get_regions(parsed)
        .iter()
        .map(|region| {
            let (area, perimeter) = get_dimensions(region);
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
    fn day12b_example1() {
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

        assert_eq!((4, 4), get_dimensions(&region_a));
        assert_eq!((4, 4), get_dimensions(&region_b));
        assert_eq!((4, 8), get_dimensions(&region_c));
        assert_eq!((1, 4), get_dimensions(&region_d));
        assert_eq!((3, 4), get_dimensions(&region_e));

        assert_eq!(80, solve(&parsed));
    }

    #[test]
    fn day12b_example2() {
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

        assert_eq!(436, solve(&parsed));
    }

    #[test]
    fn day12b_example3() {
        let input = "
    EEEEE
    EXXXX
    EEEEE
    EXXXX
    EEEEE
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(236, solve(&parsed));
    }

    #[test]
    fn day12b_example4() {
        let input = "
    AAAAAA
    AAABBA
    AAABBA
    ABBAAA
    ABBAAA
    AAAAAA
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(368, solve(&parsed));
    }

    #[test]
    fn day12b_example5() {
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

        assert_eq!(1206, solve(&parsed));
    }
}
