use crate::syntax::{App, Error, Lambda, Raise, Term, Try, TryWithVal, Unit, Var};

pub trait Subst {
    fn subst(self, v: &Var, t: Term) -> Term;
}

impl Subst for Term {
    fn subst(self, v: &Var, t: Term) -> Term {
        match self {
            Term::Var(v1) => {
                if *v == v1 {
                    t
                } else {
                    Term::Var(v1)
                }
            }
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Unit(u) => u.subst(v, t),
            Term::Error(err) => err.subst(v, t),
            Term::Try(tr) => tr.subst(v, t),
            Term::Raise(r) => r.subst(v, t),
            Term::TryWithVal(tr) => tr.subst(v, t),
        }
    }
}

impl Subst for Lambda {
    fn subst(self, v: &Var, t: Term) -> Term {
        if self.var == *v {
            self.into()
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: Box::new((*self.body).subst(v, t)),
            }
            .into()
        }
    }
}
impl Subst for App {
    fn subst(self, v: &Var, t: Term) -> Term {
        App {
            fun: Box::new((*self.fun).subst(v, t.clone())),
            arg: Box::new((*self.arg).subst(v, t)),
        }
        .into()
    }
}

impl Subst for Error {
    fn subst(self, _: &Var, _: Term) -> Term {
        self.into()
    }
}

impl Subst for Try {
    fn subst(self, v: &Var, t: Term) -> Term {
        Try {
            term: Box::new((*self.term).subst(v, t.clone())),
            handler: Box::new((*self.handler).subst(v, t)),
        }
        .into()
    }
}

impl Subst for Raise {
    fn subst(self, v: &Var, t: Term) -> Term {
        Raise {
            exception: Box::new((*self.exception).subst(v, t)),
            ex_ty: self.ex_ty,
            cont_ty: self.cont_ty,
        }
        .into()
    }
}
impl Subst for TryWithVal {
    fn subst(self, v: &Var, t: Term) -> Term {
        TryWithVal {
            term: Box::new((*self.term).subst(v, t.clone())),
            handler: Box::new((*self.handler).subst(v, t)),
        }
        .into()
    }
}

impl Subst for Unit {
    fn subst(self, _: &Var, _: Term) -> Term {
        self.into()
    }
}
