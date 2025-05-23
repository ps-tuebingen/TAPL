impl<Ty> Normalize<Ty> for Exists<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.into()
    }
}
