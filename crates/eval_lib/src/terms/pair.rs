impl<T> Eval for Pair<T>
where
    T: LanguageTerm,
    PairVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let fst_val = self.fst.eval(env)?;
        let snd_val = self.snd.eval(env)?;
        Ok(PairVal::<T>::new(fst_val, snd_val).into())
    }
}
