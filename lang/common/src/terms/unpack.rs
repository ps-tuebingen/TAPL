use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval},
    kinds::Kind,
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unpack<T>
where
    T: LanguageTerm,
{
    ty_name: TypeVar,
    term_name: Var,
    bound_term: Box<T>,
    in_term: Box<T>,
}

impl<T> Unpack<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, T2>(tyn: &str, tn: &str, bound: T1, int: T2) -> Unpack<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        Unpack {
            ty_name: tyn.to_owned(),
            term_name: tn.to_owned(),
            bound_term: Box::new(bound.into()),
            in_term: Box::new(int.into()),
        }
    }
}

impl<T> Term for Unpack<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Unpack<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.term_name {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(self.bound_term.subst(v, t)),
                in_term: self.in_term,
            }
            .into()
        } else {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(self.bound_term.subst(v, t)),
                in_term: Box::new(self.in_term.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Unpack<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;

    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        let bound_subst = self.bound_term.subst_type(v, ty);
        if *v == self.ty_name {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(bound_subst),
                in_term: self.in_term,
            }
            .into()
        } else {
            Unpack {
                ty_name: self.ty_name,
                term_name: self.term_name,
                bound_term: Box::new(bound_subst),
                in_term: Box::new(self.in_term.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<T> Eval for Unpack<T>
where
    T: LanguageTerm,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.bound_term.eval(env)?;
        let pack_val = term_val.into_pack().map_err(to_eval_err)?;
        self.in_term
            .subst_type(&self.ty_name, &pack_val.inner_ty)
            .subst(&self.term_name, &((*pack_val.val).into()))
            .eval(env)
    }
}

impl<T> Typecheck for Unpack<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self.bound_term.check(env)?;
        let bound_exists = bound_ty.into_exists().map_err(to_check_err)?;
        if self.ty_name != bound_exists.var {
            return Err(to_check_err(ErrorKind::TypeMismatch {
                found: bound_exists.var,
                expected: self.ty_name.clone(),
            }));
        }
        env.add_tyvar_kind(bound_exists.var, Kind::Star);
        env.add_var(self.term_name.clone(), *bound_exists.ty);
        self.in_term.check(env)
    }
}

impl<T> fmt::Display for Unpack<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "let {{{},{}}}={} in {}",
            self.ty_name, self.term_name, self.bound_term, self.in_term
        )
    }
}
