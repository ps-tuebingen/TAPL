impl<Ty> Normalize<Ty> for Fun<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let from_norm = self.from.normalize(env);
        let to_norm = self.to.normalize(env);
        Fun {
            from: Box::new(from_norm),
            to: Box::new(to_norm),
        }
        .into()
    }
}
