use super::{BoundedQuantification, terms::Term, types::Type};
use check::{Kindcheck, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::{NoKinding, check_error::CheckError};
use syntax::env::Environment;
impl Typecheck for Term {
    type Lang = BoundedQuantification;

    fn check(
        &self,
        env: Environment<BoundedQuantification>,
    ) -> Result<Derivation<BoundedQuantification>, CheckError> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::LambdaSub(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Record(rec) => rec.check(env),
            Term::Projection(proj) => proj.check(env),
        }
    }
}

impl Subtypecheck for Type {
    type Lang = BoundedQuantification;
    fn check_subtype(
        &self,
        sup: &Self,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        match self {
            Type::Var(var) => var.check_subtype(sup, env),
            Type::Top(t) => t.check_subtype(sup, env),
            Type::Nat(n) => n.check_subtype(sup, env),
            Type::Fun(f) => f.check_subtype(sup, env),
            Type::Forall(f) => f.check_subtype(sup, env),
            Type::Exists(e) => e.check_subtype(sup, env),
            Type::Record(rec) => rec.check_subtype(sup, env),
        }
    }
}

impl Kindcheck for Type {
    type Lang = BoundedQuantification;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Bounded Quantification").into())
    }
}
