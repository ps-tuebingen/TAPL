use super::terms::Term;
use common::Eval;
use std::convert::Infallible;

pub mod big;
pub mod cbn;
pub mod cbv;
pub mod full_beta;

use cbn::eval_once_cbn;
use cbv::eval_once_cbv;
use full_beta::eval_once_full_beta;

#[derive(Default)]
pub enum EvalOrder {
    #[default]
    CBV,
    CBN,
    FullBeta,
}

impl EvalOrder {
    fn get_eval_fun(&self) -> impl Fn(Term) -> Term {
        match self {
            EvalOrder::CBV => eval_once_cbv,
            EvalOrder::CBN => eval_once_cbn,
            EvalOrder::FullBeta => eval_once_full_beta,
        }
    }
}

pub fn is_value(t: &Term) -> bool {
    matches!(t, Term::Lambda(_, _))
}

impl Eval<'_> for Term {
    type Value = Term;
    type Err = Infallible;
    type Env = EvalOrder;

    fn eval(self, eo: EvalOrder) -> Result<Self::Value, Self::Err> {
        let eval_once = eo.get_eval_fun();
        let evaled = eval_once(self.clone());
        if self == evaled {
            Ok(evaled)
        } else {
            evaled.eval(eo)
        }
    }
}

#[cfg(test)]
mod eval_tests {
    use super::is_value;
    use crate::terms::Term;

    #[test]
    fn is_value_lam() {
        let result = is_value(&Term::Lambda(
            "x".to_owned(),
            Box::new(Term::Var("x".to_owned())),
        ));
        assert!(result)
    }

    #[test]
    fn is_value_var() {
        let result = is_value(&Term::Var("x".to_owned()));
        assert!(!result)
    }
}
