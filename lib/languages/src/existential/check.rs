use super::{Existential, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use syntax::env::Environment;

impl Typecheck for Term {
    type Lang = Existential;

    fn check(&self, env: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Term::Var(v) => v.check(env),
            Term::Unit(u) => u.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::IsZero(isz) => isz.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Fix(fix) => fix.check(env),
        }
    }
}

impl Subtypecheck for Type {
    type Lang = Existential;

    fn check_subtype(
        &self,
        _: &Self,
        _: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoSubtyping::new("Existential").into())
    }
}

impl Kindcheck for Type {
    type Lang = Existential;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Existential").into())
    }
}
