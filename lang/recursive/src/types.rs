use crate::traits::subst::SubstTy;
use common::{errors::ErrorKind, Label};
use std::{collections::HashMap, fmt};

pub type TypeVar = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    TypeVar(TypeVar),
    Unit,
    Fun { from: Box<Type>, to: Box<Type> },
    Mu(TypeVar, Box<Type>),
    Variant(Vec<(Label, Type)>),
    Pair(Box<Type>, Box<Type>),
    Nat,
    Bool,
    Record(HashMap<Label, Type>),
}

impl Type {
    pub fn fun(from: Type, to: Type) -> Type {
        Type::Fun {
            from: Box::new(from),
            to: Box::new(to),
        }
    }

    pub fn as_fun(&self) -> Result<(Type, Type), ErrorKind> {
        if let Type::Fun { from, to } = self {
            Ok((*from.clone(), *to.clone()))
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Function Type".to_owned(),
            })
        }
    }

    pub fn mu(var: &str, ty: Type) -> Type {
        Type::Mu(var.to_owned(), Box::new(ty))
    }

    pub fn as_mu(&self) -> Result<(TypeVar, Type), ErrorKind> {
        if let Type::Mu(v, ty) = self {
            Ok((v.clone(), *ty.clone()))
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Recursive Type".to_owned(),
            })
        }
    }

    pub fn variant(vars: Vec<(&str, Type)>) -> Type {
        Type::Variant(
            vars.into_iter()
                .map(|(label, ty)| (label.to_owned(), ty))
                .collect(),
        )
    }

    pub fn as_variant(&self) -> Result<Vec<(Label, Type)>, ErrorKind> {
        if let Type::Variant(vars) = self {
            Ok(vars.clone())
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Variant Type".to_owned(),
            })
        }
    }

    pub fn pair(fst: Type, snd: Type) -> Type {
        Type::Pair(Box::new(fst), Box::new(snd))
    }

    pub fn as_pair(&self) -> Result<(Type, Type), ErrorKind> {
        if let Type::Pair(fst, snd) = self {
            Ok((*fst.clone(), *snd.clone()))
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Pair Type".to_owned(),
            })
        }
    }

    pub fn record(records: &[(&str, Type)]) -> Type {
        let mut recs = HashMap::new();
        for (label, ty) in records {
            recs.insert(label.to_string(), ty.clone());
        }
        Type::Record(recs)
    }

    pub fn as_record(&self) -> Result<HashMap<Label, Type>, ErrorKind> {
        if let Type::Record(recs) = self {
            Ok(recs.clone())
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Record Type".to_owned(),
            })
        }
    }

    pub fn equal(&self, ty: &Type) -> Result<Type, ErrorKind> {
        if self == ty {
            Ok(self.clone())
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: ty.to_string(),
            })
        }
    }
}

impl SubstTy for Type {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        match self {
            Type::TypeVar(v2) => {
                if v == v2 {
                    ty
                } else {
                    Type::TypeVar(v2)
                }
            }
            Type::Unit => Type::Unit,
            Type::Fun { from, to } => Type::Fun {
                from: Box::new(from.subst_ty(v.clone(), ty.clone())),
                to: Box::new(to.subst_ty(v, ty)),
            },
            Type::Mu(var, t) => {
                if v == var {
                    Type::Mu(var, t)
                } else {
                    Type::Mu(var, Box::new(t.subst_ty(v, ty)))
                }
            }
            Type::Variant(vars) => Type::Variant(
                vars.into_iter()
                    .map(|(label, t)| (label, t.subst_ty(v.clone(), ty.clone())))
                    .collect(),
            ),
            Type::Pair(ty1, ty2) => Type::Pair(
                Box::new(ty1.subst_ty(v.clone(), ty.clone())),
                Box::new(ty2.subst_ty(v, ty)),
            ),
            Type::Nat => Type::Nat,
            Type::Bool => Type::Bool,
            Type::Record(recs) => Type::Record(
                recs.into_iter()
                    .map(|(label, typ)| (label, typ.subst_ty(v.clone(), ty.clone())))
                    .collect(),
            ),
        }
    }
}

impl From<&str> for Type {
    fn from(s: &str) -> Type {
        Type::TypeVar(s.to_owned())
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::TypeVar(v) => f.write_str(v),
            Type::Unit => f.write_str("Unit"),
            Type::Fun { from, to } => write!(f, "({from}) -> ({to})"),
            Type::Mu(var, ty) => write!(f, "mu {var}.{ty}"),
            Type::Variant(vars) => {
                let mut vars: Vec<&(Label, Type)> = vars.iter().collect();
                vars.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
                write!(
                    f,
                    "<{}>",
                    vars.iter()
                        .map(|(label, ty)| format!("{label}:{ty}"))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Type::Pair(ty1, ty2) => write!(f, "{{ {ty1}, {ty2} }}"),
            Type::Nat => f.write_str("Nat"),
            Type::Bool => f.write_str("Bool"),
            Type::Record(recs) => {
                let mut recs: Vec<(&Label, &Type)> = recs.iter().collect();
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
