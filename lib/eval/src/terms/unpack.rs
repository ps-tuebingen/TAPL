use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};

use syntax::{
    eval_context::EvalContext,
    language::Language,
    subst::{SubstTerm, SubstType},
    terms::{Term, Unpack},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Unpack<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang> + From<Lang::Value>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.bound_term.eval(env)?;
        let term_val = term_res.val();
        let pack_val = term_val.clone().into_pack()?;
        let in_subst = self
            .in_term
            .clone()
            .subst_type(&self.ty_name, &pack_val.inner_ty)
            .subst(&self.term_name, &((*pack_val.val).into()));
        let next_step = EvalStep::unpackpack(
            Self::new(
                &self.ty_name,
                &self.term_name,
                term_val,
                Rc::unwrap_or_clone(self.in_term.clone()),
            ),
            Rc::unwrap_or_clone(in_subst.clone()),
        );
        let in_res = in_subst.eval(env)?;
        let val = in_res.val();

        let mut steps = term_res.congruence(&move |t| {
            Self::new(
                &self.ty_name,
                &self.term_name,
                t,
                Rc::unwrap_or_clone(self.in_term.clone()),
            )
            .into()
        });
        steps.push(next_step);
        steps.extend(in_res.steps);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Let.into(),
                        Symbol::brack(Symbol::comma_sep(Symbol::Typevariable, Symbol::Variable)),
                        SpecialChar::Equals.into(),
                        sym,
                        Keyword::In.into(),
                        Symbol::sub(Symbol::Term, 3),
                    ]
                },
                "E-Unpack1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Let.into(),
                    Symbol::paren(Symbol::comma_sep(Symbol::Typevariable, Symbol::Variable)),
                    SpecialChar::Equals.into(),
                    Symbol::paren(vec![
                        Symbol::brack(Symbol::comma_sep(
                            vec![SpecialChar::Star.into(), Symbol::sub(Symbol::Type, 1)],
                            Symbol::Value,
                        )),
                        Keyword::As.into(),
                        Symbol::sub(Symbol::Type, 2),
                    ]),
                    Keyword::In.into(),
                    Symbol::Term,
                ],
                vec![
                    Symbol::Term,
                    Symbol::sqbrack(Symbol::comma_sep(
                        Symbol::mapto(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1)),
                        Symbol::mapto(Symbol::Variable, Symbol::Value),
                    )),
                ],
                "E-UnpackPack",
            ),
        ])
    }
}
