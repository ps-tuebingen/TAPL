use crate::syntax::Label;
use common::errors::kind::ErrorKind;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

pub type TypeVar = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Var(TypeVar),
    Top,
    Nat,
    Fun {
        from: Box<Type>,
        to: Box<Type>,
    },
    Forall {
        var: TypeVar,
        sup_ty: Box<Type>,
        ty: Box<Type>,
    },
    Exists {
        var: TypeVar,
        sup_ty: Box<Type>,
        ty: Box<Type>,
    },
    Record(HashMap<Label, Type>),
}

impl Type {
    pub fn forall_unbounded(var: &str, ty: Type) -> Type {
        Type::Forall {
            var: var.to_owned(),
            sup_ty: Box::new(Type::Top),
            ty: Box::new(ty),
        }
    }

    pub fn forall(var: &str, sup_ty: Type, ty: Type) -> Type {
        Type::Forall {
            var: var.to_owned(),
            sup_ty: Box::new(sup_ty),
            ty: Box::new(ty),
        }
    }

    pub fn fun(from: Type, to: Type) -> Type {
        Type::Fun {
            from: Box::new(from),
            to: Box::new(to),
        }
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

    pub fn as_forall(self) -> Result<(TypeVar, Type, Type), ErrorKind> {
        if let Type::Forall { var, sup_ty, ty } = self {
            Ok((var, *sup_ty, *ty))
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Universal Type".to_owned(),
            })
        }
    }

    pub fn as_exists(self) -> Result<(TypeVar, Type, Type), ErrorKind> {
        if let Type::Exists { var, sup_ty, ty } = self {
            Ok((var, *sup_ty, *ty))
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Existential Type".to_owned(),
            })
        }
    }

    pub fn as_record(self) -> Result<HashMap<Label, Type>, ErrorKind> {
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

    pub fn free_tyvars(&self) -> HashSet<TypeVar> {
        match self {
            Type::Var(v) => HashSet::from([v.clone()]),
            Type::Top => HashSet::new(),
            Type::Nat => HashSet::new(),
            Type::Fun { from, to } => {
                let mut vars = from.free_tyvars();
                vars.extend(to.free_tyvars());
                vars
            }
            Type::Forall { var, sup_ty, ty } => {
                let mut vars = ty.free_tyvars();
                vars.remove(var);
                vars.extend(sup_ty.free_tyvars());
                vars
            }
            Type::Exists { var, sup_ty, ty } => {
                let mut vars = ty.free_tyvars();
                vars.remove(var);
                vars.extend(sup_ty.free_tyvars());
                vars
            }
            Type::Record(recs) => {
                let mut vars = HashSet::new();
                for (_, ty) in recs.iter() {
                    vars.extend(ty.free_tyvars());
                }
                vars
            }
        }
    }

    pub fn fresh_tyvar(&self, others: Vec<&Type>) -> TypeVar {
        let mut vars = self.free_tyvars();
        let _ = others.iter().map(|ty| vars.extend(ty.free_tyvars()));
        let mut num = 0;
        while vars.contains(&format!("X{num}")) {
            num += 1;
        }
        format!("X{num}")
    }

    pub fn rename_fresh(self, v: TypeVar) -> Type {
        let new_name = self.fresh_tyvar(vec![]);
        self.rename(v, new_name)
    }

    pub fn rename(self, v: TypeVar, new_name: TypeVar) -> Type {
        match self {
            Type::Var(var) => {
                if v == var {
                    Type::Var(new_name)
                } else {
                    Type::Var(var)
                }
            }
            Type::Top => Type::Top,
            Type::Nat => Type::Nat,
            Type::Fun { from, to } => Type::Fun {
                from: Box::new(from.rename(v.clone(), new_name.clone())),
                to: Box::new(to.rename(v, new_name)),
            },
            Type::Forall { var, sup_ty, ty } => Type::Forall {
                var: if var == v { new_name.clone() } else { var },
                sup_ty: Box::new(sup_ty.rename(v.clone(), new_name.clone())),
                ty: Box::new(ty.rename(v, new_name)),
            },
            Type::Exists { var, sup_ty, ty } => Type::Exists {
                var: if var == v { new_name.clone() } else { var },
                sup_ty: Box::new(sup_ty.rename(v.clone(), new_name.clone())),
                ty: Box::new(ty.rename(v, new_name)),
            },
            Type::Record(recs) => {
                let mut new_recs = HashMap::new();
                for (lb, ty) in recs.into_iter() {
                    let ty_renamed = ty.rename(v.clone(), new_name.clone());
                    new_recs.insert(lb, ty_renamed);
                }
                Type::Record(new_recs)
            }
        }
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
            Type::Top => f.write_str("Top"),
            Type::Nat => f.write_str("Nat"),
            Type::Fun { from, to } => write!(f, "({from}) -> ({to})"),
            Type::Forall { var, sup_ty, ty } => write!(f, "forall {var}<:{sup_ty}.{ty}"),
            Type::Exists { var, sup_ty, ty } => write!(f, "exists {var}<:{sup_ty},{ty}"),
            Type::Record(recs) => {
                let mut recs: Vec<(&Label, &Type)> = recs.iter().collect();
                recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
                let rec_strs: Vec<String> =
                    recs.iter().map(|(lb, ty)| format!("{lb}:{ty}")).collect();
                write!(f, "{{ {} }}", rec_strs.join(", "))
            }
        }
    }
}
