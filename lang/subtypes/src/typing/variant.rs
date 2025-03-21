use super::{errors::Error, is_subtype, Typecheck, TypingContext};
use crate::{
    syntax::{Variant, VariantCase},
    types::Type,
};

impl Typecheck for Variant {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let ty = self.term.check(env)?;
        Ok(Type::Variant(vec![(self.label.clone(), ty)]))
    }
}

impl Typecheck for VariantCase {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let mut var_tys = vec![];
        let mut rhs_tys = vec![];
        for pt in self.patterns.iter() {
            var_tys.push((pt.label.clone(), pt.var_ty.clone()));
            let mut new_env = env.clone();
            new_env.add_var(&pt.bound_var, &pt.var_ty);
            let rhs_ty = pt.rhs.check(&mut new_env)?;
            rhs_tys.push(rhs_ty);
        }
        let rhs_ty = combine_rhs(rhs_tys)?;
        let bound_ty = self.bound_term.check(env)?;
        let variant_ty = Type::Variant(var_tys);
        if is_subtype(&bound_ty, &variant_ty) {
            Ok(rhs_ty)
        } else {
            Err(Error::TypeMismatch(bound_ty, variant_ty))
        }
    }
}

fn combine_rhs(tys: Vec<Type>) -> Result<Type, Error> {
    if tys.is_empty() {
        return Err(Error::EmptyCase);
    }
    let mut ret_ty = tys.first().unwrap();
    for ty in tys.iter() {
        if is_subtype(ty, ret_ty) {
            ret_ty = ty;
        } else if is_subtype(ret_ty, ty) {
            continue;
        } else {
            return Err(Error::TypeMismatch(ty.clone(), ret_ty.clone()));
        }
    }
    Ok(ret_ty.clone())
}
