use bilt_rust::{
    atom::{BinaryOp, Symbol},
    tree::Tree,
};

#[test]
fn test_leaf() {
    let x = Symbol::get("x");

    let my_tree = Tree::new(x, vec![]);
    assert_eq!(*my_tree.value(), x);
    assert!(my_tree.is_leaf());
    assert!(!my_tree.is_binary());
    assert_eq!(my_tree.left(), None);
    assert_eq!(my_tree.right(), None);
}

#[test]
fn test_add() {
    let x = Symbol::get("x");
    let y = Symbol::get("y");

    let lhs = Tree::new_leaf(x);
    let rhs = Tree::new_leaf(y);

    let my_tree = Tree::new(BinaryOp::ADD, vec![lhs.clone(), rhs.clone()]);
    assert_eq!(*my_tree.value(), BinaryOp::ADD);
    assert!(!my_tree.is_leaf());
    assert!(my_tree.is_binary());
    assert_eq!(my_tree.left(), Some(&lhs));
    assert_eq!(my_tree.right(), Some(&rhs));
}

#[test]
fn test_new_chain() {
    let chain_tree = Tree::new_chain(&[Symbol::get("x"), Symbol::get("y"), Symbol::get("z")]);
    let expected = Tree::new(
        Symbol::get("x"),
        vec![Tree::new(
            Symbol::get("y"),
            vec![Tree::new_leaf(Symbol::get("z"))],
        )],
    );
    assert_eq!(chain_tree, expected);
}
