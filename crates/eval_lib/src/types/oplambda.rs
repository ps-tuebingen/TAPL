impl<Ty> Normalize<Ty> for OpLambda<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let body_norm = self.body.normalize(env);
        OpLambda {
            var: self.var,
            annot: self.annot,
            body: Box::new(body_norm),
        }
        .into()
    }
}
