use super::{Check, Env};
use crate::{
    errors::{Error, ErrorKind},
    terms::{Variant, VariantCase},
    types::Type,
};

impl Check for Variant {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let vars = self
            .annot
            .as_variant()
            .map_err(|knd| Error::check(knd, self))?;
        let term_ty = self.term.check(env)?;
        let (_, ty) = vars
            .iter()
            .find(|(lb, _)| *lb == self.label)
            .ok_or(Error::check(
                ErrorKind::UndefinedLabel(self.label.clone()),
                self,
            ))?;
        term_ty.equal(ty).map_err(|knd| Error::check(knd, self))?;
        Ok(self.annot.clone())
    }
}

impl Check for VariantCase {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let term_ty = self.bound_term.check(env)?;
        let variants = term_ty
            .as_variant()
            .map_err(|knd| Error::check(knd, self))?;

        let mut rhs_tys = vec![];

        for pt in self.patterns.iter() {
            let (_, var_ty) = variants
                .iter()
                .find(|(label, _)| *label == pt.label)
                .ok_or(Error::check(
                    ErrorKind::UndefinedLabel(pt.label.clone()),
                    self,
                ))?;
            let mut new_env = env.clone();
            new_env.insert(pt.bound_var.clone(), var_ty.clone());
            let rhs_ty = pt.rhs.check(&mut new_env)?;
            rhs_tys.push(rhs_ty);
        }

        let rhs_fst = rhs_tys
            .first()
            .ok_or(Error::check(ErrorKind::EmptyCase, self))
            .cloned()?;
        let _ = rhs_tys
            .into_iter()
            .map(|ty| ty.equal(&rhs_fst.clone()))
            .collect::<Result<Vec<Type>, ErrorKind>>()
            .map_err(|knd| Error::check(knd, self))?;

        Ok(rhs_fst)
    }
}
