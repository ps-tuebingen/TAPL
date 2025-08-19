use super::{terms::Term, types::Type};
use check::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivations::Derivation;
use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind};

impl Typecheck for Term {
    type Term = Term;
    type Type = Type;

    fn check(
        &self,
        env: Environment<Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        match self {
            Term::Var(var) => var.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(unit) => unit.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Cast(cast) => cast.check(env),
            Term::Nil(nil) => nil.check(env),
            Term::Cons(cons) => cons.check(env),
            Term::ListCase(case) => case.check(env),
            Term::Ref(rf) => rf.check(env),
            Term::Deref(deref) => deref.check(env),
            Term::Assign(assign) => assign.check(env),
            Term::Loc(loc) => loc.check(env),
            Term::Num(num) => num.check(env),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::True(tru) => tru.check(env),
            Term::False(fls) => fls.check(env),
            Term::If(ift) => ift.check(env),
            Term::Let(lt) => lt.check(env),
            Term::Fix(fix) => fix.check(env),
        }
    }
}

impl Subtypecheck<Type> for Type {
    fn check_subtype(&self, sup: &Self, env: Environment<Type>) -> Result<(), CheckError> {
        match self {
            Type::Top(top) => top.check_subtype(sup, env),
            Type::Bot(bot) => bot.check_subtype(sup, env),
            Type::Fun(fun) => fun.check_subtype(sup, env),
            Type::Record(rec) => rec.check_subtype(sup, env),
            Type::Variant(variant) => variant.check_subtype(sup, env),
            Type::List(list) => list.check_subtype(sup, env),
            Type::Ref(refty) => refty.check_subtype(sup, env),
            Type::Source(src) => src.check_subtype(sup, env),
            Type::Sink(snk) => snk.check_subtype(sup, env),
            Type::Nat(nat) => nat.check_subtype(sup, env),
            Type::Unit(unit) => unit.check_subtype(sup, env),
            Type::Bool(b) => b.check_subtype(sup, env),
        }
    }
}

impl Kindcheck<Type> for Type {
    fn check_kind(&self, _: Environment<Type>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

impl Normalize<Type> for Type {
    fn normalize(self, _: Environment<Type>) -> Type {
        self
    }
}
