impl<Ty> Normalize<Ty> for Forall<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_norm = self.ty.normalize(env);
        Forall {
            var: self.var,
            kind: self.kind,
            ty: Box::new(ty_norm),
        }
        .into()
    }
}
