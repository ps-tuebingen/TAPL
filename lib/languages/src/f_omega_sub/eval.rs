use super::{FOmegaSub, terms::Term, types::Type};
use check::Normalize;
use derivations::Derivation;
use errors::eval_error::EvalError;
use eval::Eval;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment,
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

impl Normalize for Type {
    type Lang = FOmegaSub;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        match self {
            Type::Var(var) => var.normalize(env),
            Type::Top(top) => top.normalize(env),
            Type::Fun(fun) => fun.normalize(env),
            Type::Forall(forall) => forall.normalize(env),
            Type::OpLambdaSub(lam) => lam.normalize(env),
            Type::OpApp(app) => app.normalize(env),
            Type::Exists(ex) => ex.normalize(env),
            Type::Record(rec) => rec.normalize(env),
            Type::Nat(nat) => nat.normalize(env),
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
