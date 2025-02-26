use super::{errors::Error, Check, TypingEnv};
use crate::{
    syntax::{Left, Right, SumCase},
    types::Type,
};

impl Check for Left {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let left_ty = self.left_term.check(env)?;
        Ok(Type::Sum(
            Box::new(left_ty),
            Box::new(self.right_ty.clone()),
        ))
    }
}

impl Check for Right {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let right_ty = self.right_term.check(env)?;
        Ok(Type::Sum(
            Box::new(self.left_ty.clone()),
            Box::new(right_ty),
        ))
    }
}

impl Check for SumCase {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check_local(env)?;
        if let Type::Sum(left_ty, right_ty) = bound_ty {
            env.used_vars.insert(self.left_var.clone(), *left_ty);
            let left_ty = self.left_term.check_local(env)?;
            env.used_vars.remove(&self.left_var);

            env.used_vars.insert(self.right_var.clone(), *right_ty);
            let right_ty = self.right_term.check(env)?;

            if left_ty == right_ty {
                Ok(left_ty)
            } else {
                Err(Error::TypeMismatch {
                    types: vec![left_ty, right_ty],
                })
            }
        } else {
            Err(Error::UnexpectedType {
                ty: bound_ty.clone(),
                term: self.clone().into(),
            })
        }
    }
}

#[cfg(test)]
mod sum_tests {
    use super::{Check, Left, Right, SumCase};
    use crate::{
        syntax::{IsZero, True, Zero},
        types::Type,
    };

    #[test]
    fn check_left() {
        let result = Left {
            left_term: Box::new(Zero.into()),
            right_ty: Type::Bool,
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_right() {
        let result = Right {
            right_term: Box::new(True.into()),
            left_ty: Type::Nat,
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_case() {
        let result = SumCase {
            bound_term: Box::new(
                Left {
                    left_term: Box::new(Zero.into()),
                    right_ty: Type::Bool,
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
            right_var: "y".to_owned(),
            right_term: Box::new("y".to_owned().into()),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }
}
