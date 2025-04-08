use super::{errors::Error, TypingEnv};
use crate::{
    syntax::{Variant, VariantCase, VariantPattern},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Variant {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let term_ty = self.term.check(env)?;
        match &self.ty {
            Type::Variant(labels) => {
                if labels.get(&self.label) == Some(&term_ty) {
                    Ok(self.ty.clone())
                } else {
                    Err(Error::UndefinedLabel {
                        label: self.label.clone(),
                    })
                }
            }
            _ => Err(Error::UnexpectedType {
                ty: self.ty.clone(),
                term: self.clone().into(),
            }),
        }
    }
}

impl<'a> Typecheck<'a> for VariantCase {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingEnv;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        let var_ty = if let Type::Variant(vars) = bound_ty {
            Ok(vars)
        } else {
            Err(Error::UnexpectedType {
                ty: bound_ty,
                term: self.clone().into(),
            })
        }?;

        if var_ty.keys().len() != self.cases.len() {
            Err(Error::WrongNumberOfCases {
                found: self.cases.len(),
                expected: var_ty.keys().len(),
            })
        } else {
            Ok(())
        }?;

        let mut rhs_types = vec![];
        for VariantPattern {
            label,
            bound_var,
            rhs,
        } in self.cases.iter()
        {
            let var_ty = var_ty.get(label).ok_or(Error::UndefinedLabel {
                label: label.clone(),
            })?;
            env.used_vars.insert(bound_var.clone(), var_ty.clone());
            let rhs_ty = rhs.check(&mut env.clone())?;
            env.used_vars.remove(bound_var);
            rhs_types.push(rhs_ty);
        }
        if rhs_types.windows(2).all(|tys| tys[0] == tys[1]) {
            Ok(rhs_types.first().unwrap().clone())
        } else {
            let mut tys_unique = rhs_types.clone();
            tys_unique.sort_by_key(|ty| ty.to_string());
            tys_unique.dedup();
            Err(Error::TypeMismatch { types: tys_unique })
        }
    }
}

#[cfg(test)]
mod variant_tests {
    use super::{Variant, VariantCase, VariantPattern};
    use crate::{
        syntax::{IsZero, Zero},
        types::Type,
    };
    use common::Typecheck;
    use std::collections::HashMap;

    #[test]
    fn check_variant() {
        let result = Variant {
            label: "label1".to_owned(),
            term: Box::new(Zero.into()),
            ty: Type::Variant(HashMap::from([
                ("label1".to_owned(), Type::Nat),
                ("label2".to_owned(), Type::Bool),
            ])),
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Variant(HashMap::from([
            ("label1".to_owned(), Type::Nat),
            ("label2".to_owned(), Type::Bool),
        ]));
        assert_eq!(result, expected)
    }

    #[test]
    fn check_variantcase() {
        let result = VariantCase {
            bound_term: Box::new(
                Variant {
                    label: "label1".to_owned(),
                    term: Box::new(Zero.into()),
                    ty: Type::Variant(HashMap::from([
                        ("label1".to_owned(), Type::Nat),
                        ("label2".to_owned(), Type::Bool),
                    ])),
                }
                .into(),
            ),
            cases: vec![
                VariantPattern {
                    bound_var: "x".to_owned(),
                    label: "label1".to_owned(),
                    rhs: Box::new(
                        IsZero {
                            term: Box::new("x".to_owned().into()),
                        }
                        .into(),
                    ),
                },
                VariantPattern {
                    bound_var: "y".to_owned(),
                    label: "label2".to_owned(),
                    rhs: Box::new("y".to_owned().into()),
                },
            ],
        }
        .check(&mut Default::default())
        .unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }
}
