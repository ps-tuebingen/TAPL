use common::{errors::ErrorKind, Label};
use std::{collections::HashMap, fmt};

pub type TypeVar = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVar),
    Unit,
    Nat,
    Bool,
    Fun { from: Box<Type>, to: Box<Type> },
    Package { ty_var: TypeVar, ty: Box<Type> },
    Record(HashMap<Label, Type>),
}

impl Type {
    pub fn fun(from: Type, to: Type) -> Type {
        Type::Fun {
            from: Box::new(from),
            to: Box::new(to),
        }
    }

    pub fn pack(var: &str, ty: Type) -> Type {
        Type::Package {
            ty_var: var.to_owned(),
            ty: Box::new(ty),
        }
    }

    pub fn record(recs: Vec<(&str, Type)>) -> Type {
        Type::Record(
            recs.into_iter()
                .map(|(label, ty)| (label.to_owned(), ty))
                .collect(),
        )
    }

    pub fn as_fun(self) -> Result<(Type, Type), ErrorKind> {
        if let Type::Fun { from, to } = self {
            Ok((*from, *to))
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Function Type".to_owned(),
            })
        }
    }

    pub fn as_pack(self) -> Result<(TypeVar, Type), ErrorKind> {
        if let Type::Package { ty_var, ty } = self {
            Ok((ty_var, *ty))
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Package Type".to_owned(),
            })
        }
    }

    pub fn as_rec(self) -> Result<HashMap<Label, Type>, ErrorKind> {
        if let Type::Record(recs) = self {
            Ok(recs)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Record Type".to_owned(),
            })
        }
    }

    pub fn check_equal(&self, other: &Type) -> Result<(), ErrorKind> {
        if self == other {
            Ok(())
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: other.to_string(),
            })
        }
    }
}

impl From<TypeVar> for Type {
    fn from(s: TypeVar) -> Type {
        Type::Var(s)
    }
}

impl From<&str> for Type {
    fn from(s: &str) -> Type {
        Type::Var(s.to_owned())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => f.write_str(v),
            Type::Unit => f.write_str("Unit"),
            Type::Nat => f.write_str("Nat"),
            Type::Bool => f.write_str("Bool"),
            Type::Fun { from, to } => write!(f, "({from}) -> ({to})"),
            Type::Package { ty_var, ty } => write!(f, "{{ exists {ty_var}, {ty} }}"),
            Type::Record(recs) => {
                let mut recs: Vec<(&String, &Type)> = recs.iter().collect();
                recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
                write!(
                    f,
                    "{{ {} }}",
                    recs.iter()
                        .map(|(label, ty)| format!("{label}:{ty}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}
