use super::SolveState;
use crate::constraints::KindConstraint;
use errors::{KindMismatch, inference_error::InferenceError};
use syntax::{kinds::Kind, language::Language};

pub fn solve_kinding<Lang>(
    knd: KindConstraint,
    state: &mut SolveState<Lang>,
) -> Result<(), InferenceError>
where
    Lang: Language,
{
    match (knd.left, knd.right) {
        (Kind::Var(v1), Kind::Var(v2)) => {
            match (state.kind_vars.get(&v1), state.kind_vars.get(&v2)) {
                (None, None) => {
                    state.add_constraint(KindConstraint::new(v1.as_str(), v2.as_str(), knd.span));
                    Ok(())
                }
                (Some(k1), None) => {
                    state.kind_vars.insert(v2, k1.clone());
                    Ok(())
                }
                (None, Some(k2)) => {
                    state.kind_vars.insert(v1, k2.clone());
                    Ok(())
                }
                (Some(k1), Some(k2)) => {
                    if k1 == k2 {
                        Ok(())
                    } else {
                        Err(KindMismatch::new(k1.to_string(), k2.to_string(), knd.span).into())
                    }
                }
            }
        }
        (left, Kind::Var(v)) => match state.kind_vars.get(&v) {
            None => {
                state.kind_vars.insert(v, left);
                Ok(())
            }
            Some(k2) => {
                if left == *k2 {
                    Ok(())
                } else {
                    Err(KindMismatch::new(left.to_string(), k2.to_string(), knd.span).into())
                }
            }
        },
        (Kind::Var(v), right) => match state.kind_vars.get(&v) {
            None => {
                state.kind_vars.insert(v, right);
                Ok(())
            }
            Some(k2) => {
                if right == *k2 {
                    Ok(())
                } else {
                    Err(KindMismatch::new(right.to_string(), k2.to_string(), knd.span).into())
                }
            }
        },
        (left, right) => {
            if left == right {
                Ok(())
            } else {
                Err(KindMismatch::new(left.to_string(), right.to_string(), knd.span).into())
            }
        }
    }
}
