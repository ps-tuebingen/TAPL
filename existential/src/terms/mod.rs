use std::fmt;

pub type Var = String;

pub mod bool;
pub mod lambda;
pub mod nat;
pub mod pack;
pub mod record;
pub use bool::{False, If, True};
pub use lambda::{App, Lambda};
pub use nat::{IsZero, Pred, Succ, Zero};
pub use pack::{Pack, Unpack};
pub use record::{Record, RecordProj};

#[derive(Debug, Clone)]
pub enum Term {
    Var(Var),
    Unit,
    Lambda(Lambda),
    App(App),
    Pack(Pack),
    Unpack(Unpack),
    Zero(Zero),
    Succ(Succ),
    Pred(Pred),
    IsZero(IsZero),
    Record(Record),
    RecordProj(RecordProj),
    True(True),
    False(False),
    If(If),
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
            Term::Unit => f.write_str("unit"),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Pack(pack) => pack.fmt(f),
            Term::Unpack(unpack) => unpack.fmt(f),
            Term::Zero(zero) => zero.fmt(f),
            Term::Succ(succ) => succ.fmt(f),
            Term::Pred(pred) => pred.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::If(ift) => ift.fmt(f),
        }
    }
}
