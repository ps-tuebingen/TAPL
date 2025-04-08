use super::{errors::Error, Eval, Value};
use crate::{
    syntax::{Variant, VariantCase, VariantPattern},
    traits::subst::Subst,
};

impl Eval<'_> for Variant {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let val = self.term.eval(env)?;
        Ok(Value::Variant {
            label: self.label,
            ty: self.ty,
            val: Box::new(val),
        })
    }
}

impl Eval<'_> for VariantCase {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        let bound_val = self.bound_term.eval(env)?;
        let (lb, val) = if let Value::Variant { label, ty: _, val } = bound_val {
            Ok((label, val))
        } else {
            Err(Error::BadValue { val: bound_val })
        }?;
        let VariantPattern {
            label: _,
            bound_var,
            rhs,
        } = self
            .cases
            .into_iter()
            .find(
                |VariantPattern {
                     label,
                     bound_var: _,
                     rhs: _,
                 }| *label == lb,
            )
            .ok_or(Error::MissingPattern { label: lb })?;
        rhs.subst(&bound_var, (*val).into()).eval(env)
    }
}

#[cfg(test)]
mod variant_tests {
    use super::{Eval, Value, Variant, VariantCase, VariantPattern};
    use crate::{
        syntax::{True, Zero},
        types::Type,
    };
    use std::collections::HashMap;

    #[test]
    fn eval_variant() {
        let result = Variant {
            label: "label".to_owned(),
            term: Box::new(Zero.into()),
            ty: Type::Variant(HashMap::from([("label".to_owned(), Type::Nat)])),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Variant {
            label: "label".to_owned(),
            ty: Type::Variant(HashMap::from([("label".to_owned(), Type::Nat)])),
            val: Box::new(Value::Zero),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_case() {
        let result = VariantCase {
            bound_term: Box::new(
                Variant {
                    label: "label".to_owned(),
                    term: Box::new(Zero.into()),
                    ty: Type::Variant(HashMap::from([("label".to_owned(), Type::Nat)])),
                }
                .into(),
            ),
            cases: vec![
                VariantPattern {
                    label: "label".to_owned(),
                    bound_var: "x".to_owned(),
                    rhs: Box::new("x".to_owned().into()),
                },
                VariantPattern {
                    label: "label2".to_owned(),
                    bound_var: "y".to_owned(),
                    rhs: Box::new(True.into()),
                },
            ],
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Zero;
        assert_eq!(result, expected)
    }
}
