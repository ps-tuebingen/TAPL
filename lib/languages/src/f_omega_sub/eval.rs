use super::{FOmegaSub, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    terms::{
        App, Lambda, LambdaSub, Let, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack,
        Variable,
    },
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = FOmegaSub;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::LambdaSub(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::Let(lt) => lt.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<FOmegaSub>::rules());
        rules.extend(Lambda::<FOmegaSub>::rules());
        rules.extend(App::<FOmegaSub>::rules());
        rules.extend(LambdaSub::<FOmegaSub>::rules());
        rules.extend(TyApp::<FOmegaSub>::rules());
        rules.extend(Pack::<FOmegaSub>::rules());
        rules.extend(Unpack::<FOmegaSub>::rules());
        rules.extend(Record::<FOmegaSub>::rules());
        rules.extend(RecordProj::<FOmegaSub>::rules());
        rules.extend(Num::<FOmegaSub>::rules());
        rules.extend(Succ::<FOmegaSub>::rules());
        rules.extend(Pred::<FOmegaSub>::rules());
        rules.extend(Let::<FOmegaSub>::rules());
        rules
    }
}
