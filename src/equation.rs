use core::fmt;

use crate::{atom::RelationOp, tree::Tree};

pub struct Equation {
    tree: Tree,
}

impl Equation {
    pub fn new(lhs: Tree, rhs: Tree) -> Self {
        Equation {
            tree: Tree::new(RelationOp::EQUALS, vec![lhs, rhs]),
        }
    }

    pub fn left(&self) -> &Tree {
        self.tree.left().unwrap()
    }

    pub fn right(&self) -> &Tree {
        self.tree.right().unwrap()
    }

    pub fn into_sides(self) -> (Tree, Tree) {
        let [lhs_tree, rhs_tree]: [Tree; 2] = self
            .tree
            .into_children()
            .try_into()
            .expect("Equation should have exactly two children trees.");

        (lhs_tree, rhs_tree)
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", tree_to_str(&self.tree))
    }
}

fn tree_to_str(tree: &Tree) -> String {
    if tree.is_leaf() {
        tree.value().name().to_string()
    } else {
        if tree.children().len() == 1 {
            format!(
                "{}{}",
                tree.value().name(),
                tree_to_str(&tree.children()[0])
            )
        } else {
            format!(
                "({})",
                tree.children()
                    .iter()
                    .map(tree_to_str)
                    .collect::<Vec<String>>()
                    .join(&format!(" {} ", tree.value().name()))
            )
        }
    }
}
