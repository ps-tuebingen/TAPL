impl<T> Typecheck for VariantCase<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        let bound_var = bound_ty.into_variant().map_err(to_check_err)?;

        let mut rhs_tys = vec![];
        let mut rhs_knd = None;

        for pt in self.patterns.iter() {
            let var_ty = bound_var
                .variants
                .get(&pt.label)
                .ok_or(to_check_err(ErrorKind::UndefinedLabel(pt.label.clone())))
                .cloned()?
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
                    rhs.check_equal(&knd).map_err(to_check_err)?;
                }
            }
            rhs_tys.push(rhs_ty)
        }

        if rhs_tys.is_empty() {
            return Err(to_check_err(ErrorKind::Arity {
                found: 0,
                expected: bound_var.variants.len(),
            }));
        }

        let rhs_fst = rhs_tys.remove(0);
        if let Some(ty) = rhs_tys.iter().find(|ty| rhs_fst.check_equal(ty).is_err()) {
            return Err(to_check_err(ErrorKind::TypeMismatch {
                found: ty.to_string(),
                expected: rhs_fst.to_string(),
            }));
        }

        Ok(rhs_fst)
    }
}
