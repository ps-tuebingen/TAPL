use super::{Check, Env};
use crate::{
    errors::Error,
    terms::{Fst, Pair, Snd},
    types::Type,
};

impl Check for Pair {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let fst_ty = self.fst.check(&mut env.clone())?;
        let snd_ty = self.snd.check(env)?;
        Ok(Type::Pair(Box::new(fst_ty), Box::new(snd_ty)))
    }
}

impl Check for Fst {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let ty = self.term.check(env)?;
        let (fst, _) = ty.as_pair().map_err(|knd| Error::check(knd, self))?;
        Ok(fst)
    }
}

impl Check for Snd {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        let ty = self.term.check(env)?;
        let (_, snd) = ty.as_pair().map_err(|knd| Error::check(knd, self))?;
        Ok(snd)
    }
}
