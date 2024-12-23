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

fn get_groups_of_three(connections: &Vec<(String, String)>) -> Vec<HashSet<String>> {
    // Example of direct connections involving "aq":
    //
    // "aq": {"vc", "yn", "cg", "wq"},
    // "cg": {"tb", "yn", "de", "aq"},
    // "yn": {"cg", "wh", "td", "aq"},
    // "wq": {"vc", "tb", "aq", "ub"},
    // "vc": {"ub", "aq", "wq", "tb"},
    //
    // Take aq,vc then find {vc,yn,cq,wq} ∩ {ub,aq,wq,tb} = {wq}, so aq,vc,wq is one 3-group.

    let mut groups = Vec::new();

    // Lots of trying to make the output deterministic because the tests would randomly fail. Turns
    // out, of course, the direct children intersection of two machines may have [0,3] results (at
    // least with the example input), so always calling .next() to get the first element from the
    // (unordered) set was causing the problem.
    //
    // Side note: All this String cloning (I'm guessing) makes this take ~14 seconds. But it could
    // be lots of small set operations, too...

    let direct_connections = get_direct_connections(connections);
    let mut keys: Vec<String> = direct_connections.keys().cloned().collect();
    keys.sort();

    for lparent in &keys {
        let lchildren = direct_connections.get(lparent).unwrap();
        for rparent in keys.iter().filter(|k| lchildren.contains(*k)) {
            let rchildren = direct_connections.get(rparent).unwrap();
            let mut mchildren: Vec<String> = lchildren.intersection(rchildren).cloned().collect();
            mchildren.sort();
            for mchild in mchildren {
                let group =
                    HashSet::from([lparent.to_string(), rparent.to_string(), mchild.to_string()]);
                if !groups.contains(&group) {
                    groups.push(group);
                }
            }
        }
    }

    groups
}

fn solve(parsed: &Vec<(String, String)>) -> usize {
    get_groups_of_three(parsed)
        .iter()
        .filter(|group| group.iter().any(|id| id.starts_with("t")))
        .count()
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
    fn day23a_example1() {
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

        // {
        // 'aq': {'cg', 'vc', 'wq', 'yn'},
        // 'cg': {'aq', 'de', 'tb', 'yn'},
        // 'co': {'de', 'ka', 'ta', 'tc'},
        // 'de': {'cg', 'co', 'ka', 'ta'},
        // 'ka': {'co', 'de', 'ta', 'tb'},
        // 'kh': {'qp', 'ta', 'tc', 'ub'},
        // 'qp': {'kh', 'td', 'ub', 'wh'},
        // 'ta': {'co', 'de', 'ka', 'kh'},
        // 'tb': {'cg', 'ka', 'vc', 'wq'},
        // 'tc': {'co', 'kh', 'td', 'wh'},
        // 'td': {'qp', 'tc', 'wh', 'yn'},
        // 'ub': {'kh', 'qp', 'vc', 'wq'},
        // 'vc': {'aq', 'tb', 'ub', 'wq'},
        // 'wh': {'qp', 'tc', 'td', 'yn'},
        // 'wq': {'aq', 'tb', 'ub', 'vc'},
        // 'yn': {'aq', 'cg', 'td', 'wh'}
        // }

        assert_eq!(32, parsed.len());

        assert_eq!(("kh".to_string(), "tc".to_string()), parsed[0]);
        assert_eq!(
            ("td".to_string(), "yn".to_string()),
            parsed[parsed.len() - 1]
        );

        let expected = vec![
            HashSet::from(["aq".to_string(), "cg".to_string(), "yn".to_string()]),
            HashSet::from(["aq".to_string(), "vc".to_string(), "wq".to_string()]),
            HashSet::from(["co".to_string(), "de".to_string(), "ka".to_string()]),
            HashSet::from(["co".to_string(), "de".to_string(), "ta".to_string()]),
            HashSet::from(["co".to_string(), "ka".to_string(), "ta".to_string()]),
            HashSet::from(["de".to_string(), "ka".to_string(), "ta".to_string()]),
            HashSet::from(["kh".to_string(), "qp".to_string(), "ub".to_string()]),
            HashSet::from(["qp".to_string(), "td".to_string(), "wh".to_string()]),
            HashSet::from(["tb".to_string(), "vc".to_string(), "wq".to_string()]),
            HashSet::from(["tc".to_string(), "td".to_string(), "wh".to_string()]),
            HashSet::from(["td".to_string(), "wh".to_string(), "yn".to_string()]),
            HashSet::from(["ub".to_string(), "vc".to_string(), "wq".to_string()]),
        ];
        let actual = get_groups_of_three(&parsed);
        assert_eq!(expected.len(), actual.len());
        for expected_item in expected {
            assert!(actual.contains(&expected_item));
        }

        let candidate = get_direct_connections(&parsed);
        println!("{:?}", candidate);
        for _ in 0..50 {
            assert_eq!(candidate, get_direct_connections(&parsed));
        }

        let candidate = get_groups_of_three(&parsed);
        for _ in 0..50 {
            assert_eq!(candidate, get_groups_of_three(&parsed));
        }

        assert_eq!(7, solve(&parsed));
    }
}

