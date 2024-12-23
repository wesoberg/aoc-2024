use std::collections::{HashMap, HashSet};

use aoc_2024_rs::*;

fn parse_input(input: String) -> Vec<(String, String)> {
    input
        .split_whitespace()
        .map(|chunk| {
            let mut parts = chunk.trim().split("-");
            let lhs = parts.next().unwrap().to_string();
            let rhs = parts.next().unwrap().to_string();
            (lhs, rhs)
        })
        .collect()
}

fn get_direct_connections(connections: &Vec<(String, String)>) -> HashMap<String, HashSet<String>> {
    // All the one-hop directly connected machines. Example:
    // "aq": {"vc", "yn", "cg", "wq"},

    let mut groups: HashMap<String, HashSet<String>> = HashMap::new();

    for (a, b) in connections {
        groups
            .entry(a.clone())
            .and_modify(|children| {
                children.insert(b.clone());
            })
            .or_insert(HashSet::from([b.clone()]));
        groups
            .entry(b.clone())
            .and_modify(|children| {
                children.insert(a.clone());
            })
            .or_insert(HashSet::from([a.clone()]));
    }

    groups
}

fn get_largest_local_group(
    parent: &String,
    direct_connections: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    // For some reason I could only think of set operations, then of course
    // HashMap<HashSet<String>, usize> or equivalent doesn't seem to work out of the box.

    let first: HashSet<String> = direct_connections.get(parent).unwrap().clone();
    let mut first_ordered: Vec<String> = first.iter().cloned().collect();
    first_ordered.sort();

    // Get a list of connections that each child shares with this parent. Excluding the two parents
    // themselves, as that produces the wrong result.
    let rest: Vec<HashSet<String>> = first_ordered
        .iter()
        .map(|child| {
            first
                .intersection(direct_connections.get(child).unwrap())
                .cloned()
                .collect()
        })
        .collect();

    // Find the most common subset of connections.
    let mut max_count = 0;
    let mut max_index = 0;
    for i in 0..rest.len() {
        let mut count = 0;
        for j in (i + 1)..rest.len() {
            if rest[i] == rest[j] {
                count += 1;
                if count > max_count {
                    max_count = count;
                    max_index = i;
                }
            }
        }
    }

    // Complete the connectivity group by adding the parents of the chosen connections.
    let mut group = rest[max_index].clone();
    group.insert(parent.to_string());
    group.insert(first_ordered[max_index].to_string());

    group
}

fn get_largest_global_group(connections: &Vec<(String, String)>) -> HashSet<String> {
    let direct_connections = get_direct_connections(connections);
    let mut keys: Vec<String> = direct_connections.keys().cloned().collect();
    keys.sort();

    let mut groups: Vec<HashSet<String>> = keys
        .iter()
        .map(|parent| get_largest_local_group(parent, &direct_connections))
        .collect();

    groups.sort_by_key(|group| group.len());
    groups[groups.len() - 1].clone()
}

fn solve(parsed: &Vec<(String, String)>) -> String {
    let mut group: Vec<String> = get_largest_global_group(parsed).iter().cloned().collect();
    group.sort();
    group.join(",")
}

fn main() {
    let input = load_input(2024, 23);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23b_example1() {
        let input = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
        "
        .trim()
        .to_string();
        let parsed = parse_input(input);

        assert_eq!(32, parsed.len());

        assert_eq!(("kh".to_string(), "tc".to_string()), parsed[0]);
        assert_eq!(
            ("td".to_string(), "yn".to_string()),
            parsed[parsed.len() - 1]
        );

        assert_eq!("co,de,ka,ta".to_string(), solve(&parsed));
    }
}

