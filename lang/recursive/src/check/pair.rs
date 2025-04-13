use super::{to_check_err, Env};
use crate::{
    terms::{Fst, Pair, Snd},
    types::Type,
};

use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Pair {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let fst_ty = self.fst.check(&mut env.clone())?;
        let snd_ty = self.snd.check(env)?;
        Ok(Type::Pair(Box::new(fst_ty), Box::new(snd_ty)))
    }
}

impl<'a> Typecheck<'a> for Fst {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let ty = self.term.check(env)?;
        let (fst, _) = ty.as_pair().map_err(to_check_err)?;
        Ok(fst)
    }
}

impl<'a> Typecheck<'a> for Snd {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let ty = self.term.check(env)?;
        let (_, snd) = ty.as_pair().map_err(to_check_err)?;
        Ok(snd)
    }
}
