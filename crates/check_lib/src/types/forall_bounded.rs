impl<Ty> Subtypecheck<Ty> for ForallBounded<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let other_forall = sup.clone().into_forall_bounded().map_err(to_subty_err)?;
        let sup_norm = other_forall.sup_ty.normalize(env);
        let self_norm = self.sup_ty.clone().normalize(env);
        sup_norm.check_equal(&self_norm).map_err(to_subty_err)?;
        if self.var != other_forall.var {
            return Err(to_subty_err(ErrorKind::TypeMismatch {
                found: other_forall.var.clone(),
                expected: self.var.clone(),
            }));
        }
        let ty_norm = self.ty.clone().normalize(env);
        ty_norm.check_subtype(&(*other_forall.ty), env)
    }
}

impl<Ty> Kindcheck<Ty> for ForallBounded<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let sup_kind = self.sup_ty.check_kind(env)?;
        env.add_tyvar_kind(self.var.clone(), sup_kind);
        self.ty.check_kind(env)
    }
}
