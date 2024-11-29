use super::Var;
use std::collections::HashMap;
use std::fmt;

pub type TypeVar = String;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Type {
    Var(TypeVar),
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

impl Type {
    pub fn subst(self, v: &TypeVar, ty: Type) -> Type {
        match self {
            Type::Var(v2) => {
                if *v == v2 {
                    ty
                } else {
                    Type::Var(v2)
                }
            }
            Type::Unit => Type::Unit,
            Type::Fun(ty1, ty2) => Type::Fun(
                Box::new((*ty1).subst(v, ty.clone())),
                Box::new((*ty2).subst(v, ty)),
            ),
            Type::Bool => Type::Bool,
            Type::Nat => Type::Nat,
            Type::Prod(ty1, ty2) => Type::Prod(
                Box::new((*ty1).subst(v, ty.clone())),
                Box::new((*ty2).subst(v, ty)),
            ),
            Type::Tup(tys) => Type::Tup(
                tys.into_iter()
                    .map(|ty2| ty2.subst(v, ty.clone()))
                    .collect(),
            ),
            Type::Record(records) => Type::Record(
                records
                    .into_iter()
                    .map(|(label, ty2)| (label, ty2.subst(v, ty.clone())))
                    .collect(),
            ),
            Type::Sum(ty1, ty2) => Type::Sum(
                Box::new((*ty1).subst(v, ty.clone())),
                Box::new((*ty2).subst(v, ty)),
            ),
            Type::Variant(variants) => Type::Variant(
                variants
                    .into_iter()
                    .map(|(label, ty2)| (label, ty2.subst(v, ty.clone())))
                    .collect(),
            ),
            Type::Optional(ty2) => Type::Optional(Box::new((*ty2).subst(v, ty))),
            Type::List(ty2) => Type::List(Box::new((*ty2).subst(v, ty))),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => write!(f, "{v}"),
            Type::Unit => f.write_str("Unit"),
            Type::Fun(ty1, ty2) => write!(f, "{ty1} -> {ty2}"),
            Type::Bool => f.write_str("Bool"),
            Type::Nat => f.write_str("Nat"),
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
