impl<T> Typecheck for Tuple<T>
where
    T: LanguageTerm,
    TupleTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let mut tys: Vec<Self::Type> = vec![];
        let mut knd = None;
        for t in self.terms.iter() {
            let ty = t.check(&mut env.clone())?.normalize(&mut env.clone());
            let ty_knd = ty.check_kind(env)?;
            tys.push(ty);

            match knd {
                None => {
                    knd = Some(ty_knd);
                }
                Some(ref knd) => {
                    ty_knd.check_equal(knd).map_err(to_check_err)?;
                }
            }
        }
        Ok(TupleTy::new::<Self::Type>(tys).into())
    }
}
