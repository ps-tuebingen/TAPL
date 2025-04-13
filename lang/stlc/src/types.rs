use common::Var;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Unit,
    Fun(Box<Type>, Box<Type>),
    Bool,
    Nat,
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
            Type::Fun(ty1, ty2) => write!(f, "({ty1} -> {ty2})"),
            Type::Bool => f.write_str("Bool"),
            Type::Nat => f.write_str("Nat"),
            Type::Prod(ty1, ty2) => write!(f, "({ty1} x {ty2})"),
            Type::Tup(tys) => write!(
                f,
                "({})",
                tys.iter()
                    .map(|ty| format!("{}", ty))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Type::Record(records) => {
                let mut recs: Vec<(&String, &Type)> = records.iter().collect();
                recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));

                write!(
                    f,
                    "{{{}}}",
                    recs.iter()
                        .map(|(label, ty)| format!("{label}: {ty}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Type::Sum(ty1, ty2) => write!(f, "({ty1}+{ty2})"),
            Type::Variant(vars) => {
                let mut variants: Vec<(&String, &Type)> = vars.iter().collect();
                variants.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
                let variant_strs: Vec<String> = variants
                    .iter()
                    .map(|(lb, ty)| format!("{lb}:{ty}"))
                    .collect();
                write!(f, "<{}>", variant_strs.join(", "))
            }
            Type::Optional(ty) => write!(f, "Optional[{ty}]"),
            Type::List(ty) => write!(f, "List {ty}"),
        }
    }
}
