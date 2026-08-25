use crate::{
    atom::{Atom, BinaryOp, Symbol, UnaryOp},
    equation::Equation,
    tree::Tree,
};

pub struct IsolationOp;

impl IsolationOp {
    pub const ISOLATION_OP: &str = "ISOLATION_OP";
    pub const INV: Atom = Self::get("INV");

    pub const fn get(name: &'static str) -> Atom {
        Atom::get(Self::ISOLATION_OP, name)
    }
}

/// Assumption: subject only appears once in the equation
pub fn isolate(target: &Atom, in_eqn: &Equation) -> Equation {
    let analysis = analyze_equation_with_target(target, in_eqn);
    validate_from_analysis(&analysis);

    let mut lhs = in_eqn.left().clone();
    let mut rhs = in_eqn.right().clone();

    if analysis.need_to_swap {
        std::mem::swap(&mut lhs, &mut rhs);
    }

    (lhs, rhs) = isolate_by_inversions(target, lhs, rhs);

    lhs = simplify_inversions(lhs);
    rhs = simplify_inversions(rhs);

    Equation::new(lhs, rhs)
}

struct EquationWithTargetAnalysis {
    total_occurrences: usize,
    need_to_swap: bool,
}

fn analyze_equation_with_target(target: &Atom, in_eqn: &Equation) -> EquationWithTargetAnalysis {
    let lhs_occurrences = count_occurrences(target, in_eqn.left());
    let rhs_occurrences = count_occurrences(target, in_eqn.right());

    EquationWithTargetAnalysis {
        total_occurrences: lhs_occurrences + rhs_occurrences,
        need_to_swap: rhs_occurrences == 1,
    }
}

fn validate_from_analysis(analysis: &EquationWithTargetAnalysis) {
    if analysis.total_occurrences == 0 {
        panic!("Target not present in equation.");
    } else if analysis.total_occurrences > 1 {
        unimplemented!("Target present more than once in equation, not currently supported.");
    }
}

fn isolate_by_inversions(target: &Atom, lhs: Tree, rhs: Tree) -> (Tree, Tree) {
    let mut lhs = lhs;
    let mut rhs = rhs;

    while lhs.value() != target {
        let occurrences_of_target_in_children: Vec<usize> = lhs
            .children()
            .iter()
            .map(|child| count_occurrences(target, child))
            .collect();

        let index_of_child_with_target = occurrences_of_target_in_children
            .iter()
            .position(|&x| x == 1)
            .unwrap();

        let operator = *lhs.value();
        let mut children = lhs.into_children();
        let post_children = children.drain((index_of_child_with_target + 1)..);

        for child in post_children {
            let inverse_part = Tree::new(IsolationOp::INV, vec![Tree::new(operator, vec![child])]);
            rhs = Tree::new(operator, vec![rhs, inverse_part]);
        }

        let next_lhs = children.pop().unwrap();

        for child in children {
            let inverse_part = Tree::new(IsolationOp::INV, vec![Tree::new(operator, vec![child])]);
            rhs = Tree::new(operator, vec![inverse_part, rhs]);
        }

        lhs = next_lhs
    }

    (lhs, rhs)
}

fn count_occurrences(target: &Atom, in_tree: &Tree) -> usize {
    let mut count = 0;

    if in_tree.value() == target {
        count += 1;
    }

    if in_tree.is_leaf() {
        return count;
    }

    count += in_tree
        .children()
        .iter()
        .fold(0, |acc, sub_tree| acc + count_occurrences(target, sub_tree));

    count
}

fn simplify_inversions(tree: Tree) -> Tree {
    if tree.is_leaf() {
        return tree;
    }

    if tree.value() == &IsolationOp::INV {
        assert_eq!(tree.children().len(), 1);
        assert_eq!(tree.children()[0].children().len(), 1);

        let inv_node = tree;
        assert_eq!(inv_node.children().len(), 1);

        let op_node = inv_node.into_children().pop().unwrap();
        assert_eq!(op_node.children().len(), 1);

        let op_to_invert = *op_node.value();
        let op_child = op_node.into_children().pop().unwrap();
        let simplified_child = simplify_inversions(op_child);

        match op_to_invert {
            BinaryOp::SUB | BinaryOp::DIV => simplified_child,
            BinaryOp::ADD => Tree::new(UnaryOp::NEG, vec![simplified_child]),
            BinaryOp::MUL => Tree::new(
                BinaryOp::DIV,
                vec![Tree::new_leaf(Symbol::ONE), simplified_child],
            ),
            _ => unimplemented!(),
        }
    } else {
        Tree::new(
            *tree.value(),
            tree.into_children()
                .into_iter()
                .map(|child| simplify_inversions(child))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        atom::{BinaryOp, Symbol, UnaryOp},
        isolate::{IsolationOp, count_occurrences, simplify_inversions},
        tree::Tree,
    };

    #[test]
    fn test_count_occurrences() {
        let lhs = Tree::new(
            BinaryOp::ADD,
            vec![
                Tree::new_leaf(Symbol::get("y")),
                Tree::new(
                    BinaryOp::MUL,
                    vec![
                        Tree::new_leaf(Symbol::get("a")),
                        Tree::new(
                            BinaryOp::ADD,
                            vec![
                                Tree::new_leaf(Symbol::get("y")),
                                Tree::new(
                                    BinaryOp::MUL,
                                    vec![
                                        Tree::new_leaf(Symbol::get("b")),
                                        Tree::new_leaf(Symbol::get("y")),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        );

        let rhs = Tree::new(
            BinaryOp::MUL,
            vec![
                Tree::new_leaf(Symbol::get("c")),
                Tree::new(
                    BinaryOp::ADD,
                    vec![
                        Tree::new_leaf(Symbol::get("x")),
                        Tree::new(
                            BinaryOp::MUL,
                            vec![
                                Tree::new_leaf(Symbol::get("d")),
                                Tree::new(
                                    BinaryOp::ADD,
                                    vec![
                                        Tree::new_leaf(Symbol::get("y")),
                                        Tree::new(
                                            BinaryOp::MUL,
                                            vec![
                                                Tree::new_leaf(Symbol::get("e")),
                                                Tree::new_leaf(Symbol::get("y")),
                                            ],
                                        ),
                                    ],
                                ),
                            ],
                        ),
                    ],
                ),
            ],
        );

        assert_eq!(count_occurrences(&Symbol::get("y"), &lhs), 3);
        assert_eq!(count_occurrences(&Symbol::get("y"), &rhs), 2);
    }

    #[test]
    fn test_simplify_invs() {
        let inv_subtraction = Tree::new_chain(&[IsolationOp::INV, BinaryOp::SUB, Symbol::get("c")]);
        let inv_division = Tree::new_chain(&[IsolationOp::INV, BinaryOp::DIV, Symbol::get("c")]);
        let inv_addition = Tree::new_chain(&[IsolationOp::INV, BinaryOp::ADD, Symbol::get("c")]);
        let inv_multiplication =
            Tree::new_chain(&[IsolationOp::INV, BinaryOp::MUL, Symbol::get("c")]);

        assert_eq!(
            simplify_inversions(inv_subtraction),
            Tree::new_leaf(Symbol::get("c"))
        );
        assert_eq!(
            simplify_inversions(inv_division),
            Tree::new_leaf(Symbol::get("c"))
        );
        assert_eq!(
            simplify_inversions(inv_addition),
            Tree::new_chain(&[UnaryOp::NEG, Symbol::get("c")])
        );
        assert_eq!(
            simplify_inversions(inv_multiplication),
            Tree::new(
                BinaryOp::DIV,
                vec![
                    Tree::new_leaf(Symbol::ONE),
                    Tree::new_leaf(Symbol::get("c"))
                ]
            )
        );
    }
}
