use super::{BoundedQuantification, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    terms::{
        App, Lambda, LambdaSub, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
    },
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = BoundedQuantification;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::LambdaSub(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<BoundedQuantification>::rules());
        rules.extend(Num::<BoundedQuantification>::rules());
        rules.extend(Succ::<BoundedQuantification>::rules());
        rules.extend(Pred::<BoundedQuantification>::rules());
        rules.extend(Lambda::<BoundedQuantification>::rules());
        rules.extend(App::<BoundedQuantification>::rules());
        rules.extend(LambdaSub::<BoundedQuantification>::rules());
        rules.extend(TyApp::<BoundedQuantification>::rules());
        rules.extend(Pack::<BoundedQuantification>::rules());
        rules.extend(Unpack::<BoundedQuantification>::rules());
        rules.extend(Record::<BoundedQuantification>::rules());
        rules.extend(RecordProj::<BoundedQuantification>::rules());
        rules
    }
}
