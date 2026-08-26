use bilt_rust::{
    atom::{BinaryOp, Symbol},
    equation::Equation,
    isolate::isolate,
    tree::Tree,
};

#[test]
fn test_e2e_isolate() {
    let lhs = Tree::new_leaf(Symbol::get("z"));
    let rhs = Tree::new(
        BinaryOp::MUL,
        vec![
            Tree::new_leaf(Symbol::get("c")),
            Tree::new(
                BinaryOp::ADD,
                vec![
                    Tree::new_leaf(Symbol::get("x")),
                    Tree::new_leaf(Symbol::get("y")),
                ],
            ),
        ],
    );

    let my_eqn = Equation::new(lhs, rhs);
    let target = Symbol::get("y");
    let rearranged = isolate(&target, &my_eqn);

    assert_eq!(rearranged.to_string(), "(y = ((z / c) - x))");
}

#[test]
fn test_isolate_pre_operation() {
    let lhs = Tree::new(
        BinaryOp::ADD,
        vec![
            Tree::new_leaf(Symbol::get("x")),
            Tree::new_leaf(Symbol::get("y")),
        ],
    );
    let rhs = Tree::new_leaf(Symbol::get("c"));
    let eqn = Equation::new(lhs, rhs);
    let rearranged = isolate(&Symbol::get("y"), &eqn);

    assert_eq!(rearranged.to_string(), "(y = (c - x))");
}

#[test]
fn test_isolate_post_operation() {
    let lhs = Tree::new(
        BinaryOp::ADD,
        vec![
            Tree::new_leaf(Symbol::get("y")),
            Tree::new_leaf(Symbol::get("x")),
        ],
    );
    let rhs = Tree::new_leaf(Symbol::get("c"));
    let eqn = Equation::new(lhs, rhs);
    let rearranged = isolate(&Symbol::get("y"), &eqn);

    assert_eq!(rearranged.to_string(), "(y = (c - x))");
}

#[test]
fn test_isolate_with_non_commutative_op() {
    // x / y = c
    let lhs = Tree::new(
        BinaryOp::DIV,
        vec![
            Tree::new_leaf(Symbol::get("x")),
            Tree::new_leaf(Symbol::get("y")),
        ],
    );
    let rhs = Tree::new_leaf(Symbol::get("c"));
    let eqn = Equation::new(lhs, rhs);
    let rearranged = isolate(&Symbol::get("y"), &eqn);

    assert_eq!(rearranged.to_string(), "(y = (x / c))");
}

#[test]
fn test_isolate_with_non_commutative_op_and_left_target() {
    // x / y = c
    let lhs = Tree::new(
        BinaryOp::DIV,
        vec![
            Tree::new_leaf(Symbol::get("x")),
            Tree::new_leaf(Symbol::get("y")),
        ],
    );
    let rhs = Tree::new_leaf(Symbol::get("c"));
    let eqn = Equation::new(lhs, rhs);
    let rearranged = isolate(&Symbol::get("x"), &eqn);

    assert_eq!(rearranged.to_string(), "(x = (c * y))");
}
