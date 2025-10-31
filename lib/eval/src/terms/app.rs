use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext,
    language::Language,
    subst::SubstTerm,
    terms::{App, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for App<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang> + From<Lang::Value>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let fun_res = self.fun.clone().eval(env)?;
        let fun_val = fun_res.val();
        let lam = fun_val.clone().into_lambda()?;

        let arg_res = self.arg.clone().eval(env)?;
        let arg_val: Lang::Value = arg_res.val();

        let body_subst = lam.body.subst(&lam.var, &arg_val.clone().into());
        let next_step = EvalStep::app_abs(App::new(fun_val, arg_val), body_subst.clone());

        let mut steps =
            fun_res.congruence(&move |t| App::new(t, Rc::unwrap_or_clone(self.arg.clone())).into());
        steps.extend(
            arg_res.congruence(&move |t| App::new(Rc::unwrap_or_clone(self.fun.clone()), t).into()),
        );
        steps.push(next_step);

        let body_res = body_subst.eval(env)?;
        let body_val = body_res.val();
        steps.extend(body_res.steps);

        Ok(EvalTrace::<Lang>::new(steps, body_val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            // sym t3 -> sym' t3
            DerivationRule::eval_cong(
                |sym| vec![sym, SpecialChar::Space.into(), Symbol::sub(Symbol::Term, 3)],
                "E-App1",
            ),
            // val sym -> val sym'
            DerivationRule::eval_cong(
                |sym| vec![Symbol::Value, SpecialChar::Space.into(), sym],
                "E-App2",
            ),
            DerivationRule::eval(
                vec![
                    Symbol::paren(vec![
                        SpecialChar::Lambda.into(),
                        Symbol::Variable,
                        SpecialChar::Colon.into(),
                        Symbol::Type,
                        SpecialChar::Dot.into(),
                        Symbol::sub(Symbol::Term, 1),
                    ]),
                    SpecialChar::Space.into(),
                    Symbol::Value,
                ],
                vec![
                    Symbol::sub(Symbol::Term, 1),
                    Symbol::sqbrack(Symbol::mapto(Symbol::Variable, Symbol::Value)),
                ],
                "E-AppAbs",
            ),
        ])
    }
}
