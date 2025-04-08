use super::Env;
use crate::{errors::Error, kinds::Kind, syntax::Term, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::Var(v) => env.get_var(v),
            Term::Const(_) => Ok(Type::Nat),
            Term::Unit => Ok(Type::Unit),
            Term::True => Ok(Type::Bool),
            Term::False => Ok(Type::Bool),
            Term::Lambda { var, annot, body } => {
                let annot_kind = annot.check(&mut env.clone())?;
                env.add_var(var, annot);
                if annot_kind == Kind::Star {
                    let body_ty = body.check(env)?;
                    Ok(Type::Fun {
                        from: Box::new(annot.clone()),
                        to: Box::new(body_ty),
                    })
                } else {
                    Err(Error::KindMismatch {
                        found: annot_kind,
                        expected: "*".to_owned(),
                    })
                }
            }
            Term::App { fun, arg } => {
                let fun_ty = fun.check(&mut env.clone())?;
                let (from, to) = fun_ty.as_fun()?;
                let arg_ty = arg.check(env)?;
                if from == arg_ty {
                    Ok(to)
                } else {
                    Err(Error::TypeMismatch {
                        found: arg_ty,
                        expected: from.to_string(),
                    })
                }
            }
            Term::TyLambda { var, kind, body } => {
                env.add_tyvar(var, kind);
                let body_ty = body.check(env)?;
                Ok(Type::Forall {
                    var: var.clone(),
                    ty: Box::new(body_ty),
                })
            }
            Term::TyApp { fun, arg } => {
                let fun_ty = fun.check(env)?;
                let (var, ty) = fun_ty.as_forall()?;
                Ok(ty.subst(&var, arg.clone()))
            }
        }
    }
}
