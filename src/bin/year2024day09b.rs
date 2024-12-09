use aoc_2024_rs::*;

#[derive(Debug, PartialEq, Clone)]
enum Block {
    File(u64, u64),
    Free(u64),
}

impl Block {
    fn is_file(&self) -> bool {
        matches!(self, Self::File(_, _))
    }

    fn get_id(&self) -> Option<u64> {
        match self {
            Self::File(id, _) => Some(*id),
            _ => None,
        }
    }
}

fn parse_input(input: String) -> Vec<Block> {
    let mut id = u64::MAX;
    input
        .trim()
        .char_indices()
        .map(|(i, c)| {
            let v = c.to_digit(10).unwrap().into();
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

/// 12345 -> 0..111....22222
fn get_expanded(blocks: &[Block]) -> Vec<Block> {
    blocks
        .iter()
        .flat_map(|b| match b {
            Block::File(i, v) => (0..*v).map(|_| Block::File(*i, *v)).collect::<Vec<Block>>(),
            Block::Free(v) => (0..*v).map(|_| Block::Free(*v)).collect::<Vec<Block>>(),
        })
        .collect()
}

#[allow(dead_code)]
fn get_repr(blocks: &[Block]) -> String {
    blocks
        .iter()
        .map(|b| match b {
            Block::File(i, _) => i.to_string(),
            Block::Free(_) => ".".to_string(),
        })
        .collect()
}

fn get_compacted(blocks: &[Block]) -> Vec<Block> {
    // Gather the ranges of files and free space. At first I tried a bunch of "pointer chasing" but
    // it was difficult to debug.
    let mut file_ranges = Vec::new();
    let mut free_ranges = Vec::new();
    let mut i = 0;
    while i < blocks.len() {
        let j;
        if blocks[i].is_file() {
            j = (i..blocks.len())
                .take_while(|&j| blocks[j].is_file() && blocks[i].get_id() == blocks[j].get_id())
                .last()
                .unwrap();
            file_ranges.push((i, j));
        } else {
            j = (i..blocks.len())
                .take_while(|&j| !blocks[j].is_file())
                .last()
                .unwrap();
            free_ranges.push(Some((i, j)));
        }
        i = j + 1;
    }

    let mut compacted = blocks.to_owned();

    // Walk over the file ranges once in reverse.
    let mut file_range_index = file_ranges.len() - 1;
    loop {
        let (file_start, file_end) = file_ranges[file_range_index];
        let file_size = file_end - file_start + 1;

        // Walk over the free space ranges forwards until either one free space is found that fits
        // the current file (has a fit), or the free space occurs after the file (no fits).
        let mut free_range_index = 0;
        while free_range_index < free_ranges.len() {
            if let Some((free_start, free_end)) = free_ranges[free_range_index] {
                if free_start >= file_end {
                    // This was the last bug, visible by the "12345" example, where files were
                    // moving to the right!
                    break;
                }

                let free_size = free_end - free_start + 1;
                if free_size < file_size {
                    free_range_index += 1;
                    continue;
                }

                for offset in 0..file_size {
                    compacted.swap(free_start + offset, file_start + offset);
                }
                if free_size == file_size {
                    free_ranges[free_range_index] = None;
                } else {
                    free_ranges[free_range_index] = Some((free_start + file_size, free_end));
                }

                break;
            }
            free_range_index += 1;
        }

        file_range_index -= 1;
        if file_range_index == 0 {
            break;
        }
    }

    compacted
}

fn solve(parsed: Vec<Block>) -> u64 {
    let mut accumulator = 0;
    for (i, b) in get_compacted(&get_expanded(&parsed)).iter().enumerate() {
        if let Block::File(id, _) = b {
            let lhs: u64 = i.try_into().unwrap();
            let rhs: u64 = *id;
            accumulator += lhs * rhs;
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
    fn day09b_example1() {
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

        assert_eq!("0..111....22222", get_repr(&get_compacted(&expanded)));
    }

    #[test]
    fn day09b_example2() {
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
            "00992111777.44.333....5555.6666.....8888..",
            get_repr(&get_compacted(&expanded))
        );

        assert_eq!(2858, solve(parsed));
    }
}
