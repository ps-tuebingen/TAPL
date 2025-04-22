use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::Eval,
    language::{LanguageTerm, LanguageType},
    subst::{SubstTerm, SubstType},
    types::Variant as VariantTy,
    values::Variant as VariantVal,
    Label, TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<T>
where
    T: LanguageTerm,
{
    label: Label,
    term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Variant<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty1>(lb: &str, t: T1, ty: Ty1) -> Variant<T>
    where
        T1: Into<T>,
        Ty1: Into<<T as LanguageTerm>::Type>,
    {
        Variant {
            label: lb.to_owned(),
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Variant<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Variant<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Variant {
            label: self.label,
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Variant<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Variant {
            label: self.label,
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Variant<T>
where
    T: LanguageTerm,
    VariantTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.term.check(env)?;
        let term_knd = term_ty.check_kind(env)?;

        let var_ty = self.ty.clone().into_variant().map_err(to_check_err)?;
        let lb_ty = var_ty
            .variants
            .get(&self.label)
            .ok_or(to_check_err(ErrorKind::UndefinedLabel(self.label.clone())))
            .cloned()?;
        let lb_knd = lb_ty.check_kind(env)?;

        lb_knd.check_equal(&term_knd).map_err(to_check_err)?;
        lb_ty.check_equal(&term_ty).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}

impl<T> Eval for Variant<T>
where
    T: LanguageTerm,
    VariantVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(VariantVal::<T>::new(&self.label, term_val, self.ty).into())
    }
}

impl<T> fmt::Display for Variant<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> as {}", self.label, self.term, self.ty)
    }
}
