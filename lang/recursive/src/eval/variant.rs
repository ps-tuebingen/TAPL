use super::Value;
use crate::{
    errors::{Error, ErrorKind},
    terms::{Variant, VariantCase},
    traits::subst::SubstTerm,
};
use common::Eval;
impl Eval<'_> for Variant {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
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
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let bound_val = self.bound_term.clone().eval(_env)?;
        let (label, val, _) = bound_val
            .clone()
            .into_variant()
            .map_err(|knd| Error::eval(knd, &self))?;

        let matching_pattern = self
            .patterns
            .clone()
            .into_iter()
            .find(|pt| pt.label == label)
            .ok_or(Error::eval(
                ErrorKind::UndefinedLabel(label.clone()),
                &VariantCase {
                    bound_term: Box::new(bound_val.clone().into()),
                    patterns: self.patterns,
                },
            ))?;
        matching_pattern
            .rhs
            .subst(matching_pattern.bound_var, val.into())
            .eval(_env)
    }
}
