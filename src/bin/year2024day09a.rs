use aoc_2024_rs::*;

#[derive(Debug, PartialEq, Clone)]
enum Block {
    File(u64, u64),
    Free(u64),
}

impl Block {
    fn is_file(&self) -> bool {
        match self {
            Self::File(_, _) => true,
            _ => false,
        }
    }
}

fn parse_input(input: String) -> Vec<Block> {
    let mut id = u64::MAX;
    input
        .trim()
        .char_indices()
        .map(|(i, c)| {
            let v = c.to_digit(10).unwrap().try_into().unwrap();
            match i % 2 {
                0 => {
                    if id == u64::MAX {
                        id = 0;
                    } else {
                        id += 1;
                    }
                    Block::File(id, v)
                }
                1 => Block::Free(v),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn get_expanded(blocks: &Vec<Block>) -> Vec<Block> {
    // 12345 -> 0..111....22222

    blocks
        .iter()
        .flat_map(|b| match b {
            Block::File(i, v) => (0..*v).map(|_| Block::File(*i, *v)).collect::<Vec<Block>>(),
            Block::Free(v) => (0..*v).map(|_| Block::Free(*v)).collect::<Vec<Block>>(),
        })
        .collect()
}

#[allow(dead_code)]
fn get_repr(blocks: &Vec<Block>) -> String {
    blocks
        .iter()
        .map(|b| match b {
            Block::File(i, _) => i.to_string(),
            Block::Free(_) => ".".to_string(),
        })
        .collect()
}

fn get_compacted(blocks: &Vec<Block>) -> Vec<Block> {
    let mut compacted = blocks.clone();

    let mut i = 0;
    let mut j = compacted.len() - 1;

    loop {
        while compacted[i].is_file() {
            i += 1;
        }
        while !compacted[j].is_file() {
            j -= 1;
        }
        if i >= j {
            break;
        }
        compacted.swap(i, j);
    }

    compacted
}

fn solve(parsed: Vec<Block>) -> u64 {
    let mut accumulator = 0;
    for (i, b) in get_compacted(&get_expanded(&parsed)).iter().enumerate() {
        match b {
            Block::File(id, _) => {
                let lhs: u64 = i.try_into().unwrap();
                accumulator += lhs * *id;
            }
            _ => {}
        }
    }
    accumulator
}

fn main() {
    let input = load_input(2024, 9);
    let parsed = parse_input(input);
    let answer = solve(parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09a_example1() {
        let input = "
12345
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(
            vec![
                Block::File(0, 1),
                Block::Free(2),
                Block::File(1, 3),
                Block::Free(4),
                Block::File(2, 5),
            ],
            parsed
        );

        let expanded = get_expanded(&parsed);
        assert_eq!(
            vec![
                Block::File(0, 1),
                Block::Free(2),
                Block::Free(2),
                Block::File(1, 3),
                Block::File(1, 3),
                Block::File(1, 3),
                Block::Free(4),
                Block::Free(4),
                Block::Free(4),
                Block::Free(4),
                Block::File(2, 5),
                Block::File(2, 5),
                Block::File(2, 5),
                Block::File(2, 5),
                Block::File(2, 5)
            ],
            expanded
        );
        assert_eq!("0..111....22222", get_repr(&expanded));

        assert_eq!("022111222......", get_repr(&get_compacted(&expanded)));
    }

    #[test]
    fn day09a_example2() {
        let input = "
2333133121414131402
            "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        let expanded = get_expanded(&parsed);
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            get_repr(&expanded)
        );

        assert_eq!(
            "0099811188827773336446555566..............",
            get_repr(&get_compacted(&expanded))
        );

        assert_eq!(1928, solve(parsed));
    }
}
