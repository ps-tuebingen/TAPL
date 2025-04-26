use super::Term;
use crate::{
    check::{to_check_err, CheckEnvironment, Kindcheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListCase<T>
where
    T: LanguageTerm,
{
    bound_term: Box<T>,
    nil_rhs: Box<T>,
    cons_fst: Var,
    cons_rst: Var,
    cons_rhs: Box<T>,
}

impl<T> ListCase<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, T2, T3>(bound: T1, nil: T2, hd: &str, tl: &str, cons: T3) -> ListCase<T>
    where
        T1: Into<T>,
        T2: Into<T>,
        T3: Into<T>,
    {
        ListCase {
            bound_term: Box::new(bound.into()),
            nil_rhs: Box::new(nil.into()),
            cons_fst: hd.to_owned(),
            cons_rst: tl.to_owned(),
            cons_rhs: Box::new(cons.into()),
        }
    }
}

impl<T> Term for ListCase<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for ListCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        let bound_subst = self.bound_term.subst(v, t);
        let nil_subst = self.nil_rhs.subst(v, t);
        if *v == self.cons_fst || *v == self.cons_rst {
            ListCase {
                bound_term: Box::new(bound_subst),
                nil_rhs: Box::new(nil_subst),
                cons_fst: self.cons_fst,
                cons_rst: self.cons_rst,
                cons_rhs: self.cons_rhs,
            }
            .into()
        } else {
            ListCase {
                bound_term: Box::new(bound_subst),
                nil_rhs: Box::new(nil_subst),
                cons_fst: self.cons_fst,
                cons_rst: self.cons_rst,
                cons_rhs: Box::new(self.cons_rhs.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for ListCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        ListCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            nil_rhs: Box::new(self.nil_rhs.subst_type(v, ty)),
            cons_fst: self.cons_fst,
            cons_rst: self.cons_rst,
            cons_rhs: Box::new(self.cons_rhs.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for ListCase<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self
            .bound_term
            .check(&mut &mut env.clone().clone())?
            .normalize(&mut env.clone());
        bound_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        let bound_list = bound_ty.clone().into_list().map_err(to_check_err)?;

        let nil_ty = self
            .nil_rhs
            .check(&mut &mut env.clone().clone())?
            .normalize(&mut env.clone());
        let nil_kind = nil_ty.check_kind(&mut env.clone())?;

        env.add_var(self.cons_fst.clone(), *bound_list.ty);
        env.add_var(self.cons_rst.clone(), bound_ty);
        let cons_ty = self
            .cons_rhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let cons_kind = cons_ty.check_kind(env)?;

        nil_kind.check_equal(&cons_kind).map_err(to_check_err)?;
        nil_ty.check_equal(&cons_ty).map_err(to_check_err)?;
        Ok(cons_ty)
    }
}

impl<T> Eval for ListCase<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(env)?;
        if bound_val.clone().into_nil().is_ok() {
            self.nil_rhs.eval(env)
        } else if let Ok(cons) = bound_val.clone().into_cons() {
            self.cons_rhs
                .subst(&self.cons_fst, &((*cons.head).into()))
                .subst(&self.cons_rst, &((*cons.tail).into()))
                .eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: bound_val.to_string(),
                expected: "List".to_owned(),
            }))
        }
    }
}

impl<T> fmt::Display for ListCase<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ Nil => {} | Cons({},{}) => {} }}",
            self.bound_term, self.nil_rhs, self.cons_fst, self.cons_rst, self.cons_rhs
        )
    }
}
