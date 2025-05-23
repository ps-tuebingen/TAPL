impl<Ty> Normalize<Ty> for TypeVariable<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.get_tyvar_super(&self.v).unwrap_or(self.into())
    }
}
