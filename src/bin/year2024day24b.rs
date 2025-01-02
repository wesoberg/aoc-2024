use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    str::FromStr,
};

use aoc_2024_rs::*;

const DEBUG: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    And,
    Xor,
    Or,
}

impl Op {
    fn render(&self) -> String {
        match self {
            Self::And => "&".to_string(),
            Self::Xor => "^".to_string(),
            Self::Or => "|".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Label {
    A(usize),
    X(usize),
    Y(usize),
    Z(usize),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseLabelError;

impl FromStr for Label {
    type Err = ParseLabelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("x") {
            Ok(Label::X(rest.chars().fold(0, |acc, ch| {
                let digit: usize = ch.to_digit(10).unwrap().try_into().unwrap();
                acc * 10 + digit
            })))
        } else if let Some(rest) = s.strip_prefix("y") {
            Ok(Label::Y(rest.chars().fold(0, |acc, ch| {
                let digit: usize = ch.to_digit(10).unwrap().try_into().unwrap();
                acc * 10 + digit
            })))
        } else if let Some(rest) = s.strip_prefix("z") {
            Ok(Label::Z(rest.chars().fold(0, |acc, ch| {
                let digit: usize = ch.to_digit(10).unwrap().try_into().unwrap();
                acc * 10 + digit
            })))
        } else {
            Ok(Label::A(s.chars().fold(0, |acc, ch| {
                let ascii: usize = (ch as u8).into();
                acc * 1000 + ascii
            })))
        }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X(vs) => {
                let mut s = String::new();
                let mut v = *vs;
                while v != 0 {
                    let digit = v.rem_euclid(10);
                    s.push_str(&digit.to_string());
                    v /= 10;
                }
                s.push('x');
                write!(f, "{}", s.chars().rev().collect::<String>())
            }
            Self::Y(vs) => {
                let mut s = String::new();
                let mut v = *vs;
                while v != 0 {
                    let digit = v.rem_euclid(10);
                    s.push_str(&digit.to_string());
                    v /= 10;
                }
                s.push('y');
                write!(f, "{}", s.chars().rev().collect::<String>())
            }
            Self::Z(vs) => {
                let mut s = String::new();
                let mut v = *vs;
                while v != 0 {
                    let digit = v.rem_euclid(10);
                    s.push_str(&digit.to_string());
                    v /= 10;
                }
                s.push('z');
                write!(f, "{}", s.chars().rev().collect::<String>())
            }
            Self::A(vs) => {
                let mut s = String::new();
                let mut v = *vs;
                while v != 0 {
                    let digit: u8 = v.rem_euclid(1000).try_into().unwrap();
                    s.push(char::from(digit));
                    v /= 1000;
                }
                write!(f, "{}", s.chars().rev().collect::<String>())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    values: HashMap<Label, bool>,
    outputs: HashMap<Label, (Op, Label, Label)>,
    wires: HashSet<Label>,
}

impl State {
    fn new() -> Self {
        Self {
            values: HashMap::new(),
            outputs: HashMap::new(),
            wires: HashSet::new(),
        }
    }
}

fn parse_input(input: String) -> State {
    let mut state = State::new();

    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.contains(":") {
            let mut chunks = line.split_whitespace();
            let wire = chunks.next().unwrap().trim_matches(':').parse().unwrap();
            let value = match chunks.next().unwrap() {
                "0" => false,
                "1" => true,
                _ => panic!("Not a bool value."),
            };
            if state.values.insert(wire, value).is_some() {
                panic!("Overwrote a value.");
            }
            state.wires.insert(wire);
        } else if line.contains("->") {
            let mut chunks = line.split_whitespace();
            let lhs = chunks.next().unwrap().parse().unwrap();
            let op = match chunks.next().unwrap() {
                "AND" => Op::And,
                "XOR" => Op::Xor,
                "OR" => Op::Or,
                other => panic!("Unknown op: {:?}", other),
            };
            let rhs = chunks.next().unwrap().parse().unwrap();
            chunks.next();
            let assign = chunks.next().unwrap().parse().unwrap();
            if let Some(prev) = state.outputs.insert(assign, (op, lhs, rhs)) {
                panic!("Overwrote output formula for {:?}: {:?}", assign, prev);
            }
            state.wires.insert(lhs);
            state.wires.insert(rhs);
            state.wires.insert(assign);
        } else {
            panic!("Unparsed line: {:?}", line);
        }
    }

    // Not the case in the example inputs, but the real input is a very specific kind of machine.
    // So validate that all the input parts of each gate are unique, so there's no ambiguity. Only
    // the output wires need to be swapped around, not any of the input wires (this is also in the
    // problem description).
    assert_eq!(
        state.outputs.len(),
        state
            .outputs
            .values()
            .cloned()
            .collect::<HashSet<(Op, Label, Label)>>()
            .len()
    );

    state
}

#[derive(Debug, Clone)]
enum Child {
    Leaf(Label),
    Tree(Tree),
}

impl Child {
    #[allow(dead_code)]
    fn render(&self) -> String {
        match self {
            Self::Leaf(leaf) => leaf.to_string(),
            Self::Tree(tree) => format!(
                "({} {} {})",
                (*tree.lhs).render(),
                tree.op.render(),
                (*tree.rhs).render()
            ),
        }
    }

    fn get_label(&self) -> Label {
        match self {
            Self::Leaf(leaf) => *leaf,
            Self::Tree(tree) => tree.out,
        }
    }

    fn flatten(&self) -> Vec<Child> {
        let mut flat = vec![self.clone()];
        match self {
            Self::Leaf(leaf) => {
                flat.push(Self::Leaf(*leaf));
            }
            Self::Tree(tree) => {
                flat.extend(tree.lhs.flatten());
                flat.extend(tree.rhs.flatten());
            }
        }
        flat
    }
}

#[derive(Debug, Clone)]
struct Tree {
    out: Label,
    op: Op,
    lhs: Box<Child>,
    rhs: Box<Child>,
}

impl Tree {
    fn new(out: Label, op: Op, lhs: Child, rhs: Child) -> Self {
        // First time actually using any Box<>, Rc<>, whatevers, I think.
        Self {
            out,
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    #[allow(dead_code)]
    fn render(self) -> String {
        Child::Tree(self).render()
    }
}

// Printing the normalized formulas for each z-out shows:
//
// z00 <- (y00 ^ x00)
// z01 <- ((y00 & x00) ^ (x01 ^ y01))
// z02 <- ((((y00 & x00) & (x01 ^ y01)) | (x01 & y01)) ^ (y02 ^ x02))
// z03 <- ((((((y00 & x00) & (x01 ^ y01)) | (x01 & y01)) & (y02 ^ x02)) | (x02 & y02)) ^ (y03 ^ x03))
// z04 <- ((((((((y00 & x00) & (x01 ^ y01)) | (x01 & y01)) & (y02 ^ x02)) | (x02 & y02)) & (y03 ^ x03)) | (y03 & x03)) ^ (x04 ^ y04))
// z05 <- ((((((((((y00 & x00) & (x01 ^ y01)) | (x01 & y01)) & (y02 ^ x02)) | (x02 & y02)) & (y03 ^ x03)) | (y03 & x03)) & (x04 ^ y04)) | (x04 & y04)) ^ (y05 ^ x05))
//
// Rules:
//
// * z-out must be a XOR
// * next term children (XOR'd):
//   * previous term with XOR -> AND
//   * XOR of zN's x,y (ex: z11 => (x11 ^ y11))
// * starting at 3rd level: (z02):
//   * next term children (XOR'd):
//     * previous term with XOR -> AND
//       * with OR (x{N-1} & y{N-1})
//     * XOR of zN's x,y (ex: z11 => (x11 ^ y11))
//
// Patterns:
//
// z00 <- (y00 ^ x00)
// z01 <- ((z00{^->&}) ^ (x01 ^ y01))
// z02 <- (((z01{^->&}) | (x01 & y01)) ^ (y02 ^ x02))
// z03 <- (((z02{^->&}) | (x02 & y02)) ^ (y03 ^ x03))
// z04 <- (((z03{^->&}) | (y03 & x03)) ^ (x04 ^ y04))
// z05 <- (((z04{^->&}) | (x04 & y04)) ^ (y05 ^ x05))
//
// zN <- (((z{N-1}{^->&}) | (x{N-1} & y{N-1})) ^ (y{N} ^ x{N}))
// when N >= 2
//
// Since these are all boolean operations, the order doesn't matter. This makes it much easier to
// print out the above and spot the patterns. Normalized form rules:
//
// * prefer (xN, yN) over (yN, xN)
// * prefer larger sub-tree size on the left child

fn get_correct_tree(z: usize) -> Tree {
    if z == 0 {
        // z00 <- (y00 ^ x00)
        Tree::new(
            Label::Z(z),
            Op::Xor,
            Child::Leaf(Label::X(0)),
            Child::Leaf(Label::Y(0)),
        )
    } else if z == 1 {
        // z01 <- ((z00{^->&}) ^ (x01 ^ y01))
        let mut lhs = get_correct_tree(z - 1);
        lhs.op = Op::And;
        Tree::new(
            Label::Z(z),
            Op::Xor,
            Child::Tree(lhs),
            Child::Tree(Tree::new(
                Label::A(0),
                Op::Xor,
                Child::Leaf(Label::X(1)),
                Child::Leaf(Label::Y(1)),
            )),
        )
    } else {
        // zN <- (((z{N-1}{^->&}) | (x{N-1} & y{N-1})) ^ (y{N} ^ x{N}))
        let mut lhs = get_correct_tree(z - 1);
        lhs.op = Op::And;
        let lhs = Tree::new(
            Label::A(0),
            Op::Or,
            Child::Tree(lhs),
            Child::Tree(Tree::new(
                Label::A(0),
                Op::And,
                Child::Leaf(Label::X(z - 1)),
                Child::Leaf(Label::Y(z - 1)),
            )),
        );
        Tree::new(
            Label::Z(z),
            Op::Xor,
            Child::Tree(lhs),
            Child::Tree(Tree::new(
                Label::A(0),
                Op::Xor,
                Child::Leaf(Label::X(z)),
                Child::Leaf(Label::Y(z)),
            )),
        )
    }
}

fn get_labeled_correct_tree(state: &State, tree: Tree) -> Tree {
    fn inner(state: &State, node: Child) -> Child {
        match node {
            Child::Leaf(leaf) => Child::Leaf(leaf),
            Child::Tree(tree) => {
                let this = tree.clone();

                // Hydrate children first, as labels are built up from the leaf nodes.
                let lhs = inner(state, *this.lhs);
                let rhs = inner(state, *this.rhs);

                let mut out = this.out;

                for (c_out, (c_op, c_lhs, c_rhs)) in &state.outputs {
                    // This will initially only match gates with x, y, or z labels, as those are
                    // the only labels populated by building the correct tree, and they're all
                    // input labels, so by definition they're correct. But the order of the
                    // two children may be arbitrary in the input.
                    if this.op == *c_op
                        && ((lhs.get_label() == *c_lhs && rhs.get_label() == *c_rhs)
                            || (lhs.get_label() == *c_rhs && rhs.get_label() == *c_lhs))
                    {
                        out = *c_out;
                        break;
                    }
                }

                Child::Tree(Tree::new(out, tree.op, lhs, rhs))
            }
        }
    }

    match inner(state, Child::Tree(tree)) {
        Child::Tree(tree) => tree,
        Child::Leaf(_) => unreachable!("Did not build a tree from Tree parent."),
    }
}

fn get_tree_size(state: &State, parent: &Label) -> usize {
    let mut depth = 0;

    if let Some((_, lhs, rhs)) = state.outputs.get(parent) {
        depth += 1;
        depth += get_tree_size(state, lhs);
        depth += get_tree_size(state, rhs);
    }

    depth
}

fn get_tree_from_gates(state: &State, parent: &Label) -> Tree {
    fn inner(state: &State, parent: &Label) -> Child {
        if let Some((op, lhs, rhs)) = state.outputs.get(parent) {
            let (mut lhs, mut rhs) = (lhs, rhs);
            if matches!(lhs, Label::Y(_)) && matches!(rhs, Label::X(_)) {
                (lhs, rhs) = (rhs, lhs);
            }
            if get_tree_size(state, lhs) < get_tree_size(state, rhs) {
                (lhs, rhs) = (rhs, lhs);
            }
            Child::Tree(Tree {
                out: *parent,
                op: *op,
                lhs: Box::new(inner(state, lhs)),
                rhs: Box::new(inner(state, rhs)),
            })
        } else {
            Child::Leaf(*parent)
        }
    }

    match inner(state, parent) {
        Child::Tree(tree) => tree,
        Child::Leaf(_) => unreachable!("Did not build a tree from parent: {:?}", parent),
    }
}

fn get_label_diffs(state: &State, z: usize) -> (Vec<Label>, Vec<Label>) {
    let correct = get_correct_tree(z);

    let correct_labels: HashSet<Label> =
        Child::Tree(get_labeled_correct_tree(state, correct.clone()))
            .flatten()
            .iter()
            .map(|v| v.clone().get_label())
            .collect();

    let actual_labels: HashSet<Label> = Child::Tree(get_tree_from_gates(state, &Label::Z(z)))
        .flatten()
        .iter()
        .map(|v| v.clone().get_label())
        .collect();

    let mut correct_only: Vec<Label> = correct_labels.difference(&actual_labels).cloned().collect();
    let mut input_only: Vec<Label> = actual_labels.difference(&correct_labels).cloned().collect();

    correct_only.sort();
    input_only.sort();

    (correct_only, input_only)
}

fn solve(parsed: &State) -> String {
    // What a journey this one was. Multiple days of trying out all sorts of ideas.
    //
    // The first approach I kept trying to push through was a localized brute force swap search,
    // running the full simulation each step, and detecting where to start this swap search by
    // finding the first LSB that didn't match in x+y compared to z. I think the lineage derivation
    // probably had some bugs (the logic to get a local area around the malformed z-out), for
    // example I wasn't even looking at any z-outs for a long time in that subset, and also there
    // were probably many small bugs about state mutation in unexpected places.
    //
    // I did notice the tree depth of each z-out is nicely increasing, after the 1st (or maybe 1st
    // two), +3 depth each time, at least by the way I calculate depth here. That seemed to only be
    // a proxy, or rather, equivalent, to the first LSB mismatch search in the end, though.
    //
    // Eventually I was worn down enough to study the input in detail. Found a very simple 3-step
    // piecewise formula (described somewhere above) to derive each z-out tree. Armed with a
    // correct tree, comparing the delta to the input tree did eventually reveal only a small
    // handful of swap candidates, localized around the error. This solution takes the correct tree
    // and tries to label each gate's output from the input gates. If all gates can be labeled, the
    // structure of the input gates matches the correct tree, otherwise you have some swap
    // candidates.
    //
    // Even along this approach, I went down a few bad idea dead ends, like trying to look for
    // operator differences and swap that way, which blew up when I needed to address some node
    // deep down in one of the trees (and that doesn't even make sense, right?). Fortunately the
    // labels were largely assignable across, and it just worked, and I was saved. Particularly
    // because the gate input labels are invariant, so there's always a base truth to build up
    // from.
    //
    // The flamegraph for today looks like all the other days combined, in complexity of layers and
    // columns, but each cell is mostly just another clone call. Hilarious. Maybe this is a good
    // one to try to accept that lifetimes are a thing and finally really learn them.

    let mut zs: Vec<&Label> = parsed
        .wires
        .iter()
        .filter(|wire| matches!(wire, Label::Z(_)))
        .collect();
    zs.sort();

    let mut state = parsed.clone();
    let mut swaps: Vec<(Label, Label)> = Vec::new();

    let mut z = 0;
    while z < zs.len() {
        let (correct_only, actual_only) = get_label_diffs(&state, z);

        // This doesn't happen until the end, but fortunately I don't think the swaps are at the
        // end, so this works out.
        if (correct_only.is_empty() && !actual_only.is_empty())
            || (!correct_only.is_empty() && actual_only.is_empty())
        {
            panic!("Only one side of the outputs was in the delta!");
        }

        if DEBUG {
            // If you also print the next two rounds (z+1, z+2), at least for my input, the search
            // space is drastically reduced compared to z, for one swap in particular. When I tried
            // to implement something like that (always iterate below on the z+1 or z+2
            // differences), I got a stackoverflow, and quickly accepted worse performance at this
            // point at the end.
            if !correct_only.is_empty() || !actual_only.is_empty() {
                println!("z={:?}", z);
                println!("labels only in correct tree: {:?}", correct_only);
                println!("  labels only in input tree: {:?}", actual_only);
            }
        }

        // The labels don't match up between the structure of the correct tree and the input tree,
        // so try swapping each correct label with each input label until the structure does match
        // up (because no label deltas are seen between them).
        let mut found = false;
        for a_out in &correct_only {
            // The correct tree labels are only known for x,y,z wires. Can't swap an unknown wire
            // (labeled "") with a known wire, so skip it here!
            if matches!(a_out, Label::A(0)) {
                continue;
            }
            for b_out in &actual_only {
                if DEBUG {
                    println!("Trying swap of {:?} and {:?}", a_out, b_out);
                }

                let mut check = state.clone();

                let a_gate = parsed.outputs.get(a_out).unwrap();
                let b_gate = parsed.outputs.get(b_out).unwrap();
                check.outputs.insert(*a_out, *b_gate);
                check.outputs.insert(*b_out, *a_gate);

                let (c_only, a_only) = get_label_diffs(&check, z);

                if c_only.is_empty() && a_only.is_empty() {
                    if DEBUG {
                        println!("This worked, saving state...");
                    }
                    found = true;
                    state.outputs.insert(*a_out, *b_gate);
                    state.outputs.insert(*b_out, *a_gate);
                    swaps.push((*a_out, *b_out));
                    break;
                } else {
                    // Clippy does not like nested if/else so I'm adding a comment.
                    if DEBUG {
                        println!("Not a fix because:");
                        println!("z={:?}", z);
                        println!("labels only in correct tree: {:?}", c_only);
                        println!("  labels only in input tree: {:?}", a_only);
                    }
                }
            }
            if found {
                break;
            }
        }

        if swaps.len() == 4 {
            // If you don't do this abort just because you have the answer, and want to terminate
            // naturally after visiting all the gates, you have to go look at what the last z-out
            // layer formula is, and I don't want to do that at this point.
            if DEBUG {
                println!("Found 4 swaps, aborting search.");
            }
            break;
        }

        z += 1;
    }

    let mut swaps: Vec<Label> = swaps
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .cloned()
        .collect();
    swaps.sort();

    swaps
        .iter()
        .map(|label| label.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn main() {
    let input = load_input(2024, 24);
    let parsed = parse_input(input);
    let answer = solve(&parsed);
    println!("Answer: {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24b_parse() {
        assert_eq!("x123", Label::from_str("x123").unwrap().to_string());
        assert_eq!("y987", Label::from_str("y987").unwrap().to_string());
        assert_eq!("z555", Label::from_str("z555").unwrap().to_string());
        assert_eq!("blah", Label::from_str("blah").unwrap().to_string());
    }
}
