use super::{Stlc, terms::Term, types::Type};
use check::Normalize;
use derivations::{Derivation, NormalizingDerivation};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
    eval_context::EvalContext,
    terms::{
        App, Ascribe, Cons, False, Fix, Fst, Head, If, IsNil, IsZero, Lambda, Left, Let, Nil,
        Nothing, Num, Pair, Pred, Projection, Record, RecordProj, Right, Snd, SomeCase, Something,
        Succ, SumCase, Tail, True, Tuple, Unit, Variable, Variant, VariantCase,
    },
};
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

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<Stlc>::rules());
        rules.extend(Lambda::<Stlc>::rules());
        rules.extend(App::<Stlc>::rules());
        rules.extend(Unit::<Stlc>::rules());
        rules.extend(True::<Stlc>::rules());
        rules.extend(False::<Stlc>::rules());
        rules.extend(If::<Stlc>::rules());
        rules.extend(Num::<Stlc>::rules());
        rules.extend(Pred::<Stlc>::rules());
        rules.extend(Succ::<Stlc>::rules());
        rules.extend(IsZero::<Stlc>::rules());
        rules.extend(Ascribe::<Stlc>::rules());
        rules.extend(Let::<Stlc>::rules());
        rules.extend(Pair::<Stlc>::rules());
        rules.extend(Tuple::<Stlc>::rules());
        rules.extend(Projection::<Stlc>::rules());
        rules.extend(Fst::<Stlc>::rules());
        rules.extend(Snd::<Stlc>::rules());
        rules.extend(Record::<Stlc>::rules());
        rules.extend(RecordProj::<Stlc>::rules());
        rules.extend(Left::<Stlc>::rules());
        rules.extend(Right::<Stlc>::rules());
        rules.extend(SumCase::<Stlc>::rules());
        rules.extend(Variant::<Stlc>::rules());
        rules.extend(VariantCase::<Stlc>::rules());
        rules.extend(Nothing::<Stlc>::rules());
        rules.extend(Something::<Stlc>::rules());
        rules.extend(SomeCase::<Stlc>::rules());
        rules.extend(Fix::<Stlc>::rules());
        rules.extend(Nil::<Stlc>::rules());
        rules.extend(Cons::<Stlc>::rules());
        rules.extend(IsNil::<Stlc>::rules());
        rules.extend(Head::<Stlc>::rules());
        rules.extend(Tail::<Stlc>::rules());
        rules
    }
}

impl Normalize for Type {
    type Lang = Stlc;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
