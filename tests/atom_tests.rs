use bilt_rust::atom::{Atom, AtomType, BinaryOp, RelationOp, Symbol};

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

    assert_eq!(x.atom_type(), AtomType::SYMBOL);
    assert_eq!(add.atom_type(), AtomType::BINARY_OP);
    assert_eq!(equals.atom_type(), AtomType::RELATION_OP);
}

#[test]
#[should_panic(expected = "Can't get atom of internal type through this public getter.")]
fn external_code_blocked_from_instantiating_internally_typed_atom() {
    _ = Atom::get(AtomType::SYMBOL, "+");
}
