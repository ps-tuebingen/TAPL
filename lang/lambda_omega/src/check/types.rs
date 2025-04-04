use super::{kind_ty, Env};
use crate::{errors::Error, kinds::Kind, syntax::Term, types::Type};

pub fn check(t: &Term, env: &mut Env) -> Result<Type, Error> {
    match t {
        Term::Var(v) => env.get_var(v),
        Term::Const(_) => Ok(Type::Nat),
        Term::Unit => Ok(Type::Unit),
        Term::True => Ok(Type::Bool),
        Term::False => Ok(Type::Bool),
        Term::Lambda { var, annot, body } => {
            let annot_kind = kind_ty(annot, &mut env.clone())?;
            env.add_var(var, annot);
            if annot_kind == Kind::Star {
                let body_ty = check(body, env)?;
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
            let fun_ty = check(fun, &mut env.clone())?;
            let (from, to) = fun_ty.as_fun()?;
            let arg_ty = check(arg, env)?;
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
            let body_ty = check(body, env)?;
            Ok(Type::Forall {
                var: var.clone(),
                ty: Box::new(body_ty),
            })
        }
        Term::TyApp { fun, arg } => {
            let fun_ty = check(fun, env)?;
            let (var, ty) = fun_ty.as_forall()?;
            Ok(ty.subst(&var, arg.clone()))
        }
    }
}
