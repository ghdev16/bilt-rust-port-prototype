#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Atom {
    atom_type: &'static str,
    name: &'static str,
}

impl Atom {
    pub const fn get(atom_type: &'static str, name: &'static str) -> Self {
        Self::check_type_is_not_internal(atom_type);
        Atom { atom_type, name }
    }

    pub fn atom_type(&self) -> &str {
        self.atom_type
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    const fn check_type_is_not_internal(atom_type: &str) {
        if AtomType::is_internal_type(atom_type) {
            panic!("Can't get atom of internal type through this public getter.");
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct AtomType;

impl AtomType {
    pub const SYMBOL: &str = "SYMBOL";
    pub const BINARY_OP: &str = "BINARY_OP";
    pub const UNARY_OP: &str = "UNARY_OP";
    pub const RELATION_OP: &str = "RELATION_OP";

    const fn is_internal_type(atom_type: &str) -> bool {
        matches!(
            atom_type.as_bytes(),
            b"SYMBOL" | b"BINARY_OP" | b"UNARY_OP" | b"RELATION_OP"
        )
    }
}

pub struct Symbol;

impl Symbol {
    pub const ONE: Atom = Self::get("1");

    pub const fn get(name: &'static str) -> Atom {
        Atom {
            atom_type: AtomType::SYMBOL,
            name,
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
            atom_type: AtomType::BINARY_OP,
            name,
        }
    }
}

pub struct UnaryOp;

impl UnaryOp {
    pub const NEG: Atom = Self::get("-");

    pub const fn get(name: &'static str) -> Atom {
        Atom {
            atom_type: AtomType::UNARY_OP,
            name,
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
            atom_type: AtomType::RELATION_OP,
            name,
        }
    }
}
