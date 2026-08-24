use bilt_rust::{
    atom::{BinaryOp, Symbol, UnaryOp},
    equation::Equation,
    tree::Tree,
};

#[test]
fn test_string_conversion_for_binary_tree() {
    let lhs = Tree::new_leaf(Symbol::get("z"));
    let rhs = Tree::new(
        BinaryOp::ADD,
        vec![
            Tree::new_leaf(Symbol::get("x")),
            Tree::new_leaf(Symbol::get("y")),
        ],
    );

    let my_eqn = Equation::new(lhs, rhs);
    let eqn_str = format!("{}", my_eqn);

    assert_eq!(eqn_str, "(z = (x + y))");
}

#[test]
fn test_string_conversion_for_non_binary_tree() {
    let lhs = Tree::new_leaf(Symbol::get("w"));
    let rhs = Tree::new(
        BinaryOp::ADD,
        vec![
            Tree::new_leaf(Symbol::get("x")),
            Tree::new_leaf(Symbol::get("y")),
            Tree::new_leaf(Symbol::get("z")),
        ],
    );

    let my_eqn = Equation::new(lhs, rhs);
    let eqn_str = format!("{}", my_eqn);

    assert_eq!(eqn_str, "(w = (x + y + z))");
}

#[test]
fn test_string_conversion_with_one_child() {
    let lhs = Tree::new_chain(&[UnaryOp::NEG, Symbol::get("x")]);
    let rhs = Tree::new_leaf(Symbol::get("y"));
    let eqn = Equation::new(lhs, rhs);

    assert_eq!(eqn.to_string(), "(-x = y)")
}
