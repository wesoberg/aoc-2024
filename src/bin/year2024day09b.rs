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
    // First build a list for file ranges, and another list for free ranges bucketed by the block
    // size. The idea is to use the file size to index into these free ranges buckets. Instead of
    // walking the entire free ranges list every single file block iteration, we can instead walk a
    // few buckets and check the last element in each bucket to see if indices are good. The free
    // ranges bucket lists will need to be maintained sorted in reverse order, so that last() is
    // always applicable.
    let mut free_buckets = Vec::new();
    let mut file_ranges = Vec::new();
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
            let s = j - i + 1;
            while s >= free_buckets.len() {
                free_buckets.push(Vec::new());
            }
            free_buckets[s].push((i, j));
        }
        i = j + 1;
    }

    for bucket in &mut free_buckets {
        bucket.sort();
        bucket.reverse();
    }

    let mut compacted = blocks.to_owned();

    // Walk over the file ranges once in reverse.
    for (file_start, file_end) in file_ranges.iter().rev() {
        let file_size = file_end - file_start + 1;

        // Interestingly, adding this (obviously logically incorrect, given the loop below) check
        // here makes the tests fail but still gives the same correct answer on the input:
        // || free_buckets[file_size].is_empty()
        // That's so wild!
        if file_size >= free_buckets.len() {
            continue;
        }

        // Walk the free ranges buckets that could fit this file. Look for the lowest index free
        // block in all of those potential buckets.
        let mut best_bucket_index = usize::MAX;
        let mut smallest_index_seen = usize::MAX;
        for (bucket_index, bucket) in free_buckets.iter().enumerate().skip(file_size) {
            if let Some((free_start, _)) = bucket.last() {
                if free_start >= file_start {
                    continue;
                }
                if *free_start < smallest_index_seen {
                    best_bucket_index = bucket_index;
                    smallest_index_seen = *free_start;
                }
            }
        }
        if best_bucket_index == usize::MAX {
            continue;
        }

        let (free_start, free_end) = free_buckets[best_bucket_index].pop().unwrap();
        let free_size = free_end - free_start + 1;

        for offset in 0..file_size {
            compacted.swap(free_start + offset, file_start + offset);
        }

        // If there's leftover free space, re-allocate this remaining free space to the appropriate
        // free ranges bucket.
        if free_size > file_size {
            let new_free_start = free_start + file_size;
            let new_free_size = free_end - new_free_start + 1;
            free_buckets[new_free_size].push((new_free_start, free_end));
            free_buckets[new_free_size].sort();
            free_buckets[new_free_size].reverse();
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
