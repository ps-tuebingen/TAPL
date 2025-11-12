use super::{Recursive, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    terms::{
        App, False, Fix, Fold, Fst, If, IsZero, Lambda, Let, Num, Pair, Pred, Record, RecordProj,
        Snd, Succ, True, Unfold, Unit, Variable, Variant, VariantCase,
    },
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = Recursive;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(v) => v.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::Fold(fold) => fold.eval(env),
            Term::Unfold(unfold) => unfold.eval(env),
            Term::Variant(var) => var.eval(env),
            Term::VariantCase(case) => case.eval(env),
            Term::Pair(p) => p.eval(env),
            Term::Fst(fst) => fst.eval(env),
            Term::Snd(snd) => snd.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Fix(fix) => fix.eval(env),
            Term::Let(lt) => lt.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<Recursive>::rules());
        rules.extend(Lambda::<Recursive>::rules());
        rules.extend(App::<Recursive>::rules());
        rules.extend(Unit::<Recursive>::rules());
        rules.extend(Fold::<Recursive>::rules());
        rules.extend(Unfold::<Recursive>::rules());
        rules.extend(Variant::<Recursive>::rules());
        rules.extend(VariantCase::<Recursive>::rules());
        rules.extend(Pair::<Recursive>::rules());
        rules.extend(Fst::<Recursive>::rules());
        rules.extend(Snd::<Recursive>::rules());
        rules.extend(Num::<Recursive>::rules());
        rules.extend(Succ::<Recursive>::rules());
        rules.extend(Pred::<Recursive>::rules());
        rules.extend(IsZero::<Recursive>::rules());
        rules.extend(True::<Recursive>::rules());
        rules.extend(False::<Recursive>::rules());
        rules.extend(If::<Recursive>::rules());
        rules.extend(Fix::<Recursive>::rules());
        rules.extend(Let::<Recursive>::rules());
        rules.extend(Record::<Recursive>::rules());
        rules.extend(RecordProj::<Recursive>::rules());
        rules
    }
}
