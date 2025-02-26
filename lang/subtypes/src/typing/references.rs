use super::{errors::Error, is_subtype, Typecheck, TypingContext};
use crate::{
    syntax::{Assign, Deref, Location, Ref},
    types::Type,
};

impl Typecheck for Ref {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let inner = self.term.check(env)?;
        Ok(Type::Ref(Box::new(inner)))
    }
}

impl Typecheck for Deref {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
        let inner = self.term.check(env)?;
        match inner {
            Type::Ref(ty) => Ok(*ty),
            Type::Sink(ty) => Ok(*ty),
            Type::Source(ty) => Ok(*ty),
            ty => Err(Error::NoReference(ty)),
        }
    }
}

impl Typecheck for Assign {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
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

impl Typecheck for Location {
    fn check(&self, env: &mut TypingContext) -> Result<Type, Error> {
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
