use std::{collections::HashSet, fmt};

pub type TyVar = String;

#[derive(Debug, Clone)]
pub enum Type {
    Var(TyVar),
    Fun(Box<Type>, Box<Type>),
    Forall(TyVar, Box<Type>),
}

impl Type {
    pub fn fun(from: Type, to: Type) -> Type {
        Type::Fun(Box::new(from), Box::new(to))
    }

    pub fn forall(v: &str, ty: Type) -> Type {
        Type::Forall(v.to_owned(), Box::new(ty))
    }

    pub fn free_tyvars(&self) -> HashSet<TyVar> {
        match self {
            Type::Var(v) => HashSet::from([v.clone()]),
            Type::Fun(from, to) => {
                let mut vars = from.free_tyvars();
                vars.extend(to.free_tyvars());
                vars
            }
            Type::Forall(v, ty) => {
                let mut vars = ty.free_tyvars();
                vars.remove(v);
                vars
            }
        }
    }

    pub fn fresh_tyvar(&self) -> TyVar {
        let free_v = self.free_tyvars();
        let mut ind = 0;
        while free_v.contains(&format!("X{ind}")) {
            ind += 1
        }
        format!("X{ind}")
    }

    pub fn subst(self, v: &TyVar, ty: Type) -> Type {
        match self {
            Type::Var(v2) => {
                if *v == v2 {
                    ty
                } else {
                    Type::Var(v2)
                }
            }
            Type::Fun(from, to) => Type::Fun(
                Box::new((*from).subst(v, ty.clone())),
                Box::new((*to).subst(v, ty)),
            ),
            Type::Forall(v2, inner) => {
                if *v == v2 {
                    Type::Forall(v2, inner)
                } else {
                    Type::Forall(v2, Box::new((*inner).subst(v, ty)))
                }
            }
        }
    }
}
impl From<TyVar> for Type {
    fn from(s: TyVar) -> Type {
        Type::Var(s)
    }
}

impl From<&str> for Type {
    fn from(s: &str) -> Type {
        Type::Var(s.to_owned())
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Var(_), Type::Var(_)) => true,
            (Type::Fun(from1, to1), Type::Fun(from2, to2)) => *to1 == *to2 && from1 == from2,
            (Type::Forall(_, ty1), Type::Forall(_, ty2)) => ty1 == ty2,
            _ => false,
        }
    }
}

impl Eq for Type {}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Var(v) => f.write_str(v),
            Type::Fun(from, to) => write!(f, "({from}) → ({to})"),
            Type::Forall(v, ty) => write!(f, "∀{v}.{ty}"),
        }
    }
}
