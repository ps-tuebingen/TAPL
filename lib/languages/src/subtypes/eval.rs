use super::{Subtypes, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    terms::{
        App, Assign, Cast, Cons, Deref, False, Fix, If, Lambda, Let, Loc, Nil, Num, Pred, Record,
        RecordProj, Ref, Succ, True, Unit, Variable, Variant, VariantCase,
    },
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = Subtypes;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(unit) => unit.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::Variant(var) => var.eval(env),
            Term::VariantCase(case) => case.eval(env),
            Term::Cast(cast) => cast.eval(env),
            Term::Nil(nil) => nil.eval(env),
            Term::Cons(cons) => cons.eval(env),
            Term::ListCase(case) => case.eval(env),
            Term::Ref(rf) => rf.eval(env),
            Term::Deref(deref) => deref.eval(env),
            Term::Assign(assign) => assign.eval(env),
            Term::Loc(loc) => loc.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Let(lt) => lt.eval(env),
            Term::Fix(fix) => fix.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<Subtypes>::rules());
        rules.extend(Lambda::<Subtypes>::rules());
        rules.extend(App::<Subtypes>::rules());
        rules.extend(Unit::<Subtypes>::rules());
        rules.extend(Record::<Subtypes>::rules());
        rules.extend(RecordProj::<Subtypes>::rules());
        rules.extend(Variant::<Subtypes>::rules());
        rules.extend(VariantCase::<Subtypes>::rules());
        rules.extend(Cast::<Subtypes>::rules());
        rules.extend(Nil::<Subtypes>::rules());
        rules.extend(Cons::<Subtypes>::rules());
        rules.extend(Ref::<Subtypes>::rules());
        rules.extend(Deref::<Subtypes>::rules());
        rules.extend(Assign::<Subtypes>::rules());
        rules.extend(Loc::<Subtypes>::rules());
        rules.extend(Num::<Subtypes>::rules());
        rules.extend(Succ::<Subtypes>::rules());
        rules.extend(Pred::<Subtypes>::rules());
        rules.extend(True::<Subtypes>::rules());
        rules.extend(False::<Subtypes>::rules());
        rules.extend(If::<Subtypes>::rules());
        rules.extend(Let::<Subtypes>::rules());
        rules.extend(Fix::<Subtypes>::rules());
        rules
    }
}
