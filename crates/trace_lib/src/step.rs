use super::rules::EvaluationRule;
use syntax::{
    terms::{Assign, Cast, Deref, False, IsNil, Loc, Num, Pair, Pred, Succ, Term, True, Unit},
    types::Type,
    values::Value,
    Location,
};

pub struct EvalStep<T>
where
    T: Term,
{
    pub source: T,
    pub rule: EvaluationRule,
    pub target: T,
}

impl<T> EvalStep<T>
where
    T: Term,
{
    pub fn congruence(self, into_fun: &impl Fn(T) -> T) -> EvalStep<T> {
        EvalStep {
            source: into_fun(self.source),
            rule: self.rule,
            target: into_fun(self.target),
        }
    }

    pub fn app_abs<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::AppAbs,
            target: target.into(),
        }
    }

    pub fn ascribe<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::Ascribe,
            target: target.into(),
        }
    }

    pub fn assign<V1, V2>(lhs: V1, rhs: V2) -> EvalStep<T>
    where
        V1: Into<T>,
        V2: Into<T>,
        Unit<T>: Into<T>,
        Assign<T>: Into<T>,
    {
        EvalStep {
            source: Assign::new(lhs, rhs).into(),
            rule: EvaluationRule::Assign,
            target: Unit::new().into(),
        }
    }

    pub fn cast<Ty, V>(ty: Ty, val: V) -> EvalStep<T>
    where
        Ty: Type,
        V: Value + Into<T>,
        Cast<T, Ty>: Into<T>,
    {
        EvalStep {
            source: Cast::new(val.clone(), ty).into(),
            rule: EvaluationRule::Cast,
            target: val.into(),
        }
    }

    pub fn deref<V1, V2>(loc_val: V1, env_val: V2) -> EvalStep<T>
    where
        V2: Into<T>,
        V1: Into<T>,
        Deref<T>: Into<T>,
    {
        EvalStep {
            source: Deref::new(loc_val).into(),
            rule: EvaluationRule::Deref,
            target: env_val.into(),
        }
    }

    pub fn fold<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::Fold,
            target: target.into(),
        }
    }

    pub fn fst<T1, T2>(f: T1, s: T2) -> EvalStep<T>
    where
        T1: Into<T> + Clone,
        T2: Into<T>,
        Pair<T>: Into<T>,
    {
        EvalStep {
            source: Pair::new(f.clone(), s).into(),
            rule: EvaluationRule::Fst,
            target: f.into(),
        }
    }

    pub fn head<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::Head,
            target: target.into(),
        }
    }

    pub fn if_true<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::IfTrue,
            target: target.into(),
        }
    }

    pub fn if_false<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::IfFalse,
            target: target.into(),
        }
    }

    pub fn isnil_true<Ty>(ty: Ty) -> EvalStep<T>
    where
        Ty: Type,
        True<T>: Into<T>,
        IsNil<T, Ty>: Into<T>,
    {
        EvalStep {
            source: IsNil::new(True::new(), ty).into(),
            rule: EvaluationRule::IsNilTrue,
            target: True::new().into(),
        }
    }

    pub fn isnil_false<Ty>(ty: Ty) -> EvalStep<T>
    where
        Ty: Type,
        IsNil<T, Ty>: Into<T>,
        False<T>: Into<T>,
    {
        EvalStep {
            source: IsNil::new(False::new(), ty).into(),
            rule: EvaluationRule::IsNilFalse,
            target: False::new().into(),
        }
    }

    pub fn iszero_true<T1>(source: T1) -> EvalStep<T>
    where
        T1: Into<T>,
        True<T>: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::IsZeroTrue,
            target: True::new().into(),
        }
    }

    pub fn iszero_false<T1>(source: T1) -> EvalStep<T>
    where
        T1: Into<T>,
        False<T>: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::IsZeroFalse,
            target: False::new().into(),
        }
    }

    pub fn lett<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::LetSubst,
            target: target.into(),
        }
    }

    pub fn listcase_nil<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::ListCaseNil,
            target: target.into(),
        }
    }

    pub fn listcase_cons<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::ListCaseCons,
            target: target.into(),
        }
    }

    pub fn pred(num: i64) -> EvalStep<T>
    where
        Pred<T>: Into<T>,
        Num<T>: Into<T>,
    {
        EvalStep {
            source: Pred::new(Num::new(num)).into(),
            rule: EvaluationRule::Pred,
            target: Num::new(num - 1).into(),
        }
    }

    pub fn projection<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::Projection,
            target: target.into(),
        }
    }

    pub fn recordproj<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::RecordProj,
            target: target.into(),
        }
    }

    pub fn reft<T1>(source: T1, loc: Location) -> EvalStep<T>
    where
        Loc<T>: Into<T>,
        T1: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::Ref,
            target: Loc::new(loc).into(),
        }
    }

    pub fn snd<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::Snd,
            target: target.into(),
        }
    }

    pub fn somecase_some<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::SomecaseSome,
            target: target.into(),
        }
    }

    pub fn somecase_none<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::SomecaseNone,
            target: target.into(),
        }
    }

    pub fn succ(num: i64) -> EvalStep<T>
    where
        Succ<T>: Into<T>,
        Num<T>: Into<T>,
    {
        EvalStep {
            source: Succ::new(Num::new(num)).into(),
            rule: EvaluationRule::Succ,
            target: Num::new(num).into(),
        }
    }

    pub fn sumcase_left<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::SumcaseLeft,
            target: target.into(),
        }
    }

    pub fn sumcase_right<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::SumcaseRight,
            target: target.into(),
        }
    }

    pub fn tail<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::Tail,
            target: target.into(),
        }
    }

    pub fn try_catch<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::TryCatch,
            target: target.into(),
        }
    }

    pub fn try_succ<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::TrySucc,
            target: target.into(),
        }
    }

    pub fn tryval_catch<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::TryValCatch,
            target: target.into(),
        }
    }

    pub fn tryval_succ<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::TryValSucc,
            target: target.into(),
        }
    }

    pub fn tyappabs<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::TyAppAbs,
            target: target.into(),
        }
    }

    pub fn tyappabs_sub<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::TyAppAbsSub,
            target: target.into(),
        }
    }

    pub fn unfoldfold<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::UnfoldFold,
            target: target.into(),
        }
    }

    pub fn unpackpack<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::UnpackPack,
            target: target.into(),
        }
    }

    pub fn variantcase<T1, T2>(source: T1, target: T2) -> EvalStep<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        EvalStep {
            source: source.into(),
            rule: EvaluationRule::VariantCase,
            target: target.into(),
        }
    }
}
