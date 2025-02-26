use super::is_value;
use crate::terms::Term;

pub fn eval_once_cbv(t: Term) -> Term {
    match t {
        Term::App(t1, t2) => {
            if let Term::Lambda(v, t) = *t1 {
                if is_value(&t2) {
                    t.subst(&v, *t2)
                } else {
                    let t2_evaled = eval_once_cbv(*t2);
                    Term::App(Box::new(Term::Lambda(v, t)), Box::new(t2_evaled))
                }
            } else {
                let t1_evaled = eval_once_cbv(*t1);
                Term::App(Box::new(t1_evaled), t2)
            }
        }
        _ => t,
    }
}
