use crate::{errors::ErrorKind, traits::SubstTy};
use std::fmt;

pub type TypeVar = String;

pub mod app;
pub mod existential;
pub mod fun;
pub mod lambda;
pub mod record;
pub mod universal;

pub use app::OpApp;
pub use existential::Existential;
pub use fun::Fun;
pub use lambda::OpLambda;
pub use record::RecTy;
pub use universal::Universal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Var(TypeVar),
    Fun(Fun),
    Universal(Universal),
    OpLambda(OpLambda),
    OpApp(OpApp),
    Existential(Existential),
    Record(RecTy),
    Bool,
    Unit,
    Nat,
}

impl Type {
    pub fn as_fun(self) -> Result<Fun, ErrorKind> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self,
                expected: "Function Type".to_owned(),
            })
        }
    }

    pub fn as_universal(self) -> Result<Universal, ErrorKind> {
        if let Type::Universal(uni) = self {
            Ok(uni)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self,
                expected: "Universal Type".to_owned(),
            })
        }
    }

    pub fn as_oplambda(self) -> Result<OpLambda, ErrorKind> {
        if let Type::OpLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self,
                expected: "Type Operator".to_owned(),
            })
        }
    }

    pub fn as_existential(self) -> Result<Existential, ErrorKind> {
        if let Type::Existential(ex) = self {
            Ok(ex)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self,
                expected: "Existential Type".to_owned(),
            })
        }
    }

    pub fn as_rec(self) -> Result<RecTy, ErrorKind> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self,
                expected: "Record Type".to_owned(),
            })
        }
    }

    pub fn eval(self) -> Result<Type, ErrorKind> {
        if let Type::OpApp(app) = self {
            let fun_evaled = app.fun.eval()?;
            let lam = fun_evaled.as_oplambda()?;
            lam.body.subst_ty(&lam.var, *app.arg).eval()
        } else {
            Ok(self)
        }
    }

    pub fn check_equal(&self, other: &Type) -> Result<(), ErrorKind> {
        if self == other {
            return Ok(());
        }

        match (self, other) {
            (Type::Fun(fun1), Type::Fun(fun2)) => {
                fun1.from.check_equal(&fun2.from)?;
                fun1.to.check_equal(&fun2.to)
            }
            (Type::Universal(uni1), Type::Universal(uni2)) => {
                uni1.kind.check_equal(&uni2.kind)?;
                let uni2_subst = uni2
                    .ty
                    .clone()
                    .subst_ty(&uni2.var, uni1.var.as_str().into());
                uni2_subst.check_equal(&uni1.ty)
            }
            (Type::OpLambda(lam1), Type::OpLambda(lam2)) => {
                lam1.annot.check_equal(&lam2.annot)?;
                let lam2_subst = lam2
                    .body
                    .clone()
                    .subst_ty(&lam2.var, lam1.var.as_str().into());
                lam2_subst.check_equal(&lam1.body)
            }
            (Type::Existential(ex1), Type::Existential(ex2)) => {
                ex1.kind.check_equal(&ex2.kind)?;
                let ex2_subst = ex2
                    .ty
                    .clone()
                    .subst_ty(&ex2.ty_var, ex1.ty_var.as_str().into());
                ex2_subst.check_equal(&ex1.ty)
            }
            (Type::Record(rec1), Type::Record(rec2)) => {
                let mut recs2 = rec2.records.clone();
                for (label, ty1) in rec1.records.clone() {
                    let ty2 = recs2
                        .remove(&label)
                        .ok_or(ErrorKind::UndefinedLabel(label))?;
                    ty1.check_equal(&ty2)?;
                }
                if let Some(key) = recs2.into_keys().next() {
                    Err(ErrorKind::UndefinedLabel(key))
                } else {
                    Ok(())
                }
            }
            (Type::OpApp(app1), Type::OpApp(app2)) => {
                app1.fun.check_equal(&app2.fun)?;
                app1.arg.check_equal(&app2.arg)
            }
            (Type::OpApp(app), ty) => {
                let evaled = Type::OpApp(app.clone()).eval()?;
                evaled.check_equal(ty)
            }
            (ty1, ty2 @ Type::OpApp(_)) => ty2.check_equal(ty1),
            _ => Err(ErrorKind::TypeMismatch {
                found: self.clone(),
                expected: other.to_string(),
            }),
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
            Type::Fun(fun) => fun.subst_ty(v, ty).into(),
            Type::Universal(uni) => uni.subst_ty(v, ty).into(),
            Type::OpLambda(lam) => lam.subst_ty(v, ty).into(),
            Type::OpApp(app) => app.subst_ty(v, ty).into(),
            Type::Existential(ex) => ex.subst_ty(v, ty).into(),
            Type::Record(rec) => rec.subst_ty(v, ty).into(),
            Type::Bool => Type::Bool,
            Type::Unit => Type::Unit,
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
            Type::OpLambda(lambda) => lambda.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::OpApp(app) => app.fmt(f),
            Type::Universal(uni) => uni.fmt(f),
            Type::Existential(ex) => ex.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Bool => f.write_str("Bool"),
            Type::Unit => f.write_str("Unit"),
            Type::Nat => f.write_str("Nat"),
        }
    }
}
