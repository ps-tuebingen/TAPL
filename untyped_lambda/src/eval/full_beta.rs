use super::is_value;
use crate::terms::Term;

pub fn eval_once_full_beta(t: Term) -> Term {
    match t {
        Term::App(t1, t2) => {
            if let Term::Lambda(v, t) = *t1 {
                t.subst(&v, *t2)
            } else if is_value(&t1) {
                let t2_evaled = eval_once_full_beta(*t2);
                Term::App(t1, Box::new(t2_evaled))
            } else {
                let t1_evaled = eval_once_full_beta(*t1);
                Term::App(Box::new(t1_evaled), t2)
            }
        }
        Term::Lambda(v, t) => Term::Lambda(v, Box::new(eval_once_full_beta(*t))),
        _ => t,
    }
}
