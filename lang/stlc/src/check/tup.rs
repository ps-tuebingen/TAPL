use super::{errors::Error, TypingEnv};
use crate::{
    syntax::{Proj, Tup},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Tup {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let tup_ty = self.tup.check(env)?;
        if let Type::Tup(tys) = tup_ty {
            tys.get(self.ind)
                .ok_or(Error::ProjectionOutOfBounds {
                    proj_ty: Type::Tup(tys.clone()),
                    ind: self.ind,
                })
                .cloned()
        } else {
            Err(Error::UnexpectedType {
                ty: tup_ty.clone(),
                term: self.clone().into(),
            })
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
