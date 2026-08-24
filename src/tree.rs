use crate::atom::Atom;

#[derive(Debug, PartialEq, Clone)]
pub struct Tree {
    value: Atom,
    children: Vec<Tree>,
}

impl Tree {
    pub fn new(value: Atom, children: Vec<Tree>) -> Self {
        Tree { value, children }
    }

    pub fn new_leaf(value: Atom) -> Self {
        Tree {
            value,
            children: vec![],
        }
    }

    pub fn new_chain(atom_chain: &[Atom]) -> Tree {
        if atom_chain.len() == 0 {
            panic!("Expected at least one element in the chain");
        } else if atom_chain.len() == 1 {
            Tree::new_leaf(atom_chain[0])
        } else {
            let mut cur_node = Tree::new_leaf(*atom_chain.last().unwrap());

            for i in (0..(atom_chain.len() - 1)).rev() {
                cur_node = Tree::new(atom_chain[i], vec![cur_node]);
            }

            cur_node
        }
    }

    pub fn value(&self) -> &Atom {
        &self.value
    }

    pub fn children(&self) -> &Vec<Tree> {
        &self.children
    }

    pub fn into_children(self) -> Vec<Tree> {
        self.children
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn is_binary(&self) -> bool {
        self.children.len() == 2
    }

    pub fn left(&self) -> Option<&Tree> {
        if !self.is_binary() {
            None
        } else {
            Some(&self.children[0])
        }
    }

    pub fn right(&self) -> Option<&Tree> {
        if !self.is_binary() {
            None
        } else {
            Some(&self.children[1])
        }
    }
}
