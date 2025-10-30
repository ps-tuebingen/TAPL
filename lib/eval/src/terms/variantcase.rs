use crate::Eval;
use errors::{UndefinedLabel, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext, language::Language, subst::SubstTerm, terms::VariantCase,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for VariantCase<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang> + From<Lang::Value>,
    VariantCase<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let var_val = bound_val.clone().into_variant()?;
        let matching = self
            .patterns
            .clone()
            .into_iter()
            .find(|pt| *pt.label == var_val.label)
            .ok_or(UndefinedLabel::new(&var_val.label))?;
        let rhs_subst = matching
            .rhs
            .subst(&matching.bound_var, &((*var_val.val).into()));
        let next_step = EvalStep::variantcase(
            VariantCase::new(bound_val, self.patterns.clone()),
            Rc::unwrap_or_clone(rhs_subst.clone()),
        );
        let rhs_res = rhs_subst.eval(env)?;
        let val = rhs_res.val();

        let mut steps =
            bound_res.congruence(&move |t| VariantCase::new(t, self.patterns.clone()).into());
        steps.push(next_step);
        steps.extend(rhs_res.steps);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Case.into(),
                        sym,
                        Keyword::Of.into(),
                        Symbol::brack(Symbol::many(vec![
                            Symbol::angbrack(vec![
                                Symbol::sub(Symbol::Label, "i"),
                                SpecialChar::Equals.into(),
                                Symbol::sub(Symbol::Variable, "i"),
                            ]),
                            SpecialChar::DoubleArrow.into(),
                            Symbol::sub(Symbol::Term, "i"),
                        ])),
                    ]
                },
                "E-VariantCase1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Case.into(),
                    Symbol::angbrack(vec![
                        Symbol::sub(Symbol::Label, "k"),
                        SpecialChar::Equals.into(),
                        Symbol::Value,
                    ]),
                    Keyword::Of.into(),
                    Symbol::brack(Symbol::many(vec![
                        Symbol::angbrack(vec![
                            Symbol::sub(Symbol::Label, "i"),
                            SpecialChar::Equals.into(),
                            Symbol::sub(Symbol::Variable, "i"),
                        ]),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, "i"),
                    ])),
                ],
                vec![
                    Symbol::sub(Symbol::Term, "i"),
                    Symbol::sqbrack(vec![
                        Symbol::sub(Symbol::Variable, "i"),
                        SpecialChar::Arrow.into(),
                        Symbol::Value,
                    ]),
                ],
                "E-VariantCase",
            ),
        ])
    }
}
