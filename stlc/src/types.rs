use super::Var;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Type {
    Unit,
    Fun(Box<Type>, Box<Type>),
    Bool,
    Prod(Box<Type>, Box<Type>),
    Tup(Vec<Type>),
    Record(HashMap<Var, Type>),
    Sum(Box<Type>, Box<Type>),
    Variant(HashMap<Var, Type>),
    Optional(Box<Type>),
    List(Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Unit => f.write_str("Unit"),
            Type::Fun(ty1, ty2) => write!(f, "{ty1} -> {ty2}"),
            Type::Bool => f.write_str("Bool"),
            Type::Prod(ty1, ty2) => write!(f, "{ty1} x {ty2}"),
            Type::Tup(tys) => write!(
                f,
                "({})",
                tys.iter()
                    .map(|ty| format!("{}", ty))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Record(records) => write!(
                f,
                "{{ {} }}",
                records
                    .iter()
                    .map(|(label, ty)| format!("{label}:{ty}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Sum(ty1, ty2) => write!(f, "{ty1}+{ty2}"),
            Type::Variant(vars) => write!(
                f,
                "<{}>",
                vars.iter()
                    .map(|(label, ty)| format!("{label}:{ty}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Optional(ty) => write!(f, "Option {ty}"),
            Type::List(ty) => write!(f, "List {ty}"),
        }
    }
}
