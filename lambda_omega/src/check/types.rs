use super::{kind_ty, Env};
use crate::{errors::Error, kinds::Kind, syntax::Term, types::Type};

pub fn check(t: &Term, env: &mut Env) -> Result<Type, Error> {
    match t {
        Term::Var(v) => env.get_var(v),
        Term::Unit => Ok(Type::Unit),
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
    }
}
