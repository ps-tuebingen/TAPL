use super::SubstTerm;
use crate::terms::{Fix, Term, Var};

impl SubstTerm for Fix {
    fn subst(self, v: &Var, t: Term) -> Term {
        Fix {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}
