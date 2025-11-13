use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{DerivationRule, symbols::Symbol};
use std::collections::HashMap;
use std::collections::HashSet;
use syntax::{
    Label, eval_context::EvalContext, language::Language, terms::Record,
    values::Record as RecordVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Record<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
    RecordVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let mut records: HashMap<Label, <Self::Lang as Language>::Value> = HashMap::new();
        let mut old_recs = self.records.clone();
        let mut steps = vec![];
        for (lb, t) in self.records {
            let term_res = t.eval(env)?;
            let val = term_res.val();
            records.insert(lb.clone(), val.clone());

            let rule_recs = old_recs.clone();
            let lb_clone = lb.clone();
            steps.extend(
                term_res
                    .congruence(&move |t| {
                        let mut recs_mut = rule_recs.clone();
                        recs_mut.insert(lb_clone.clone(), t);
                        Self::new(recs_mut).into()
                    })
                    .into_iter(),
            );
            old_recs.insert(lb, val.into());
        }
        let val = RecordVal::<Lang>::new::<<Self::Lang as Language>::Value>(records);
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::eval_cong(
            |sym| {
                Symbol::brack(vec![
                    Symbol::many(Symbol::sub(Symbol::Value, "i")),
                    sym,
                    Symbol::many(Symbol::sub(Symbol::Term, "j")),
                ])
            },
            "E-Record1",
        )])
    }
}
