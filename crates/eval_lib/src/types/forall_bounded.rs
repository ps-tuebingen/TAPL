impl<Ty> Normalize<Ty> for ForallBounded<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.add_tyvar_super(self.var.clone(), *self.ty.clone());
        let ty_norm = self.ty.normalize(env);
        ForallBounded {
            var: self.var,
            sup_ty: self.sup_ty,
            ty: Box::new(ty_norm),
        }
        .into()
    }
}
