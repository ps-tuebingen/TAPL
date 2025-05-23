impl<Ty> Normalize<Ty> for Record<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let mut recs_norm = HashMap::new();
        for (lb, ty) in self.records {
            let ty_norm = ty.normalize(env);
            recs_norm.insert(lb, ty_norm);
        }
        Record { records: recs_norm }.into()
    }
}
