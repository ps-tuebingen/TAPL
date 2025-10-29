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
    terms::{Exception, Term},
    values::Exception as ExceptionVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Exception<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    ExceptionVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::<Lang>::new(
            vec![],
            ExceptionVal::new(self.ty).into(),
        ))
    }
    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval(
                vec![
                    Keyword::Err.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                    SpecialChar::Space.into(),
                    Symbol::sub(Symbol::Term, 3),
                ],
                vec![
                    Keyword::Err.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                ],
                "E-Err1",
            ),
            DerivationRule::eval(
                vec![
                    Symbol::sub(Symbol::Term, 3),
                    SpecialChar::Space.into(),
                    Keyword::Err.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                ],
                vec![
                    Keyword::Err.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                ],
                "E-Err2",
            ),
        ])
    }
}
