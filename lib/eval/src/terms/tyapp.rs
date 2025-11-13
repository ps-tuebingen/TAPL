use crate::Eval;
use errors::{ValueMismatch, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext, language::Language, subst::SubstType, terms::TyApp,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for TyApp<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let fun_res = self.fun.eval(env)?;
        let fun_val = fun_res.val();
        let (res_steps, res_val) = if let Ok(tylam) = fun_val.clone().into_tylambda() {
            let term_subst = tylam.term.subst_type(&tylam.var, &self.arg);
            let next_step =
                EvalStep::tyappabs(Self::new(fun_val, self.arg.clone()), term_subst.clone());
            let term_res = term_subst.eval(env)?;
            let term_val = term_res.val();
            let mut steps = term_res.steps;
            steps.push(next_step);
            (steps, term_val)
        } else if let Ok(lamsub) = fun_val.clone().into_lambdasub() {
            let term_subst = lamsub.term.subst_type(&lamsub.var, &self.arg);
            let next_step =
                EvalStep::tyappabs_sub(Self::new(fun_val, self.arg.clone()), term_subst.clone());
            let term_res = term_subst.eval(env)?;
            let term_val = term_res.val();
            let mut steps = term_res.steps;
            steps.push(next_step);
            (steps, term_val)
        } else {
            return Err(
                ValueMismatch::new(fun_val.to_string(), "LambdaSub Term".to_owned()).into(),
            );
        };

        let mut steps = fun_res.congruence(&move |t| Self::new(t, self.arg.clone()).into());
        steps.extend(res_steps);
        Ok(EvalTrace::<Lang>::new(steps, res_val))
    }

    fn rules() -> HashSet<DerivationRule> {
        let features = Lang::features();
        let ty_var = if features.subtyped {
            Symbol::less_colon_sep(Symbol::Typevariable, Symbol::sub(Symbol::Type, 1))
        } else {
            Symbol::double_colon_sep(Symbol::Typevariable, Symbol::Kind)
        };

        HashSet::from([
            DerivationRule::eval_cong(|sym| vec![sym, Symbol::sqbrack(Symbol::Type)], "E-TyApp1"),
            DerivationRule::eval(
                vec![
                    SpecialChar::Lambda.into(),
                    Symbol::Typevariable,
                    ty_var,
                    SpecialChar::Dot.into(),
                    Symbol::Term,
                    Symbol::sqbrack(Symbol::Type),
                ],
                vec![
                    Symbol::Term,
                    Symbol::sqbrack(Symbol::mapto(Symbol::Typevariable, Symbol::Type)),
                ],
                "E-TyAppAbs",
            ),
        ])
    }
}
