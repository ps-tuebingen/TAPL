use super::{Stlc, terms::Term, types::Type};
use check::Normalize;
use derivations::{Derivation, NormalizingDerivation};
use errors::eval_error::EvalError;
use eval::Eval;
use syntax::env::Environment;
use syntax::eval_context::EvalContext;
use trace::EvalTrace;

impl Eval for Term {
    type Lang = Stlc;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(v) => v.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(unit) => unit.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::Succ(s) => s.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Ascribe(asc) => asc.eval(env),
            Term::Let(lt) => lt.eval(env),
            Term::Pair(pr) => pr.eval(env),
            Term::Tuple(tup) => tup.eval(env),
            Term::Projection(proj) => proj.eval(env),
            Term::Fst(proj) => proj.eval(env),
            Term::Snd(proj) => proj.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::Left(lf) => lf.eval(env),
            Term::Right(rt) => rt.eval(env),
            Term::SumCase(case) => case.eval(env),
            Term::Variant(var) => var.eval(env),
            Term::VariantCase(case) => case.eval(env),
            Term::Nothing(not) => not.eval(env),
            Term::Something(some) => some.eval(env),
            Term::SomeCase(case) => case.eval(env),
            Term::Fix(fix) => fix.eval(env),
            Term::Nil(nil) => nil.eval(env),
            Term::Cons(cons) => cons.eval(env),
            Term::IsNil(isnil) => isnil.eval(env),
            Term::Head(hd) => hd.eval(env),
            Term::Tail(tl) => tl.eval(env),
        }
    }
}

impl Normalize for Type {
    type Lang = Stlc;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }
}
