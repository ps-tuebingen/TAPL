use super::{errors::Error, TypingEnv};
use crate::{
    syntax::{Left, Right, SumCase},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Left {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let ty = self.left_term.check(env)?;
        let (annot_l, annot_r) = if let Type::Sum(annot_left, annot_right) = self.ty.clone() {
            (annot_left, annot_right)
        } else {
            return Err(Error::UnexpectedType {
                term: self.clone().into(),
                ty: self.ty.clone(),
            });
        };
        if ty != *annot_l {
            return Err(Error::TypeMismatch {
                types: vec![ty, *annot_l],
            });
        }

        Ok(Type::Sum(annot_l, annot_r))
    }
}

impl<'a> Typecheck<'a> for Right {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let ty = self.right_term.check(env)?;
        let (annot_l, annot_r) = if let Type::Sum(annot_left, annot_right) = self.ty.clone() {
            (annot_left, annot_right)
        } else {
            return Err(Error::UnexpectedType {
                term: self.clone().into(),
                ty: self.ty.clone(),
            });
        };

        if ty != *annot_r {
            return Err(Error::TypeMismatch {
                types: vec![ty, *annot_r],
            });
        }

        Ok(Type::Sum(annot_l, Box::new(ty)))
    }
}

impl<'a> Typecheck<'a> for SumCase {
    type Type = Type;
    type Error = Error;
    type Env = &'a mut TypingEnv;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        if let Type::Sum(left_ty, right_ty) = bound_ty {
            env.used_vars.insert(self.left_var.clone(), *left_ty);
            let left_checked = self.left_term.check(&mut env.clone())?;
            env.used_vars.remove(&self.left_var);

            env.used_vars.insert(self.right_var.clone(), *right_ty);
            let right_checked = self.right_term.check(env)?;

            if left_checked == right_checked {
                Ok(left_checked)
            } else {
                Err(Error::TypeMismatch {
                    types: vec![left_checked, right_checked],
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
    use super::{Left, Right, SumCase};
    use crate::{
        syntax::{IsZero, True, Zero},
        types::Type,
    };
    use common::Typecheck;

    #[test]
    fn check_left() {
        let result = Left {
            left_term: Box::new(Zero.into()),
            ty: Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
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
            ty: Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
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
                    ty: Type::Sum(Box::new(Type::Nat), Box::new(Type::Bool)),
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
