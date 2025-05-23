impl<Ty> Kindcheck<Ty> for Exists<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.ty.check_kind(env)
    }
}
