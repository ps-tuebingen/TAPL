impl<Ty> Normalize<Ty> for ExistsBounded<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.add_tyvar_super(self.var.clone(), *self.sup_ty.clone());
        self.into()
    }
}
