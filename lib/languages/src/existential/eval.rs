use super::{Existential, terms::Term, types::Type};
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
        App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True, Unit,
        Unpack, Variable,
    },
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = Existential;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(v) => v.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Fix(fix) => fix.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<Existential>::rules());
        rules.extend(Unit::<Existential>::rules());
        rules.extend(Lambda::<Existential>::rules());
        rules.extend(App::<Existential>::rules());
        rules.extend(Pack::<Existential>::rules());
        rules.extend(Unpack::<Existential>::rules());
        rules.extend(Num::<Existential>::rules());
        rules.extend(Succ::<Existential>::rules());
        rules.extend(Pred::<Existential>::rules());
        rules.extend(IsZero::<Existential>::rules());
        rules.extend(Record::<Existential>::rules());
        rules.extend(RecordProj::<Existential>::rules());
        rules.extend(True::<Existential>::rules());
        rules.extend(False::<Existential>::rules());
        rules.extend(If::<Existential>::rules());
        rules.extend(Fix::<Existential>::rules());
        rules
    }
}

impl Normalize for Type {
    type Lang = Existential;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
