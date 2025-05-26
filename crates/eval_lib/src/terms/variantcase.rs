use crate::{to_eval_err, values::ValueGroup, Eval};
use common::errors::{Error, ErrorKind};
use syntax::terms::{Term, VariantCase};

impl<T> Eval for VariantCase<T>
where
    T: Term + Eval,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(env)?;
        let var_val = bound_val.into_variant().map_err(to_eval_err)?;
        let matching = self
            .patterns
            .into_iter()
            .find(|pt| *pt.label == var_val.label)
            .ok_or(to_eval_err(ErrorKind::UndefinedLabel(var_val.label)))?;
        matching
            .rhs
            .subst(&matching.bound_var, &((*var_val.val).into()))
            .eval(env)
    }
}
