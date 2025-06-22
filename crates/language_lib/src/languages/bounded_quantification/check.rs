use super::{terms::Term, types::Type};
use check::{errors::CheckError, Kindcheck, Subtypecheck, Typecheck};
use derivation::Derivation;
use syntax::{env::Environment, kinds::Kind};
impl Typecheck for Term {
    type Term = Term;
    type Type = Type;

    fn check(
        &self,
        env: Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Type>> {
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

impl Subtypecheck<Type> for Type {
    fn check_subtype(&self, sup: &Self, env: Environment<Type>) -> Result<(), CheckError<Type>> {
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

impl Kindcheck<Type> for Type {
    fn check_kind(&self, _: Environment<Type>) -> Result<Kind, CheckError<Type>> {
        Ok(Kind::Star)
    }
}
