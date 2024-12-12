use super::nameless_terms::Term;

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

pub fn eval(t: Term) -> Term {
    let evaled = eval_once(t.clone());
    if evaled == t {
        evaled
    } else {
        eval(evaled)
    }
}
