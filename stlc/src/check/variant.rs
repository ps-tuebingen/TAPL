use super::{Check, TypingEnv};
use crate::{
    terms::syntax::{Variant, VariantCase, VariantPattern},
    types::Type,
};

impl Check for Variant {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let term_ty = self.term.check(env)?;
        match &self.ty {
            Type::Variant(vars) => {
                if vars.get(&self.label) == Some(&term_ty) {
                    Some(self.ty.clone())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Check for VariantCase {
    fn check(&self, env: &mut TypingEnv) -> Option<Type> {
        let bound_ty = self.bound_term.check_local(env)?;
        let var_ty = if let Type::Variant(vars) = bound_ty {
            Some(vars)
        } else {
            None
        }?;

        if var_ty.keys().len() != self.cases.len() {
            None
        } else {
            Some(())
        }?;

        let mut rhs_types = vec![];
        for VariantPattern {
            label,
            bound_var,
            rhs,
        } in self.cases.iter()
        {
            let var_ty = var_ty.get(label)?;
            env.used_vars.insert(bound_var.clone(), var_ty.clone());
            let rhs_ty = rhs.check_local(env)?;
            env.used_vars.remove(bound_var);
            rhs_types.push(rhs_ty);
        }
        if rhs_types.windows(2).all(|tys| tys[0] == tys[1]) {
            rhs_types.first().cloned()
        } else {
            None
        }
    }
}
