use super::Type;
use crate::{
    TypeVar,
    free_vars::FreeTypeVars,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{collections::HashSet, fmt, marker::PhantomData};

/// Type Variable
#[derive(Clone, Debug, EqNoSpan)]
pub struct TypeVariable<Lang>
where
    Lang: Language,
{
    /// The Variable
    pub v: TypeVar,
    /// Source Location
    pub span: Span,
    /// Save the type  Parameter
    phantom: PhantomData<Lang>,
}

impl<Lang> TypeVariable<Lang>
where
    Lang: Language,
{
    /// Create a new Type variable with variable and span
    #[must_use]
    pub fn new(v: &str, span: Span) -> Self {
        Self {
            v: v.to_owned(),
            span,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Spanned for TypeVariable<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeTypeVars for TypeVariable<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        vars.insert(self.v.clone());
    }
}

impl<Lang> Type for TypeVariable<Lang> where Lang: Language {}

impl<Lang> SubstType for TypeVariable<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Lang::Type;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v == self.v {
            ty.clone()
        } else {
            self.into()
        }
    }
}

impl<Lang> fmt::Display for TypeVariable<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}
