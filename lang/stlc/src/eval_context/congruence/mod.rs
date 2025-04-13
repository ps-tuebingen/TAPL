use super::{Eval, EvalContext, Value};
use common::errors::Error;
pub use {
    app1::App1, app2::App2, ascribe::Ascribe, cons1::Cons1, cons2::Cons2, fix::Fix, head::Head,
    ift::If, isnil::IsNil, iszero::IsZero, left::Left, let_exp::Let, pair1::Pair1, pair2::Pair2,
    pred::Pred, proj::Proj, proj1::Proj1, proj2::Proj2, record::Record, recordproj::RecordProj,
    right::Right, somecase::SomeCase, something::Something, succ::Succ, sumcase::SumCase,
    tail::Tail, tup::Tup, variant::Variant, variantcase::VariantCase,
};

pub mod app1;
pub mod app2;
pub mod ascribe;
pub mod cons1;
pub mod cons2;
pub mod fix;
pub mod head;
pub mod ift;
pub mod isnil;
pub mod iszero;
pub mod left;
pub mod let_exp;
pub mod pair1;
pub mod pair2;
pub mod pred;
pub mod proj;
pub mod proj1;
pub mod proj2;
pub mod record;
pub mod recordproj;
pub mod right;
pub mod somecase;
pub mod something;
pub mod succ;
pub mod sumcase;
pub mod tail;
pub mod tup;
pub mod variant;
pub mod variantcase;

#[derive(Debug, PartialEq, Eq)]
pub enum CongruenceRule {
    App1(App1),
    App2(App2),
    If(If),
    Pred(Pred),
    Succ(Succ),
    IsZero(IsZero),
    Ascribe(Ascribe),
    Let(Let),
    Pair1(Pair1),
    Pair2(Pair2),
    Proj1(Proj1),
    Proj2(Proj2),
    RecordProj(RecordProj),
    Left(Left),
    Right(Right),
    SumCase(SumCase),
    SomeCase(SomeCase),
    Tup(Tup),
    Record(Record),
    Variant(Variant),
    VariantCase(VariantCase),
    Proj(Proj),
    Something(Something),
    Fix(Fix),
    Cons1(Cons1),
    Cons2(Cons2),
    IsNil(IsNil),
    Head(Head),
    Tail(Tail),
}

impl Eval for CongruenceRule {
    fn eval(self) -> Result<Value, Error> {
        match self {
            CongruenceRule::App1(app1) => app1.eval(),
            CongruenceRule::App2(app2) => app2.eval(),
            CongruenceRule::If(ift) => ift.eval(),
            CongruenceRule::Pred(pred) => pred.eval(),
            CongruenceRule::Succ(succ) => succ.eval(),
            CongruenceRule::IsZero(isz) => isz.eval(),
            CongruenceRule::Ascribe(asc) => asc.eval(),
            CongruenceRule::Let(lt) => lt.eval(),
            CongruenceRule::Pair1(pair1) => pair1.eval(),
            CongruenceRule::Pair2(pair2) => pair2.eval(),
            CongruenceRule::Proj1(proj1) => proj1.eval(),
            CongruenceRule::Proj2(proj2) => proj2.eval(),
            CongruenceRule::RecordProj(recproj) => recproj.eval(),
            CongruenceRule::Left(left) => left.eval(),
            CongruenceRule::Right(right) => right.eval(),
            CongruenceRule::SumCase(case) => case.eval(),
            CongruenceRule::SomeCase(case) => case.eval(),
            CongruenceRule::Tup(tup) => tup.eval(),
            CongruenceRule::Record(rec) => rec.eval(),
            CongruenceRule::Variant(var) => var.eval(),
            CongruenceRule::VariantCase(case) => case.eval(),
            CongruenceRule::Proj(proj) => proj.eval(),
            CongruenceRule::Something(some) => some.eval(),
            CongruenceRule::Fix(fix) => fix.eval(),
            CongruenceRule::Cons1(cons1) => cons1.eval(),
            CongruenceRule::Cons2(cons2) => cons2.eval(),
            CongruenceRule::IsNil(isnil) => isnil.eval(),
            CongruenceRule::Head(head) => head.eval(),
            CongruenceRule::Tail(tail) => tail.eval(),
        }
    }
}

impl From<CongruenceRule> for EvalContext {
    fn from(rule: CongruenceRule) -> EvalContext {
        EvalContext::Congruence(rule)
    }
}
