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
pub struct SumCase<T>
where
    T: LanguageTerm,
{
    pub bound_term: Box<T>,
    pub left_var: Var,
    pub left_term: Box<T>,
    pub right_var: Var,
    pub right_term: Box<T>,
}

impl<T> Term for SumCase<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for SumCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        let bound_subst = self.bound_term.subst(v, t);
        let left_term = if *v == self.left_var {
            self.left_term
        } else {
            Box::new(self.left_term.subst(v, t))
        };
        let right_term = if *v == self.right_var {
            self.right_term
        } else {
            Box::new(self.right_term.subst(v, t))
        };
        SumCase {
            bound_term: Box::new(bound_subst),
            left_var: self.left_var,
            left_term,
            right_var: self.right_var,
            right_term,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for SumCase<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        SumCase {
            bound_term: Box::new(self.bound_term.subst_type(v, ty)),
            left_var: self.left_var,
            left_term: Box::new(self.left_term.subst_type(v, ty)),
            right_var: self.right_var,
            right_term: Box::new(self.right_term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for SumCase<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?.normalize(env);
        bound_ty
            .check_kind(env)?
            .into_star()
            .map_err(to_check_err)?;
        let bound_sum = bound_ty.into_sum().map_err(to_check_err)?;

        let mut left_env = env.clone();
        left_env.add_var(self.left_var.clone(), *bound_sum.left);
        let left_ty = self.left_term.check(&mut left_env)?.normalize(env);
        let left_knd = left_ty.check_kind(&mut left_env)?;

        env.add_var(self.right_var.clone(), *bound_sum.right);
        let right_ty = self.right_term.check(env)?.normalize(env);
        let right_knd = right_ty.check_kind(env)?;

        left_knd.check_equal(&right_knd).map_err(to_check_err)?;
        left_ty.check_equal(&right_ty).map_err(to_check_err)?;
        Ok(right_ty)
    }
}

impl<T> Eval for SumCase<T>
where
    T: LanguageTerm,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let bound_val = self.bound_term.eval(env)?;
        if let Ok(left_val) = bound_val.clone().into_left() {
            self.left_term
                .subst(&self.left_var, &((*left_val.left_val).into()))
                .eval(env)
        } else if let Ok(right_val) = bound_val.clone().into_right() {
            self.right_term
                .subst(&self.right_var, &((*right_val.right_val).into()))
                .eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: bound_val.to_string(),
                expected: "Sum Value".to_owned(),
            }))
        }
    }
}

impl<T> fmt::Display for SumCase<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "case {} of {{ inl({}) => {} | inr({}) => {} }}",
            self.bound_term, self.left_var, self.left_term, self.right_var, self.right_term
        )
    }
}
