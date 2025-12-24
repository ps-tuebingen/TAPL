use super::SolveState;
use crate::constraints::{KindConstraint, KindOrVar};
use errors::{KindMismatch, inference_error::InferenceError};
use syntax::language::Language;

pub fn solve_kinding<Lang>(
    knd: KindConstraint,
    state: &mut SolveState<Lang>,
) -> Result<(), InferenceError>
where
    Lang: Language,
{
    match (knd.left, knd.right) {
        (KindOrVar::Kind(k1), KindOrVar::Kind(k2)) if k1 == k2 => Ok(()),
        (KindOrVar::Kind(k1), KindOrVar::Kind(k2)) => {
            Err(KindMismatch::new(k1.to_string(), k2.to_string(), knd.span).into())
        }
        (KindOrVar::Kind(k), KindOrVar::Var(v)) => match state.kind_vars.get(&v) {
            None => {
                state.kind_vars.insert(v, k);
                Ok(())
            }
            Some(k2) => {
                if k == *k2 {
                    Ok(())
                } else {
                    Err(KindMismatch::new(k.to_string(), k2.to_string(), knd.span).into())
                }
            }
        },
        (KindOrVar::Var(v), KindOrVar::Kind(k)) => match state.kind_vars.get(&v) {
            None => {
                state.kind_vars.insert(v, k);
                Ok(())
            }
            Some(k2) => {
                if k == *k2 {
                    Ok(())
                } else {
                    Err(KindMismatch::new(k.to_string(), k2.to_string(), knd.span).into())
                }
            }
        },
        (KindOrVar::Var(v1), KindOrVar::Var(v2)) => {
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
    }
}
