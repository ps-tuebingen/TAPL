use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Nat,
    Fun { from: Box<Type>, to: Box<Type> },
    Ref(Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unit => f.write_str("Unit"),
            Type::Nat => f.write_str("Nat"),
            Type::Fun { from, to } => write!(f, "({from})->({to})"),
            Type::Ref(ty) => write!(f, "Ref ({ty})"),
        }
    }
}
