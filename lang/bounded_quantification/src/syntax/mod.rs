use std::fmt;
pub type Var = String;
pub type Label = String;

pub mod lambda;
pub mod lambda_sub;
pub mod nat;
pub mod pack;
pub mod record;
pub use lambda::{App, Lambda};
pub use lambda_sub::{LambdaSub, TyApp};
pub use nat::{Const, Pred, Succ};
pub use pack::{Pack, Unpack};
pub use record::{Projection, Record};

#[derive(Clone, Debug)]
pub enum Term {
    Var(Var),
    Const(Const),
    Succ(Succ),
    Pred(Pred),
    Lambda(Lambda),
    App(App),
    LambdaSub(LambdaSub),
    TyApp(TyApp),
    Pack(Pack),
    Unpack(Unpack),
    Record(Record),
    Projection(Projection),
}

impl From<&str> for Term {
    fn from(s: &str) -> Term {
        Term::Var(s.to_owned())
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => f.write_str(v),
            Term::Const(c) => c.fmt(f),
            Term::Succ(s) => s.fmt(f),
            Term::Pred(p) => p.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::LambdaSub(lam) => lam.fmt(f),
            Term::TyApp(app) => app.fmt(f),
            Term::Pack(pack) => pack.fmt(f),
            Term::Unpack(unpack) => unpack.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::Projection(proj) => proj.fmt(f),
        }
    }
}
