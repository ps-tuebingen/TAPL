use super::{is_subtype, to_check_err, TypingContext};
use crate::{
    syntax::{Assign, Deref, Location, Ref},
    types::Type,
};
use common::{
    errors::{Error, ErrorKind},
    Typecheck,
};

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
            ty => Err(to_check_err(ErrorKind::TypeMismatch {
                found: ty.to_string(),
                expected: "Reference".to_owned(),
            })),
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
                    Err(to_check_err(ErrorKind::TypeMismatch {
                        found: ty.to_string(),
                        expected: right_ty.to_string(),
                    }))
                }
            }
            Type::Sink(ty) => {
                if is_subtype(&right_ty, &ty) {
                    Ok(Type::Unit)
                } else {
                    Err(to_check_err(ErrorKind::TypeMismatch {
                        found: ty.to_string(),
                        expected: right_ty.to_string(),
                    }))
                }
            }
            ty => Err(to_check_err(ErrorKind::TypeMismatch {
                found: right_ty.to_string(),
                expected: ty.to_string(),
            })),
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
            .ok_or(to_check_err(ErrorKind::UndefinedLocation(self.loc)))?;
        match stored {
            Type::Ref(ty) => Ok(*ty),
            Type::Sink(ty) => Ok(*ty),
            Type::Source(ty) => Ok(*ty),
            ty => Err(to_check_err(ErrorKind::TypeMismatch {
                found: ty.to_string(),
                expected: "Reference Type".to_owned(),
            })),
        }
    }
}
