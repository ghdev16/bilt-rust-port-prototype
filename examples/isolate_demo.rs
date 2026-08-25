use bilt_rust::{
    atom::{BinaryOp, Symbol},
    equation::Equation,
    isolate::isolate,
    tree::Tree,
};

fn main() {
    // Define equation `x / y = c``
    let lhs = Tree::new(
        BinaryOp::DIV,
        vec![
            Tree::new_leaf(Symbol::get("x")),
            Tree::new_leaf(Symbol::get("y")),
        ],
    );
    let rhs = Tree::new_leaf(Symbol::get("c"));
    let eqn = Equation::new(lhs, rhs);
    println!("Original equation: {}", eqn);

    let target = Symbol::get("y");

    // Rearrange equation to isolate `target` on the LHS
    let rearranged = isolate(&target, &eqn);
    println!("Rearranged for `{}`: {}", target.name(), rearranged);
}
