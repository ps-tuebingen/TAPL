use super::{errors::Error, Eval, Value};
use crate::{
    syntax::{Left, Right, SumCase},
    traits::subst::Subst,
};

impl Eval<'_> for Left {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let left_val = self.left_term.eval(env)?;
        Ok(Value::Left {
            left_term: Box::new(left_val),
            ty: self.ty,
        })
    }
}

impl Eval<'_> for Right {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let right_val = self.right_term.eval(env)?;
        Ok(Value::Right {
            right_term: Box::new(right_val),
            ty: self.ty,
        })
    }
}

impl Eval<'_> for SumCase {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, env: Self::Env) -> Result<Self::Value, Self::Err> {
        let bound_val = self.bound_term.eval(env)?;
        match bound_val {
            Value::Left {
                left_term: val,
                ty: _,
            } => self
                .left_term
                .subst(&self.left_var, (*val).into())
                .eval(env),
            Value::Right {
                right_term: val,
                ty: _,
            } => self
                .right_term
                .subst(&self.right_var, (*val).into())
                .eval(env),
            _ => Err(Error::BadValue { val: bound_val }),
        }
    }
}

#[cfg(test)]
mod sum_tests {
    use super::{Eval, Left, Right, SumCase, Value};
    use crate::{
        syntax::{IsZero, True, Zero},
        types::Type,
    };

    #[test]
    fn eval_left() {
        let result = Left {
            left_term: Box::new(Zero.into()),
            ty: Type::Bool,
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Left {
            left_term: Box::new(Value::Zero),
            ty: Type::Bool,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_right() {
        let result = Right {
            right_term: Box::new(True.into()),
            ty: Type::Nat,
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::Right {
            right_term: Box::new(Value::True),
            ty: Type::Nat,
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_case_left() {
        let result = SumCase {
            bound_term: Box::new(
                Left {
                    left_term: Box::new(Zero.into()),
                    ty: Type::Bool,
                }
                .into(),
            ),
            left_var: "x".to_owned(),
            left_term: Box::new(
                IsZero {
                    term: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            right_var: "x".to_owned(),
            right_term: Box::new("x".to_owned().into()),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_case_right() {
        let result = SumCase {
            bound_term: Box::new(
                Right {
                    right_term: Box::new(True.into()),
                    ty: Type::Nat,
                }
                .into(),
            ),
            left_var: "x".to_owned(),
            left_term: Box::new(
                IsZero {
                    term: Box::new("x".to_owned().into()),
                }
                .into(),
            ),
            right_var: "x".to_owned(),
            right_term: Box::new("x".to_owned().into()),
        }
        .eval(Default::default())
        .unwrap();
        let expected = Value::True;
        assert_eq!(result, expected)
    }
}
