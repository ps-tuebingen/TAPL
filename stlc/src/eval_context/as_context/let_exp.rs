use super::{AsContext, Error, EvalContext};
use crate::{
    eval_context::{computation::LetSubst, congruence},
    syntax::Let,
};

impl AsContext for Let {
    fn to_context(self) -> Result<EvalContext, Error> {
        match (&*self.bound_term).try_into() {
            Ok(val) => Ok(LetSubst {
                var: self.var,
                bound_val: val,
                in_term: *self.in_term,
            }
            .into()),
            Err(_) => {
                let ctx = (*self.bound_term).to_context()?;
                Ok(congruence::Let {
                    var: self.var,
                    bound_term: Box::new(ctx),
                    in_term: *self.in_term,
                }
                .into())
            }
        }
    }
}

#[cfg(test)]
mod let_tests {
    use super::{AsContext, Let, LetSubst};
    use crate::{
        eval_context::{computation::IsZeroNum, congruence, Value},
        syntax::{IsZero, Zero},
    };

    #[test]
    fn ctx_subst() {
        let result = Let {
            var: "x".to_owned(),
            bound_term: Box::new(Zero.into()),
            in_term: Box::new("x".to_owned().into()),
        }
        .to_context()
        .unwrap();
        let expected = LetSubst {
            bound_val: Value::Zero,
            var: "x".to_owned(),
            in_term: "x".to_owned().into(),
        }
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ctx_cong() {
        let result = Let {
            var: "x".to_owned(),
            bound_term: Box::new(
                IsZero {
                    term: Box::new(Zero.into()),
                }
                .into(),
            ),
            in_term: Box::new("x".to_owned().into()),
        }
        .to_context()
        .unwrap();
        let expected = congruence::Let {
            var: "x".to_owned(),
            bound_term: Box::new(IsZeroNum { num: Value::Zero }.into()),
            in_term: "x".to_owned().into(),
        }
        .into();
        assert_eq!(result, expected)
    }
}
