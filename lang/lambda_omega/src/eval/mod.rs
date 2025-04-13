use crate::{syntax::Term, to_err};
pub mod value;
use common::errors::{Error, ErrorKind, ErrorLocation};
use common::Eval;
pub use value::Value;

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}

impl Eval<'_> for Term {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        match self {
            Term::Unit => Ok(Value::Unit),
            Term::Const(i) => Ok(Value::Const(i)),
            Term::True => Ok(Value::True),
            Term::False => Ok(Value::False),
            Term::Var(v) => Err(to_eval_err(ErrorKind::FreeVariable(v))),
            Term::Lambda { var, annot, body } => Ok(Value::Lambda {
                var,
                annot,
                body: *body,
            }),
            Term::App { fun, arg } => {
                let fun_val = fun.eval(_env)?;
                let (var, _, body) = fun_val.as_lambda().map_err(to_eval_err)?;
                body.subst(&var, *arg).eval(_env)
            }
            Term::TyLambda { var, kind, body } => Ok(Value::TyLambda {
                var,
                annot: kind,
                body: *body,
            }),
            Term::TyApp { fun, arg } => {
                let fun_val = fun.eval(_env)?;
                let (var, _, body) = fun_val.as_tylambda().map_err(to_eval_err)?;
                body.subst_ty(&var, arg).eval(_env)
            }
        }
    }
}
