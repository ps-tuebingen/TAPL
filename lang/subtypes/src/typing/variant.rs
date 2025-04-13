use super::{is_subtype, to_check_err, TypingContext};
use crate::{
    syntax::{Variant, VariantCase},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Variant {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let ty = self.term.check(env)?;
        Ok(Type::Variant(vec![(self.label.clone(), ty)]))
    }
}

impl<'a> Typecheck<'a> for VariantCase {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
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
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: bound_ty.to_string(),
                expected: variant_ty.to_string(),
            }))
        }
    }
}

fn combine_rhs(tys: Vec<Type>) -> Result<Type, Error> {
    if tys.is_empty() {
        return Err(to_check_err(ErrorKind::Arity {
            found: 0,
            expected: 1,
        }));
    }
    let mut ret_ty = tys.first().unwrap();
    for ty in tys.iter() {
        if is_subtype(ty, ret_ty) {
            ret_ty = ty;
        } else if is_subtype(ret_ty, ty) {
            continue;
        } else {
            return Err(to_check_err(ErrorKind::TypeMismatch {
                found: ty.to_string(),
                expected: ret_ty.to_string(),
            }));
        }
    }
    Ok(ret_ty.clone())
}
