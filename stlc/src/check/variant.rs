use super::{errors::Error, Check, TypingEnv};
use crate::{
    terms::syntax::{Variant, VariantCase, VariantPattern},
    types::Type,
};

impl Check for Variant {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
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

impl Check for VariantCase {
    fn check(&self, env: &mut TypingEnv) -> Result<Type, Error> {
        let bound_ty = self.bound_term.check_local(env)?;
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
            let rhs_ty = rhs.check_local(env)?;
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
