use crate::{
    atom::{Atom, BinaryOp},
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

    (lhs, rhs) = isolate_directly(target, lhs, rhs);

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

fn isolate_directly(target: &Atom, lhs: Tree, rhs: Tree) -> (Tree, Tree) {
    let mut lhs = lhs;
    let mut rhs = rhs;

    while lhs.value() != target {
        (lhs, rhs) = do_isolation_step(target, lhs, rhs);
    }

    (lhs, rhs)
}

fn do_isolation_step(target: &Atom, lhs: Tree, rhs: Tree) -> (Tree, Tree) {
    let top_level_op = *lhs.value();
    let index_of_child_with_target = locate_target(target, &lhs);
    let [left_operand, right_operand] = lhs
        .into_children()
        .try_into()
        .expect("Expected exactly two children."); // TODO: Currently won't work for a unary op or multi op in the original equation (e.g. -a + b = c)
    let is_target_on_left = index_of_child_with_target == 0;

    if is_target_on_left {
        match top_level_op {
            BinaryOp::ADD => (
                left_operand,
                Tree::new(BinaryOp::SUB, vec![rhs, right_operand]),
            ),
            BinaryOp::SUB => (
                left_operand,
                Tree::new(BinaryOp::ADD, vec![rhs, right_operand]),
            ),
            BinaryOp::MUL => (
                left_operand,
                Tree::new(BinaryOp::DIV, vec![rhs, right_operand]),
            ),
            BinaryOp::DIV => (
                left_operand,
                Tree::new(BinaryOp::MUL, vec![rhs, right_operand]),
            ),
            _ => unimplemented!(),
        }
    } else {
        match top_level_op {
            BinaryOp::ADD => (
                right_operand,
                Tree::new(BinaryOp::SUB, vec![rhs, left_operand]),
            ),
            BinaryOp::SUB => (
                right_operand,
                Tree::new(BinaryOp::SUB, vec![left_operand, rhs]),
            ),
            BinaryOp::MUL => (
                right_operand,
                Tree::new(BinaryOp::DIV, vec![rhs, left_operand]),
            ),
            BinaryOp::DIV => (
                right_operand,
                Tree::new(BinaryOp::DIV, vec![left_operand, rhs]),
            ),
            _ => unimplemented!(),
        }
    }
}

fn locate_target(target: &Atom, lhs: &Tree) -> usize {
    let occurrences_of_target_in_children: Vec<usize> = lhs
        .children()
        .iter()
        .map(|child| count_occurrences(target, child))
        .collect();

    occurrences_of_target_in_children
        .iter()
        .position(|&x| x == 1)
        .unwrap()
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

#[cfg(test)]
mod tests {
    use crate::{
        atom::{BinaryOp, Symbol},
        isolate::{count_occurrences, do_isolation_step},
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
    fn test_do_isolation_step_add_with_left_target() {
        let lhs = Tree::new(
            BinaryOp::ADD,
            vec![
                Tree::new_leaf(Symbol::get("a")),
                Tree::new_leaf(Symbol::get("b")),
            ],
        );
        let rhs = Tree::new_leaf(Symbol::get("c"));
        let target = Symbol::get("a");

        let (new_lhs, new_rhs) = do_isolation_step(&target, lhs, rhs);

        assert_eq!(new_lhs, Tree::new_leaf(Symbol::get("a")));
        assert_eq!(
            new_rhs,
            Tree::new(
                BinaryOp::SUB,
                vec![
                    Tree::new_leaf(Symbol::get("c")),
                    Tree::new_leaf(Symbol::get("b"))
                ]
            )
        );
    }

    #[test]
    fn test_do_isolation_step_add_with_right_target() {
        let lhs = Tree::new(
            BinaryOp::ADD,
            vec![
                Tree::new_leaf(Symbol::get("a")),
                Tree::new_leaf(Symbol::get("b")),
            ],
        );
        let rhs = Tree::new_leaf(Symbol::get("c"));
        let target = Symbol::get("b");

        let (new_lhs, new_rhs) = do_isolation_step(&target, lhs, rhs);

        assert_eq!(new_lhs, Tree::new_leaf(Symbol::get("b")));
        assert_eq!(
            new_rhs,
            Tree::new(
                BinaryOp::SUB,
                vec![
                    Tree::new_leaf(Symbol::get("c")),
                    Tree::new_leaf(Symbol::get("a"))
                ]
            )
        );
    }

    #[test]
    fn test_do_isolation_step_sub_with_left_target() {
        let lhs = Tree::new(
            BinaryOp::SUB,
            vec![
                Tree::new_leaf(Symbol::get("a")),
                Tree::new_leaf(Symbol::get("b")),
            ],
        );
        let rhs = Tree::new_leaf(Symbol::get("c"));
        let target = Symbol::get("a");

        let (new_lhs, new_rhs) = do_isolation_step(&target, lhs, rhs);

        assert_eq!(new_lhs, Tree::new_leaf(Symbol::get("a")));
        assert_eq!(
            new_rhs,
            Tree::new(
                BinaryOp::ADD,
                vec![
                    Tree::new_leaf(Symbol::get("c")),
                    Tree::new_leaf(Symbol::get("b"))
                ]
            )
        );
    }

    #[test]
    fn test_do_isolation_step_sub_with_right_target() {
        let lhs = Tree::new(
            BinaryOp::SUB,
            vec![
                Tree::new_leaf(Symbol::get("a")),
                Tree::new_leaf(Symbol::get("b")),
            ],
        );
        let rhs = Tree::new_leaf(Symbol::get("c"));
        let target = Symbol::get("b");

        let (new_lhs, new_rhs) = do_isolation_step(&target, lhs, rhs);

        assert_eq!(new_lhs, Tree::new_leaf(Symbol::get("b")));
        assert_eq!(
            new_rhs,
            Tree::new(
                BinaryOp::SUB,
                vec![
                    Tree::new_leaf(Symbol::get("a")),
                    Tree::new_leaf(Symbol::get("c"))
                ]
            )
        );
    }

    #[test]
    fn test_do_isolation_step_mul_with_left_target() {
        let lhs = Tree::new(
            BinaryOp::MUL,
            vec![
                Tree::new_leaf(Symbol::get("a")),
                Tree::new_leaf(Symbol::get("b")),
            ],
        );
        let rhs = Tree::new_leaf(Symbol::get("c"));
        let target = Symbol::get("a");

        let (new_lhs, new_rhs) = do_isolation_step(&target, lhs, rhs);

        assert_eq!(new_lhs, Tree::new_leaf(Symbol::get("a")));
        assert_eq!(
            new_rhs,
            Tree::new(
                BinaryOp::DIV,
                vec![
                    Tree::new_leaf(Symbol::get("c")),
                    Tree::new_leaf(Symbol::get("b"))
                ]
            )
        );
    }

    #[test]
    fn test_do_isolation_step_mul_with_right_target() {
        let lhs = Tree::new(
            BinaryOp::MUL,
            vec![
                Tree::new_leaf(Symbol::get("a")),
                Tree::new_leaf(Symbol::get("b")),
            ],
        );
        let rhs = Tree::new_leaf(Symbol::get("c"));
        let target = Symbol::get("b");

        let (new_lhs, new_rhs) = do_isolation_step(&target, lhs, rhs);

        assert_eq!(new_lhs, Tree::new_leaf(Symbol::get("b")));
        assert_eq!(
            new_rhs,
            Tree::new(
                BinaryOp::DIV,
                vec![
                    Tree::new_leaf(Symbol::get("c")),
                    Tree::new_leaf(Symbol::get("a"))
                ]
            )
        );
    }

    #[test]
    fn test_do_isolation_step_div_with_left_target() {
        let lhs = Tree::new(
            BinaryOp::DIV,
            vec![
                Tree::new_leaf(Symbol::get("a")),
                Tree::new_leaf(Symbol::get("b")),
            ],
        );
        let rhs = Tree::new_leaf(Symbol::get("c"));
        let target = Symbol::get("a");

        let (new_lhs, new_rhs) = do_isolation_step(&target, lhs, rhs);

        assert_eq!(new_lhs, Tree::new_leaf(Symbol::get("a")));
        assert_eq!(
            new_rhs,
            Tree::new(
                BinaryOp::MUL,
                vec![
                    Tree::new_leaf(Symbol::get("c")),
                    Tree::new_leaf(Symbol::get("b"))
                ]
            )
        );
    }

    #[test]
    fn test_do_isolation_step_div_with_right_target() {
        let lhs = Tree::new(
            BinaryOp::DIV,
            vec![
                Tree::new_leaf(Symbol::get("a")),
                Tree::new_leaf(Symbol::get("b")),
            ],
        );
        let rhs = Tree::new_leaf(Symbol::get("c"));
        let target = Symbol::get("b");

        let (new_lhs, new_rhs) = do_isolation_step(&target, lhs, rhs);

        assert_eq!(new_lhs, Tree::new_leaf(Symbol::get("b")));
        assert_eq!(
            new_rhs,
            Tree::new(
                BinaryOp::DIV,
                vec![
                    Tree::new_leaf(Symbol::get("a")),
                    Tree::new_leaf(Symbol::get("c")),
                ]
            )
        );
    }
}
