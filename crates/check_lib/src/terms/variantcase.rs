use crate::{
    errors::{EmptyCase, UndefinedLabel},
    CheckEnvironment, Kindcheck, Normalize, Typecheck,
};
use syntax::{
    terms::{Term, VariantCase},
    types::{Type, TypeGroup},
};

impl<T> Typecheck for VariantCase<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    <T as Typecheck>::CheckError:
        From<syntax::errors::Error> + From<EmptyCase> + From<UndefinedLabel>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty.check_kind(&mut env.clone())?.into_star()?;
        let bound_var = bound_ty.into_variant()?;

        let mut rhs_tys = vec![];
        let mut rhs_knd = None;

        for pt in self.patterns.iter() {
            let var_ty = bound_var
                .variants
                .get(&pt.label)
                .cloned()
                .ok_or(UndefinedLabel::new(&pt.label))?
                .normalize(&mut env.clone());
            var_ty.check_kind(&mut env.clone())?;

            let mut rhs_env = env.clone();
            rhs_env.add_var(pt.bound_var.clone(), var_ty);
            let rhs_ty = pt.rhs.check(&mut rhs_env)?.normalize(&mut rhs_env.clone());
            let knd = rhs_ty.check_kind(env)?;

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
            return Err(syntax::errors::Error::TypeMismatch {
                found: ty.knd(),
                expected: rhs_fst.knd(),
            }
            .into());
        }

        Ok(rhs_fst)
    }
}
