use bilt_rust::atom::{AtomType, BinaryOp, RelationOp, Symbol};

#[test]
fn test_same_type_and_name_equal() {
    #[allow(non_snake_case)]
    let x1 = Symbol::get("x");
    let x2 = Symbol::get("x");

    assert_eq!(x1, x2);
}

#[test]
fn test_different_type_same_name_not_equal() {
    let x = Symbol::get("x");
    let multiply = BinaryOp::get("x");

    assert_ne!(x, multiply);
}

#[test]
fn test_types() {
    let x = Symbol::get("x");
    let add = BinaryOp::ADD;
    let equals = RelationOp::EQUALS;

    assert_eq!(x.atom_type(), AtomType::Symbol);
    assert_eq!(add.atom_type(), AtomType::BinaryOp);
    assert_eq!(equals.atom_type(), AtomType::RelationOp);
}
