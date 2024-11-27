use super::terms::Term;

pub mod cbn;
pub mod cbv;
pub mod full_beta;

use cbn::eval_once_cbn;
use cbv::eval_once_cbv;
use full_beta::eval_once_full_beta;

pub enum EvalOrder {
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
    match t {
        Term::Lambda(_, _) => true,
        _ => false,
    }
}

pub fn eval(t: Term, eo: EvalOrder) -> Term {
    let eval_once = eo.get_eval_fun();
    let evaled = eval_once(t.clone());
    if t == evaled {
        evaled
    } else {
        eval(evaled, eo)
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
