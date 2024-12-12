use crate::terms::Term;

pub fn eval_big(t: Term) -> Term {
    match t {
        Term::Var(_) => t,
        Term::Lambda(_, _) => t,
        Term::App(t1, t2) => {
            let t1_evaled = eval_big(*t1);
            let t2_evaled = eval_big(*t2);
            if let Term::Lambda(v, t) = t1_evaled {
                eval_big(t.subst(&v, t2_evaled))
            } else {
                Term::App(Box::new(t1_evaled), Box::new(t2_evaled))
            }
        }
    }
}
