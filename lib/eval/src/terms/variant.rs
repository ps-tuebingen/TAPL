use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext, language::Language, terms::Variant, values::Variant as VariantVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Variant<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
    Variant<Lang>: Into<Lang::Term>,
    VariantVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val = VariantVal::<Lang>::new(&self.label, term_val, self.ty.clone());

        let steps =
            term_res.congruence(&move |t| Variant::new(&self.label, t, self.ty.clone()).into());
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::eval_cong(
            |sym| {
                vec![
                    SpecialChar::AngBrackO.into(),
                    Symbol::Label,
                    SpecialChar::Equals.into(),
                    sym,
                    SpecialChar::AngBrackC.into(),
                ]
            },
            "E-Variant",
        )])
    }
}
