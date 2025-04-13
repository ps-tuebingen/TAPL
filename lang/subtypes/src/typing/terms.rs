use super::{to_check_err, TypingContext};
use crate::{syntax::Term, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::Var(v) => env.get_var(v).map_err(to_check_err),
            Term::Lambda(lambda) => lambda.check(env),
            Term::App(app) => app.check(env),
            Term::Unit(unit) => unit.check(env),
            Term::Cast(cast) => cast.check(env),
            Term::Record(rec) => rec.check(env),
            Term::Projection(proj) => proj.check(env),
            Term::Variant(var) => var.check(env),
            Term::VariantCase(case) => case.check(env),
            Term::Nil(nil) => nil.check(env),
            Term::Cons(cons) => cons.check(env),
            Term::ListCase(case) => case.check(env),
            Term::Ref(rf) => rf.check(env),
            Term::Deref(deref) => deref.check(env),
            Term::Assign(assign) => assign.check(env),
            Term::Loc(loc) => loc.check(env),
            Term::Zero(zero) => zero.check(env),
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
