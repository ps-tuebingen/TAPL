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
    terms::{False as FalseT, IsZero, Term, True as TrueT},
    values::{False, True, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for IsZero<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    IsZero<Lang>: Into<Lang::Term>,
    True<Lang>: Into<Lang::Value>,
    TrueT<Lang>: Into<Lang::Term>,
    False<Lang>: Into<Lang::Value>,
    FalseT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let inner_res = self.term.eval(env)?;
        let val = inner_res.val();
        let num = val.clone().into_num()?;
        let mut steps = inner_res.congruence(&move |t| IsZero::new(t).into());
        if num.num == 0 {
            steps.push(EvalStep::iszero_true(IsZero::new(val)));
            Ok(EvalTrace::new(steps, True::new()))
        } else {
            steps.push(EvalStep::iszero_false(IsZero::new(val)));
            Ok(EvalTrace::new(steps, False::new()))
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval(
                vec![Keyword::IsZero.into(), Symbol::paren(0)],
                Keyword::True,
                "E-IsZeroZero",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::IsZero.into(),
                    Symbol::paren(vec![Keyword::Succ.into(), Symbol::paren(Symbol::Value)]),
                ],
                Keyword::False,
                "E-IsZeroSucc",
            ),
            DerivationRule::eval_cong(
                |sym| vec![Keyword::IsZero.into(), Symbol::paren(sym)],
                "E-IsZero1",
            ),
        ])
    }
}
