use super::{FOmega, terms::Term, types::Type};
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
        App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True,
        TyApp, TyLambda, Unit, Unpack, Variable,
    },
};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = FOmega;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::TyLambda(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::RecordProj(proj) => proj.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::Fix(fix) => fix.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(s) => s.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::IsZero(isz) => isz.eval(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<FOmega>::rules());
        rules.extend(Lambda::<FOmega>::rules());
        rules.extend(App::<FOmega>::rules());
        rules.extend(TyLambda::<FOmega>::rules());
        rules.extend(TyApp::<FOmega>::rules());
        rules.extend(Pack::<FOmega>::rules());
        rules.extend(Unpack::<FOmega>::rules());
        rules.extend(Record::<FOmega>::rules());
        rules.extend(RecordProj::<FOmega>::rules());
        rules.extend(True::<FOmega>::rules());
        rules.extend(False::<FOmega>::rules());
        rules.extend(If::<FOmega>::rules());
        rules.extend(Unit::<FOmega>::rules());
        rules.extend(Fix::<FOmega>::rules());
        rules.extend(Num::<FOmega>::rules());
        rules.extend(Succ::<FOmega>::rules());
        rules.extend(Pred::<FOmega>::rules());
        rules.extend(IsZero::<FOmega>::rules());
        rules
    }
}

impl Normalize for Type {
    type Lang = FOmega;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        match self {
            Type::Var(var) => var.normalize(env),
            Type::Fun(fun) => fun.normalize(env),
            Type::Forall(forall) => forall.normalize(env),
            Type::OpLambda(lam) => lam.normalize(env),
            Type::OpApp(app) => app.normalize(env),
            Type::Exists(ex) => ex.normalize(env),
            Type::Record(rec) => rec.normalize(env),
            Type::Bool(b) => b.normalize(env),
            Type::Unit(u) => u.normalize(env),
            Type::Nat(nat) => nat.normalize(env),
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        let mut rules = HashSet::new();
        rules.extend(Variable::<FOmega>::rules());
        rules.extend(Lambda::<FOmega>::rules());
        rules.extend(App::<FOmega>::rules());
        rules.extend(TyLambda::<FOmega>::rules());
        rules.extend(TyApp::<FOmega>::rules());
        rules.extend(Pack::<FOmega>::rules());
        rules.extend(Unpack::<FOmega>::rules());
        rules.extend(Record::<FOmega>::rules());
        rules.extend(RecordProj::<FOmega>::rules());
        rules.extend(True::<FOmega>::rules());
        rules.extend(False::<FOmega>::rules());
        rules.extend(If::<FOmega>::rules());
        rules.extend(Unit::<FOmega>::rules());
        rules.extend(Fix::<FOmega>::rules());
        rules.extend(Num::<FOmega>::rules());
        rules.extend(Succ::<FOmega>::rules());
        rules.extend(Pred::<FOmega>::rules());
        rules.extend(IsZero::<FOmega>::rules());
        rules
    }
}
