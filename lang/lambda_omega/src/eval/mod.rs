use crate::{errors::Error, syntax::Term};
pub mod value;
pub use value::Value;

pub fn eval(t: Term) -> Result<Value, Error> {
    match t {
        Term::Unit => Ok(Value::Unit),
        Term::Var(v) => Err(Error::FreeVar(v)),
        Term::Lambda { var, annot, body } => Ok(Value::Lambda {
            var,
            annot,
            body: *body,
        }),
        Term::App { fun, arg } => {
            let fun_val = eval(*fun)?;
            let (var, _, body) = fun_val.as_lambda()?;
            eval(body.subst(&var, *arg))
        }
        Term::TyLambda { var, kind, body } => Ok(Value::TyLambda {
            var,
            annot: kind,
            body: *body,
        }),
        Term::TyApp { fun, arg } => {
            let fun_val = eval(*fun)?;
            let (var, _, body) = fun_val.as_tylambda()?;
            eval(body.subst_ty(&var, arg))
        }
    }
}
