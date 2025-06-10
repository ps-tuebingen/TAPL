use crate::{errors::EmptyCase, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch, UndefinedLabel};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, VariantCase},
    types::{Type, TypeGroup},
};

impl<T> Typecheck for VariantCase<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError:
        From<TypeMismatch> + From<EmptyCase> + From<UndefinedLabel> + From<KindMismatch>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?.into_star()?;
        let bound_var = bound_ty.into_variant()?;

        let mut rhs_tys = vec![];
        let mut rhs_ress = vec![];
        let mut rhs_knd = None;

        for pt in self.patterns.iter() {
            let var_ty = bound_var
                .variants
                .get(&pt.label)
                .cloned()
                .ok_or(UndefinedLabel::new(&pt.label))?
                .normalize(env.clone());
            var_ty.check_kind(env.clone())?;

            let mut rhs_env = env.clone();
            rhs_env.add_var(pt.bound_var.clone(), var_ty);
            let rhs_res = pt.rhs.check(rhs_env.clone())?;
            let rhs_ty = rhs_res.ty().normalize(rhs_env);
            rhs_ress.push(rhs_res);
            let knd = rhs_ty.check_kind(env.clone())?;

            match rhs_knd {
                None => {
                    rhs_knd = Some(knd);
                }
                Some(ref rhs) => {
                    rhs.check_equal(&knd)?;
                }
            }
            rhs_tys.push(rhs_ty)
        }

        if rhs_tys.is_empty() {
            return Err(EmptyCase.into());
        }

        let rhs_fst = rhs_tys.remove(0);
        if let Some(ty) = rhs_tys.iter().find(|ty| rhs_fst.check_equal(ty).is_err()) {
            return Err(TypeMismatch::new(ty.knd(), rhs_fst.knd()).into());
        }

        let conc = Conclusion::new(env, self.clone(), rhs_fst);
        let deriv = Derivation::variantcase(conc, bound_res, rhs_ress);

        Ok(deriv)
    }
}
