use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Kindcheck, Subtypecheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::{Eval, Normalize},
    language::{LanguageTerm, LanguageType},
    subst::{SubstTerm, SubstType},
    values::Pack as PackVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pack<T>
where
    T: LanguageTerm,
{
    inner_ty: <T as LanguageTerm>::Type,
    term: Box<T>,
    outer_ty: <T as LanguageTerm>::Type,
}

impl<T> Pack<T>
where
    T: LanguageTerm,
{
    pub fn new<Ty1, Ty2, T1>(inner: Ty1, t: T1, outer: Ty2) -> Pack<T>
    where
        Ty1: Into<<T as LanguageTerm>::Type>,
        Ty2: Into<<T as LanguageTerm>::Type>,
        T1: Into<T>,
    {
        Pack {
            inner_ty: inner.into(),
            term: Box::new(t.into()),
            outer_ty: outer.into(),
        }
    }
}

impl<T> Term for Pack<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Pack<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Pack {
            inner_ty: self.inner_ty,
            term: Box::new(self.term.subst(v, t)),
            outer_ty: self.outer_ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Pack<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Pack {
            inner_ty: self.inner_ty.subst_type(v, ty),
            term: Box::new(self.term.subst_type(v, ty)),
            outer_ty: self.outer_ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Eval for Pack<T>
where
    T: LanguageTerm,
    PackVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.term.eval(env)?;
        Ok(PackVal::<T>::new(self.inner_ty.clone(), term_val, self.outer_ty.clone()).into())
    }
}

impl<T> Typecheck for Pack<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let outer_norm = self.outer_ty.clone().normalize(env);
        let inner_kind = self.inner_ty.check_kind(env)?;
        let outer_knd = outer_norm.check_kind(env)?;
        println!("outer: {},outer norm:{}", self.outer_ty, outer_norm);

        if let Ok(outer_exists) = outer_norm.clone().into_exists() {
            env.add_tyvar_kind(outer_exists.var.clone(), outer_exists.kind.clone());
            let term_ty = self.term.check(env)?.normalize(env);
            let term_kind = term_ty.check_kind(env)?;

            term_kind.check_equal(&outer_knd).map_err(to_check_err)?;
            inner_kind
                .check_equal(&outer_exists.kind)
                .map_err(to_check_err)?;

            let outer_subst = outer_exists
                .ty
                .subst_type(&outer_exists.var, &self.inner_ty)
                .normalize(env);
            outer_subst.check_equal(&term_ty).map_err(to_check_err)?;
            Ok(outer_norm)
        } else if let Ok(outer_bound) = outer_norm.clone().into_exists_bounded() {
            let sup_norm = outer_bound.sup_ty.clone().normalize(env);
            let sup_kind = sup_norm.check_kind(env)?;
            env.add_tyvar_super(outer_bound.var.clone(), *outer_bound.sup_ty.clone());
            env.add_tyvar_kind(outer_bound.var.clone(), sup_kind);

            let term_ty = self.term.check(env)?;
            let term_kind = term_ty.check_kind(env)?;
            term_kind.check_equal(&outer_knd).map_err(to_check_err)?;

            let outer_subst = outer_bound.ty.subst_type(&outer_bound.var, &self.inner_ty);
            term_ty.check_subtype(&outer_subst, env)?;
            Ok(outer_norm)
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: outer_norm.to_string(),
                expected: "Existential Type".to_owned(),
            }))
        }
    }
}

impl<T> fmt::Display for Pack<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*({}),{}}} as {}",
            self.inner_ty, self.term, self.outer_ty
        )
    }
}
