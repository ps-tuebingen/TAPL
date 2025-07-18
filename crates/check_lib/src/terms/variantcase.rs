use crate::{Kindcheck, Normalize, Typecheck, errors::CheckError, errors::EmptyCase};
use derivation::{Conclusion, TypingDerivation};
use errors::{TypeMismatch, UndefinedLabel};
use syntax::{
    env::Environment,
    terms::{Term, VariantCase},
    types::{Type, TypeGroup},
};

impl<T> Typecheck for VariantCase<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
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
        let deriv = TypingDerivation::variantcase(conc, bound_res, rhs_ress);

        Ok(deriv)
    }
}
