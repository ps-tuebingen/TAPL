impl<T> Typecheck for Unpack<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        if let Ok(bound_exists) = bound_ty.clone().into_exists() {
            if self.ty_name != bound_exists.var {
                return Err(to_check_err(ErrorKind::TypeMismatch {
                    found: bound_exists.var,
                    expected: self.ty_name.clone(),
                }));
            }
            env.add_tyvar_kind(bound_exists.var, bound_exists.kind);
            env.add_var(self.term_name.clone(), *bound_exists.ty);
            Ok(self.in_term.check(&mut env.clone())?.normalize(env))
        } else if let Ok(bound_bound) = bound_ty.clone().into_exists_bounded() {
            if self.ty_name != bound_bound.var {
                return Err(to_check_err(ErrorKind::TypeMismatch {
                    found: bound_bound.var,
                    expected: self.ty_name.clone(),
                }));
            }
            let sup_kind = bound_bound.sup_ty.check_kind(env)?;
            env.add_tyvar_super(bound_bound.var, *bound_bound.sup_ty.clone());
            env.add_tyvar_kind(self.ty_name.clone(), sup_kind);
            env.add_var(self.term_name.clone(), *bound_bound.ty.clone());
            let inner_ty = self.in_term.check(&mut env.clone())?;
            Ok(inner_ty.normalize(env))
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Existential Type".to_owned(),
            }))
        }
    }
}
