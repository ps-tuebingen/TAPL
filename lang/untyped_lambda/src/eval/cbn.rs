use crate::terms::Term;

pub fn eval_once_cbn(t: Term) -> Term {
    match t {
        Term::App(t1, t2) => {
            if let Term::Lambda(v, t) = *t1 {
                t.subst(&v, *t2)
            } else {
                let t1_evaled = eval_once_cbn(*t1);
                Term::App(Box::new(t1_evaled), t2)
            }
        }
        _ => t,
    }
}
