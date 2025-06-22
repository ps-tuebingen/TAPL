use common::errors::{
    FreeTypeVariable, FreeVariable, IndexOutOfBounds, KindMismatch, NameMismatch, TypeMismatch,
    UndefinedLabel, UndefinedLocation,
};
use std::fmt;
use syntax::types::Type;

pub mod empty_case;
pub mod not_a_subtype;

pub use empty_case::EmptyCase;
pub use not_a_subtype::NotASubtype;

#[derive(Debug)]
pub enum CheckError<Ty>
where
    Ty: Type,
{
    EmptyCase(EmptyCase),
    NotASubtype(NotASubtype<Ty>),
    KindMismatch(KindMismatch),
    TypeMismatch(TypeMismatch),
    UndefinedLocation(UndefinedLocation),
    IndexOutOfBounds(IndexOutOfBounds),
    FreeTypeVariable(FreeTypeVariable),
    UndefinedLabel(UndefinedLabel),
    NameMismatch(NameMismatch),
    FreeVariable(FreeVariable),
}

impl<Ty> fmt::Display for CheckError<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CheckError::EmptyCase(ec) => ec.fmt(f),
            CheckError::NotASubtype(ns) => ns.fmt(f),
            CheckError::KindMismatch(km) => km.fmt(f),
            CheckError::TypeMismatch(tm) => tm.fmt(f),
            CheckError::UndefinedLocation(ul) => ul.fmt(f),
            CheckError::IndexOutOfBounds(io) => io.fmt(f),
            CheckError::FreeTypeVariable(fv) => fv.fmt(f),
            CheckError::UndefinedLabel(ul) => ul.fmt(f),
            CheckError::NameMismatch(nm) => nm.fmt(f),
            CheckError::FreeVariable(fv) => fv.fmt(f),
        }
    }
}

impl<Ty> std::error::Error for CheckError<Ty> where Ty: Type {}

impl<Ty> From<EmptyCase> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: EmptyCase) -> CheckError<Ty> {
        CheckError::EmptyCase(err)
    }
}

impl<Ty> From<NotASubtype<Ty>> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: NotASubtype<Ty>) -> CheckError<Ty> {
        CheckError::NotASubtype(err)
    }
}

impl<Ty> From<KindMismatch> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: KindMismatch) -> CheckError<Ty> {
        CheckError::KindMismatch(err)
    }
}

impl<Ty> From<TypeMismatch> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: TypeMismatch) -> CheckError<Ty> {
        CheckError::TypeMismatch(err)
    }
}

impl<Ty> From<UndefinedLocation> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: UndefinedLocation) -> CheckError<Ty> {
        CheckError::UndefinedLocation(err)
    }
}

impl<Ty> From<IndexOutOfBounds> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: IndexOutOfBounds) -> CheckError<Ty> {
        CheckError::IndexOutOfBounds(err)
    }
}

impl<Ty> From<FreeTypeVariable> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: FreeTypeVariable) -> CheckError<Ty> {
        CheckError::FreeTypeVariable(err)
    }
}

impl<Ty> From<UndefinedLabel> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: UndefinedLabel) -> CheckError<Ty> {
        CheckError::UndefinedLabel(err)
    }
}

impl<Ty> From<NameMismatch> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: NameMismatch) -> CheckError<Ty> {
        CheckError::NameMismatch(err)
    }
}

impl<Ty> From<FreeVariable> for CheckError<Ty>
where
    Ty: Type,
{
    fn from(err: FreeVariable) -> CheckError<Ty> {
        CheckError::FreeVariable(err)
    }
}
