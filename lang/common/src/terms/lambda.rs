use super::Term;
use crate::{
    check::{CheckEnvironment, Kindcheck, Typecheck},
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Fun,
    values::Lambda as LambdaVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lambda<T>
where
    T: LanguageTerm,
{
    pub var: Var,
    pub annot: <T as LanguageTerm>::Type,
    pub body: Box<T>,
}

impl<T> Lambda<T>
where
    T: LanguageTerm,
{
    pub fn new<A, B>(v: &str, a: A, b: B) -> Lambda<T>
    where
        A: Into<<T as LanguageTerm>::Type>,
        B: Into<T>,
    {
        Lambda {
            var: v.to_owned(),
            annot: a.into(),
            body: Box::new(b.into()),
        }
    }
}

impl<T> Term for Lambda<T> where T: LanguageTerm {}

impl<T> Typecheck for Lambda<T>
where
    T: LanguageTerm,
    Fun<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        self.annot.check_kind(env)?;
        env.add_var(self.var.clone(), self.annot.clone());
        let body_ty = self.body.check(env)?;
        body_ty.check_kind(env)?;
        Ok(Fun::new(self.annot.clone(), body_ty).into())
    }
}

impl<T> Eval for Lambda<T>
where
    T: LanguageTerm,
    LambdaVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        Ok(LambdaVal::new(&self.var, self.annot, *self.body).into())
    }
}

impl<T> SubstTerm<T> for Lambda<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Lambda<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Lambda {
            var: self.var,
            annot: self.annot.subst_type(v, ty),
            body: Box::new(self.body.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> fmt::Display for Lambda<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ty_str = self.annot.to_string();
        if ty_str.is_empty() {
            write!(f, "\\{}.{}", self.var, self.body)
        } else {
            write!(f, "\\{}:{}.({})", self.var, ty_str, self.body)
        }
    }
}
