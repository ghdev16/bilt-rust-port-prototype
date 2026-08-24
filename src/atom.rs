#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Atom {
    atom_type: AtomType,
    name: &'static str,
}

impl Atom {
    pub fn atom_type(&self) -> AtomType {
        self.atom_type
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AtomType {
    Symbol,
    BinaryOp,
    UnaryOp,
    RelationOp,
}

pub struct Symbol;

impl Symbol {
    pub const ONE: Atom = Self::get("1");

    pub const fn get(name: &'static str) -> Atom {
        Atom {
            atom_type: AtomType::Symbol,
            name: name,
        }
    }
}

pub struct BinaryOp;

impl BinaryOp {
    pub const ADD: Atom = Self::get("+");
    pub const SUB: Atom = Self::get("-");
    pub const MUL: Atom = Self::get("*");
    pub const DIV: Atom = Self::get("/");

    pub const fn get(name: &'static str) -> Atom {
        Atom {
            atom_type: AtomType::BinaryOp,
            name: name,
        }
    }
}

pub struct UnaryOp;

impl UnaryOp {
    pub const NEG: Atom = Self::get("-");
    pub const INV: Atom = Self::get("INV"); // General inversion, have this point to a particular operator.

    pub const fn get(name: &'static str) -> Atom {
        Atom {
            atom_type: AtomType::UnaryOp,
            name: name,
        }
    }
}

pub struct RelationOp;

impl RelationOp {
    pub const EQUALS: Atom = Self::get("=");
    pub const LESS_THAN: Atom = Self::get("<");
    pub const LESS_THAN_OR_EQUALS: Atom = Self::get("<=");

    pub const fn get(name: &'static str) -> Atom {
        Atom {
            atom_type: AtomType::RelationOp,
            name: name,
        }
    }
}
