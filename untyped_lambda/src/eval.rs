use super::terms::Term;

pub fn is_value(t: &Term) -> bool {
    match t {
        Term::Lambda(_, _) => true,
        _ => false,
    }
}

pub fn eval(t: Term) -> Term {
    let evaled = eval_once(t.clone());
    if t == evaled {
        evaled
    } else {
        eval(evaled)
    }
}

pub fn eval_once(t: Term) -> Term {
    match t {
        Term::App(t1, t2) => {
            if let Term::Lambda(v, t) = *t1 {
                if is_value(&t2) {
                    t.subst(&v, *t2)
                } else {
                    let t2_evaled = eval_once(*t2);
                    Term::App(Box::new(Term::Lambda(v, t)), Box::new(t2_evaled))
                }
            } else {
                let t1_evaled = eval_once(*t1);
                Term::App(Box::new(t1_evaled), t2)
            }
        }
        _ => t,
    }
}
