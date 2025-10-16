use crate::Eval;
use errors::eval_error::EvalError;
use std::rc::Rc;

use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Assign, Term, Unit},
    values::{Unit as UnitVal, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Assign<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    UnitVal<Lang>: Into<Lang::Value>,
    Assign<Lang>: Into<Lang::Term>,
    Unit<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let lhs_res = self.lhs.clone().eval(env)?;
        let lhs_val = lhs_res.val();
        let lhs_t: Lang::Term = lhs_val.clone().into();
        let lhs_loc = lhs_val.into_loc()?;

        let rhs_res = self.rhs.clone().eval(env)?;
        let rhs_val = rhs_res.val();
        let rhs_t: Lang::Term = rhs_val.clone().into();

        let mut steps = lhs_res
            .congruence(&move |t| Assign::new(t, Rc::unwrap_or_clone(self.rhs.clone())).into());
        steps
            .extend(rhs_res.congruence(&move |t| {
                Assign::new(Rc::unwrap_or_clone(self.lhs.clone()), t).into()
            }));
        env.save_location(lhs_loc.loc, rhs_val.clone());

        steps.push(EvalStep::assign(lhs_t, rhs_t));
        Ok(EvalTrace::new(steps, UnitVal::<Lang>::new()))
    }
}
