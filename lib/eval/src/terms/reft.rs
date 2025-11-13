use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Loc as LocT, Ref, Term},
    values::Loc,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Ref<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    LocT<Lang>: Into<Lang::Term>,
    Self: Into<Lang::Term>,
    Loc<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let fresh_loc = env.fresh_location();
        env.save_location(fresh_loc, term_val.clone());

        let mut steps = term_res.congruence(&move |t| Self::new(t).into());
        let val = Loc::new(fresh_loc);
        let last_step = EvalStep::reft(Self::new(term_val), fresh_loc);
        steps.push(last_step);

        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Ref.into(),
                        Symbol::paren(sym),
                        SpecialChar::Pipe.into(),
                        SpecialChar::Mu.into(),
                    ]
                },
                "E-Ref1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Ref.into(),
                    Symbol::paren(Symbol::Value),
                    SpecialChar::Pipe.into(),
                    SpecialChar::Mu.into(),
                ],
                vec![
                    Symbol::Location,
                    SpecialChar::Pipe.into(),
                    Symbol::sqbrack(Symbol::mapto(Symbol::Location, Symbol::Value)),
                ],
                "E-Ref",
            ),
        ])
    }
}
