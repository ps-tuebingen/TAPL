use super::{errors::Error, is_subtype, TypingContext};
use crate::{
    syntax::{Assign, Deref, Location, Ref},
    types::Type,
};
use common::Typecheck;

impl<'a> Typecheck<'a> for Ref {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner = self.term.check(env)?;
        Ok(Type::Ref(Box::new(inner)))
    }
}

impl<'a> Typecheck<'a> for Deref {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let inner = self.term.check(env)?;
        match inner {
            Type::Ref(ty) => Ok(*ty),
            Type::Sink(ty) => Ok(*ty),
            Type::Source(ty) => Ok(*ty),
            ty => Err(Error::NoReference(ty)),
        }
    }
}

impl<'a> Typecheck<'a> for Assign {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let left_ty = self.to.check(&mut env.clone())?;
        let right_ty = self.content.check(env)?;
        match left_ty {
            Type::Ref(ty) => {
                if right_ty == *ty {
                    Ok(Type::Unit)
                } else {
                    Err(Error::TypeMismatch(*ty, right_ty))
                }
            }
            Type::Sink(ty) => {
                if is_subtype(&right_ty, &ty) {
                    Ok(Type::Unit)
                } else {
                    Err(Error::TypeMismatch(*ty, right_ty))
                }
            }
            ty => Err(Error::TypeMismatch(right_ty, ty)),
        }
    }
}

impl<'a> Typecheck<'a> for Location {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut TypingContext;
    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let stored = env
            .lookup_location(self.loc)
            .ok_or(Error::UnassignedLocation(self.loc))?;
        match stored {
            Type::Ref(ty) => Ok(*ty),
            Type::Sink(ty) => Ok(*ty),
            Type::Source(ty) => Ok(*ty),
            ty => Err(Error::NoReference(ty)),
        }
    }
}
