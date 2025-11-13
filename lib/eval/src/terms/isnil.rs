use crate::Eval;
use errors::{ValueMismatch, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{False as FalseT, IsNil, Term, True as TrueT},
    values::{False, True, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for IsNil<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
    True<Lang>: Into<Lang::Value>,
    TrueT<Lang>: Into<Lang::Term>,
    False<Lang>: Into<Lang::Value>,
    FalseT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let (step, val) = if term_val.clone().into_nil().is_ok() {
            let last_step = EvalStep::isnil_true(self.ty.clone());
            (last_step, True::new().into())
        } else if term_val.clone().into_cons().is_ok() {
            let last_step = EvalStep::isnil_false(self.ty.clone());
            (last_step, False::new().into())
        } else {
            return Err(ValueMismatch::new(term_val.to_string(), "List".to_owned()).into());
        };
        let mut steps = term_res.congruence(&move |t| Self::new(t, self.ty.clone()).into());
        steps.push(step);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::IsNil.into(),
                        Symbol::sqbrack(Symbol::Type),
                        Symbol::paren(sym),
                    ]
                },
                "E-IsNil1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::IsNil.into(),
                    Symbol::sqbrack(Symbol::Type),
                    Symbol::paren(Keyword::Nil),
                ],
                Keyword::True,
                "E-IsNilNil",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::IsNil.into(),
                    Symbol::sqbrack(Symbol::Type),
                    Symbol::paren(vec![
                        Keyword::Cons.into(),
                        Symbol::sqbrack(Symbol::Type),
                        Symbol::paren(Symbol::comma_sep(
                            Symbol::sub(Symbol::Value, 1),
                            Symbol::sub(Symbol::Value, 2),
                        )),
                    ]),
                ],
                Keyword::False,
                "E-IsNilCons",
            ),
        ])
    }
}
