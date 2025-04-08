use super::nameless_terms::Term;
use common::Eval;
use std::convert::Infallible;

impl Eval<'_> for Term {
    type Value = Term;
    type Err = Infallible;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        let evaled = eval_once(self.clone());
        if evaled == self {
            Ok(evaled)
        } else {
            evaled.eval(_env)
        }
    }
}

pub fn eval_once(t: Term) -> Term {
    match t {
        Term::Var(_) => t,
        Term::Lambda(_) => t,
        Term::App(t1, t2) => {
            if let Term::Lambda(body) = *t1 {
                ((*body).subst(0, *t2)).shift(-1, 0)
            } else {
                let t1_evaled = eval_once(*t1);
                Term::app(t1_evaled, *t2)
            }
        }
    }
}
