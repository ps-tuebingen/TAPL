use crate::{
    errors::{Error, ErrorKind},
    eval::Eval,
    syntax::kinds::Kind,
    traits::SubstTy,
};
use std::{collections::HashSet, fmt};
pub type TypeVar = String;

pub mod existential;
pub mod fun;
pub mod opapp;
pub mod oplambda;
pub mod record;
pub mod universal;
pub use existential::Existential;
pub use fun::Fun;
pub use opapp::OpApp;
pub use oplambda::OpLambda;
pub use record::RecordTy;
pub use universal::Universal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVar),
    Top(Kind),
    Fun(Fun),
    Universal(Universal),
    OpLambda(OpLambda),
    OpApp(OpApp),
    Existential(Existential),
    Record(RecordTy),
    Nat,
}

impl Type {
    pub fn as_oplambda(self) -> Result<OpLambda, ErrorKind> {
        if let Type::OpLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::BadType {
                found: self,
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    pub fn as_fun(self) -> Result<Fun, ErrorKind> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(ErrorKind::BadType {
                found: self,
                expected: "Function Type".to_owned(),
            })
        }
    }

    pub fn as_universal(self) -> Result<Universal, ErrorKind> {
        if let Type::Universal(uni) = self {
            Ok(uni)
        } else {
            Err(ErrorKind::BadType {
                found: self,
                expected: "Universal Type".to_owned(),
            })
        }
    }

    pub fn as_opapp(self) -> Result<OpApp, ErrorKind> {
        if let Type::OpApp(app) = self {
            Ok(app)
        } else {
            Err(ErrorKind::BadType {
                found: self,
                expected: "Operator Application".to_owned(),
            })
        }
    }

    pub fn as_top(self) -> Result<Kind, ErrorKind> {
        if let Type::Top(knd) = self {
            Ok(knd)
        } else {
            Err(ErrorKind::BadType {
                found: self,
                expected: "Top".to_owned(),
            })
        }
    }

    pub fn as_rec(self) -> Result<RecordTy, ErrorKind> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ErrorKind::BadType {
                found: self,
                expected: "Record Type".to_owned(),
            })
        }
    }

    pub fn as_existential(self) -> Result<Existential, ErrorKind> {
        if let Type::Existential(ex) = self {
            Ok(ex)
        } else {
            Err(ErrorKind::BadType {
                found: self,
                expected: "Existential Type".to_owned(),
            })
        }
    }

    pub fn check_equal(&self, other: &Type) -> Result<(), Error> {
        let error_fun = |knd: ErrorKind| Error::equiv(knd, self.clone(), other.clone());
        match self {
            Type::Var(_) => {
                if matches!(other, Type::Var(_)) {
                    Ok(())
                } else {
                    Err(Error::equiv(
                        ErrorKind::BadType {
                            found: other.clone(),
                            expected: "Type Variable".to_owned(),
                        },
                        self.clone(),
                        other.clone(),
                    ))
                }
            }
            Type::Top(knd1) => {
                let knd2 = other.clone().as_top().map_err(error_fun)?;
                knd1.check_equal(&knd2).map_err(error_fun)
            }
            Type::Fun(fun1) => {
                let fun2 = other.clone().as_fun().map_err(error_fun)?;
                fun1.from.check_equal(&fun2.from)?;
                fun1.to.check_equal(&fun2.to)
            }
            Type::Universal(uni1) => {
                let uni2 = other.clone().as_universal().map_err(error_fun)?;
                uni1.sup_ty.check_equal(&uni2.sup_ty)?;
                let body_subst = uni1
                    .ty
                    .clone()
                    .subst_ty(&uni1.var, uni2.var.as_str().into());
                body_subst.check_equal(&uni2.ty)
            }
            Type::OpLambda(lam1) => {
                let lam2 = other.clone().as_oplambda().map_err(error_fun)?;
                lam1.annot.check_equal(&lam2.annot).map_err(error_fun)?;
                let body_subst = lam1
                    .body
                    .clone()
                    .subst_ty(&lam1.var, lam2.var.as_str().into());
                body_subst.check_equal(&lam2.body)
            }
            Type::OpApp(app1) => {
                let app2 = other.clone().as_opapp().map_err(error_fun)?;
                app1.fun.check_equal(&app2.fun)?;
                app1.arg.check_equal(&app2.arg)
            }
            Type::Existential(ex1) => {
                let other_evaled = other.clone().eval(&mut Default::default())?;
                let ex2 = other_evaled.as_existential().map_err(error_fun)?;
                ex1.sup_ty.check_equal(&ex2.sup_ty)?;
                let body_subst = ex1.ty.clone().subst_ty(&ex1.var, ex2.var.as_str().into());
                body_subst.check_equal(&ex2.ty)
            }
            Type::Record(rec) => {
                let rec2 = other.clone().as_rec().map_err(error_fun)?;
                let rec_labels = rec.records.keys().collect::<HashSet<&String>>();
                let rec2_labels = rec2.records.keys().collect::<HashSet<&String>>();
                let diff: HashSet<&&String> = rec_labels.difference(&rec2_labels).collect();
                if !diff.is_empty() {
                    return Err(error_fun(ErrorKind::UndefinedLabel(
                        (**diff.iter().next().unwrap()).clone(),
                    )));
                }
                for label in rec_labels {
                    let ty1 = rec
                        .records
                        .get(label)
                        .ok_or(ErrorKind::UndefinedLabel(label.clone()))
                        .map_err(error_fun)?;
                    let ty2 = rec2
                        .records
                        .get(label)
                        .ok_or(ErrorKind::UndefinedLabel(label.clone()))
                        .map_err(error_fun)?;
                    ty1.check_equal(ty2)?;
                }
                Ok(())
            }
            Type::Nat => {
                if matches!(other, Type::Nat) {
                    Ok(())
                } else {
                    Err(error_fun(ErrorKind::BadType {
                        found: other.clone(),
                        expected: "Nat".to_owned(),
                    }))
                }
            }
        }
    }
}

impl SubstTy for Type {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        match self {
            Type::Var(var) => {
                if *v == var {
                    ty
                } else {
                    Type::Var(var)
                }
            }
            Type::Top(knd) => Type::Top(knd),
            Type::Fun(fun) => fun.subst_ty(v, ty).into(),
            Type::Universal(uni) => uni.subst_ty(v, ty).into(),
            Type::OpLambda(lam) => lam.subst_ty(v, ty).into(),
            Type::OpApp(app) => app.subst_ty(v, ty).into(),
            Type::Existential(ex) => ex.subst_ty(v, ty).into(),
            Type::Record(rec) => rec.subst_ty(v, ty).into(),
            Type::Nat => Type::Nat,
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
            Type::Top(knd) => write!(f, "Top::{knd}"),
            Type::Fun(fun) => fun.fmt(f),
            Type::Universal(uni) => uni.fmt(f),
            Type::OpLambda(lam) => lam.fmt(f),
            Type::OpApp(app) => app.fmt(f),
            Type::Existential(ex) => ex.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Nat => f.write_str("Nat"),
        }
    }
}
