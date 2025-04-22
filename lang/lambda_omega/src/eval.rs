use super::{terms::Term, types::Type, values::Value};
use common::{
    errors::Error,
    eval::{Eval, Normalize},
};

impl Eval for Term {
    type Env = ();
    type Value = Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Num(num) => num.eval(env),
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::TyLambda(tylam) => tylam.eval(env),
            Term::TyApp(tyapp) => tyapp.eval(env),
        }
    }
}

impl Normalize<Type> for Type {
    fn normalize(self) -> Type {
        self
    }
}
