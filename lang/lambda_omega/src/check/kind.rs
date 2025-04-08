use super::Env;
use crate::{errors::Error, kinds::Kind, types::Type};
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
                let (from, to) = fun_kind.as_arrow()?;
                let arg_kind = arg.check(env)?;
                if from == arg_kind {
                    Ok(to)
                } else {
                    Err(Error::KindMismatch {
                        found: arg_kind,
                        expected: from.to_string(),
                    })
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
                    Err(Error::KindMismatch {
                        found: non_star,
                        expected: "*".to_owned(),
                    })
                }
            }
            Type::Forall { var, ty } => {
                env.add_tyvar(var, &Kind::Star);
                let ty_knd = ty.check(env)?;
                if let Kind::Star = ty_knd {
                    Ok(Kind::Star)
                } else {
                    Err(Error::KindMismatch {
                        found: ty_knd,
                        expected: "*".to_owned(),
                    })
                }
            }
        }
    }
}
