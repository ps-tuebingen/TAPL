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
    terms::{Raise, Term},
    values::Raise as RaiseVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Raise<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Raise<Lang>: Into<Lang::Term>,
    RaiseVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let exc_res = self.exception.eval(env)?;
        let exc_val = exc_res.val();
        let raise_val =
            RaiseVal::<Lang>::new(exc_val, self.cont_ty.clone(), self.exception_ty.clone());

        let steps = exc_res.congruence(&move |t| {
            Raise::new(t, self.cont_ty.clone(), self.exception_ty.clone()).into()
        });
        Ok(EvalTrace::new(steps, raise_val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Raise.into(),
                        Symbol::sqbrack(vec![
                            Symbol::sub(Symbol::Type, 1),
                            SpecialChar::Comma.into(),
                            Symbol::sub(Symbol::Type, 2),
                        ]),
                        Symbol::paren(sym),
                    ]
                },
                "E-Raise1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Raise.into(),
                    Symbol::sqbrack(vec![
                        Symbol::sub(Symbol::Type, 1),
                        SpecialChar::Comma.into(),
                        Symbol::sub(Symbol::Type, 2),
                    ]),
                    Symbol::paren(vec![
                        Keyword::Raise.into(),
                        Symbol::sqbrack(vec![
                            Symbol::sub(Symbol::Type, 1),
                            SpecialChar::Comma.into(),
                            Symbol::sub(Symbol::Type, 2),
                        ]),
                        Symbol::paren(Symbol::Value),
                    ]),
                ],
                vec![
                    Keyword::Raise.into(),
                    Symbol::sqbrack(vec![
                        Symbol::sub(Symbol::Type, 1),
                        SpecialChar::Comma.into(),
                        Symbol::sub(Symbol::Type, 2),
                    ]),
                    Symbol::paren(Symbol::Value),
                ],
                "E-RaiseRaise",
            ),
        ])
    }
}
