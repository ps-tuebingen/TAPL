use super::{to_check_err, Env};
use crate::{
    terms::{Variant, VariantCase},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

impl<'a> Typecheck<'a> for Variant {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let vars = self.annot.as_variant().map_err(to_check_err)?;
        let term_ty = self.term.check(env)?;
        let (_, ty) = vars
            .iter()
            .find(|(lb, _)| *lb == self.label)
            .ok_or(to_check_err(ErrorKind::UndefinedLabel(self.label.clone())))?;
        term_ty.equal(ty).map_err(to_check_err)?;
        Ok(self.annot.clone())
    }
}

impl<'a> Typecheck<'a> for VariantCase {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.bound_term.check(env)?;
        let variants = term_ty.as_variant().map_err(to_check_err)?;

        let mut rhs_tys = vec![];

        for pt in self.patterns.iter() {
            let (_, var_ty) = variants
                .iter()
                .find(|(label, _)| *label == pt.label)
                .ok_or(to_check_err(ErrorKind::UndefinedLabel(pt.label.clone())))?;
            let mut new_env = env.clone();
            new_env.insert(pt.bound_var.clone(), var_ty.clone());
            let rhs_ty = pt.rhs.check(&mut new_env)?;
            rhs_tys.push(rhs_ty);
        }

        let rhs_fst = rhs_tys
            .first()
            .ok_or(to_check_err(ErrorKind::Arity {
                found: 0,
                expected: variants.len(),
            }))
            .cloned()?;
        let _ = rhs_tys
            .into_iter()
            .map(|ty| ty.equal(&rhs_fst.clone()))
            .collect::<Result<Vec<Type>, ErrorKind>>()
            .map_err(to_check_err)?;

        Ok(rhs_fst)
    }
}
