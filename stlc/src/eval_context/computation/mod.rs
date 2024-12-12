pub mod appabs;
pub mod fixbeta;
pub mod headlist;
pub mod ifbool;
pub mod isnillist;
pub mod iszeronum;
pub mod letsubst;
pub mod predsucc;
pub mod proj1beta;
pub mod proj2beta;
pub mod projbeta;
pub mod recordprojbeta;
pub mod somecaserhs;
pub mod succpred;
pub mod sumcaserhs;
pub mod taillist;
pub mod variantcaserhs;

use super::{errors::Error, Eval, EvalContext, Value};
pub use {
    appabs::AppAbs, fixbeta::FixBeta, headlist::HeadList, ifbool::IfBool, isnillist::IsNilList,
    iszeronum::IsZeroNum, letsubst::LetSubst, predsucc::PredSucc, proj1beta::Proj1Beta,
    proj2beta::Proj2Beta, projbeta::ProjBeta, recordprojbeta::RecordProjBeta,
    somecaserhs::SomeCaseRhs, succpred::SuccPred, sumcaserhs::SumCaseRhs, taillist::TailList,
    variantcaserhs::VariantCaseRhs,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ComputationRule {
    AppAbs(AppAbs),
    IfBool(IfBool),
    PredSucc(PredSucc),
    SuccPred(SuccPred),
    IsZeroNum(IsZeroNum),
    LetSubst(LetSubst),
    Proj1Beta(Proj1Beta),
    Proj2Beta(Proj2Beta),
    SumCaseRhs(SumCaseRhs),
    SomeCaseRhs(SomeCaseRhs),
    VariantCaseRhs(VariantCaseRhs),
    ProjBeta(ProjBeta),
    RecordProjBeta(RecordProjBeta),
    FixBeta(FixBeta),
    IsNilList(IsNilList),
    HeadList(HeadList),
    TailList(TailList),
}

impl Eval for ComputationRule {
    fn eval(self) -> Result<Value, Error> {
        match self {
            ComputationRule::AppAbs(app) => app.eval(),
            ComputationRule::IfBool(ifb) => ifb.eval(),
            ComputationRule::PredSucc(pd) => pd.eval(),
            ComputationRule::SuccPred(sp) => sp.eval(),
            ComputationRule::IsZeroNum(isz) => isz.eval(),
            ComputationRule::LetSubst(lt) => lt.eval(),
            ComputationRule::Proj1Beta(proj) => proj.eval(),
            ComputationRule::Proj2Beta(proj) => proj.eval(),
            ComputationRule::SumCaseRhs(case) => case.eval(),
            ComputationRule::SomeCaseRhs(case) => case.eval(),
            ComputationRule::VariantCaseRhs(case) => case.eval(),
            ComputationRule::ProjBeta(proj) => proj.eval(),
            ComputationRule::RecordProjBeta(proj) => proj.eval(),
            ComputationRule::FixBeta(fix) => fix.eval(),
            ComputationRule::IsNilList(isnil) => isnil.eval(),
            ComputationRule::HeadList(hd) => hd.eval(),
            ComputationRule::TailList(tl) => tl.eval(),
        }
    }
}
impl From<ComputationRule> for EvalContext {
    fn from(rule: ComputationRule) -> EvalContext {
        EvalContext::Computation(rule)
    }
}
