use super::{errors::Error, Check, TypingEnv};
use crate::{
    syntax::{Proj, Tup},
    types::Type,
};

impl Check for Tup {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let mut tys = vec![];
        for term in self.terms.iter() {
            let ty = term.check_local(env)?;
            tys.push(ty);
        }
        Ok(Type::Tup(tys))
    }
}

impl Check for Proj {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
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
    use super::{Check, Proj, Tup};
    use crate::{
        syntax::{Nil, Nothing, True, Zero},
        types::Type,
    };

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
