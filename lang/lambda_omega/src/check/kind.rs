use super::{to_check_err, Env};
use crate::{kinds::Kind, types::Type};
use common::errors::{Error, ErrorKind};
use common::Typecheck;

impl<'a> Typecheck<'a> for Type {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Type::Var(v) => env.get_tyvar(v),
            Type::Unit => Ok(Kind::Star),
            Type::Nat => Ok(Kind::Star),
            Type::Bool => Ok(Kind::Star),
            Type::App { fun, arg } => {
                let fun_kind = fun.check(&mut env.clone())?;
                let (from, to) = fun_kind.as_arrow().map_err(to_check_err)?;
                let arg_kind = arg.check(env)?;
                if from == arg_kind {
                    Ok(to)
                } else {
                    Err(to_check_err(ErrorKind::KindMismatch {
                        found: arg_kind.to_string(),
                        expected: from.to_string(),
                    }))
                }
            }
            Type::Lambda { var, annot, body } => {
                env.add_tyvar(var, annot);
                let body_kind = body.check(env)?;
                Ok(Kind::Arrow(Box::new(annot.clone()), Box::new(body_kind)))
            }
            Type::Fun { from, to } => {
                let from_kind = from.check(&mut env.clone())?;
                let to_kind = to.check(env)?;
                if matches!(from_kind, Kind::Star) && matches!(to_kind, Kind::Star) {
                    Ok(Kind::Star)
                } else {
                    let non_star = if from_kind == Kind::Star {
                        to_kind
                    } else {
                        from_kind
                    };
                    Err(to_check_err(ErrorKind::KindMismatch {
                        found: non_star.to_string(),
                        expected: "*".to_owned(),
                    }))
                }
            }
            Type::Forall { var, ty } => {
                env.add_tyvar(var, &Kind::Star);
                let ty_knd = ty.check(env)?;
                if let Kind::Star = ty_knd {
                    Ok(Kind::Star)
                } else {
                    Err(to_check_err(ErrorKind::KindMismatch {
                        found: ty_knd.to_string(),
                        expected: "*".to_owned(),
                    }))
                }
            }
        }
    }
}
