use super::Eval;
use crate::{
    errors::{Error, ErrorKind},
    terms::{Term, Variant, VariantCase},
    traits::is_value::IsValue,
    traits::subst::SubstTerm,
};

impl Eval for Variant {
    fn eval_once(self) -> Result<Term, Error> {
        if self.term.is_value() {
            Ok(self.into())
        //E-Variant
        } else {
            let term_evaled = self.term.eval_once()?;
            Ok(Variant {
                label: self.label,
                term: Box::new(term_evaled),
                annot: self.annot,
            }
            .into())
        }
    }
}
impl Eval for VariantCase {
    fn eval_once(self) -> Result<Term, Error> {
        //E-VariantCaseRhs
        if self.bound_term.is_value() {
            let variant = self
                .bound_term
                .as_variant()
                .map_err(|knd| Error::eval(knd, &self))?;

            let matching_pattern = self
                .patterns
                .clone()
                .into_iter()
                .find(|pt| pt.label == variant.label)
                .ok_or(Error::eval(
                    ErrorKind::UndefinedLabel(variant.label.clone()),
                    &VariantCase {
                        bound_term: Box::new(variant.clone().into()),
                        patterns: self.patterns,
                    },
                ))?;
            Ok(matching_pattern
                .rhs
                .subst(matching_pattern.bound_var, *variant.term))

        //E-VariantCase
        } else {
            let term_evaled = self.bound_term.eval_once()?;
            Ok(VariantCase {
                bound_term: Box::new(term_evaled),
                patterns: self.patterns,
            }
            .into())
        }
    }
}
