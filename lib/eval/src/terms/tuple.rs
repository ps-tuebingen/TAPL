use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{DerivationRule, symbols::Symbol};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Term, Tuple},
    values::Tuple as TupleVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Tuple<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
    TupleVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let mut vals = vec![];
        let mut old_terms = self.terms.clone();
        let mut steps = vec![];
        for (ind, t) in self.terms.into_iter().enumerate() {
            let term_res = t.eval(env)?;
            let val = term_res.val();
            vals.push(val.clone());

            let cong_vals = old_terms.clone();
            steps.extend(term_res.congruence(&move |t| {
                let mut cong_mut = cong_vals.clone();
                cong_mut[ind] = t;
                Self::new(cong_mut).into()
            }));

            old_terms[ind] = val.into();
        }
        let val = TupleVal::<Lang>::new(vals);
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::eval_cong(
            |sym| {
                Symbol::paren(vec![
                    Symbol::many(Symbol::sub(Symbol::Value, "i")),
                    sym,
                    Symbol::many(Symbol::sub(Symbol::Term, "i")),
                ])
            },
            "E-Tup1",
        )])
    }
}
