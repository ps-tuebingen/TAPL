use super::{to_eval_err, Value};
use crate::{
    terms::{Variant, VariantCase},
    traits::subst::SubstTerm,
};
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for Variant {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(_env)?;
        Ok(Value::Variant {
            label: self.label,
            val: Box::new(term_val),
            annot: self.annot,
        })
    }
}
impl Eval<'_> for VariantCase {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.clone().eval(_env)?;
        let (label, val, _) = bound_val.clone().into_variant().map_err(to_eval_err)?;

        let matching_pattern = self
            .patterns
            .clone()
            .into_iter()
            .find(|pt| pt.label == label)
            .ok_or(to_eval_err(ErrorKind::UndefinedLabel(label.clone())))?;
        matching_pattern
            .rhs
            .subst(matching_pattern.bound_var, val.into())
            .eval(_env)
    }
}
