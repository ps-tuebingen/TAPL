use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Term, Unfold},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Unfold<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Unfold<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let term_fold = term_val.clone().into_fold()?;

        let last_step = EvalStep::unfoldfold(
            Unfold::new(self.ty.clone(), term_val),
            *term_fold.val.clone(),
        );
        let mut steps = term_res.congruence(&move |t| Unfold::new(self.ty.clone(), t).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, *term_fold.val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| vec![Keyword::Unfold.into(), Symbol::sqbrack(Symbol::Type), sym],
                "E-Unfold1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Unfold.into(),
                    Symbol::sqbrack(Symbol::sub(Symbol::Type, 1)),
                    Symbol::paren(vec![
                        Keyword::Fold.into(),
                        Symbol::sqbrack(Symbol::sub(Symbol::Type, 2)),
                        Symbol::Value,
                    ]),
                ],
                Symbol::Value,
                "E-UnfoldFold",
            ),
        ])
    }
}
