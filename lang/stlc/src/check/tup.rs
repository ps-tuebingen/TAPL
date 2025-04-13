use super::{to_check_err, TypingEnv};
use crate::{
    syntax::{Proj, Tup},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Tup {
    type Type = Type;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let mut tys = vec![];
        for term in self.terms.iter() {
            let ty = term.check(&mut env.clone())?;
            tys.push(ty);
        }
        Ok(Type::Tup(tys))
    }
}

impl<'a> Typecheck<'a> for Proj {
    type Type = Type;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let tup_ty = self.tup.check(env)?;
        if let Type::Tup(tys) = tup_ty {
            tys.get(self.ind)
                .ok_or(to_check_err(ErrorKind::TermMismatch {
                    found: self.to_string(),
                    expected: format!("Projection index less than {}", tys.len()),
                }))
                .cloned()
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: tup_ty.to_string(),
                expected: "Tuple Type".to_owned(),
            }))
        }
    }
}

#[cfg(test)]
mod tup_tests {
    use super::{Proj, Tup};
    use crate::{
        syntax::{Nil, Nothing, True, Zero},
        types::Type,
    };
    use common::Typecheck;

    #[test]
    fn check_tup() {
        let result = Tup {
            terms: vec![
                Zero.into(),
                True.into(),
                Nil {
                    inner_type: Type::Bool,
                }
                .into(),
                Nothing {
                    inner_type: Type::Nat,
                }
                .into(),
            ],
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Tup(vec![
            Type::Nat,
            Type::Bool,
            Type::List(Box::new(Type::Bool)),
            Type::Optional(Box::new(Type::Nat)),
        ]);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_proj() {
        let result = Proj {
            tup: Box::new(
                Tup {
                    terms: vec![
                        Zero.into(),
                        True.into(),
                        Nil {
                            inner_type: Type::Bool,
                        }
                        .into(),
                        Nothing {
                            inner_type: Type::Nat,
                        }
                        .into(),
                    ],
                }
                .into(),
            ),
            ind: 0,
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Nat;
        assert_eq!(result, expected)
    }
}
