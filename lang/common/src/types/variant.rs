use super::Type;
use crate::{
    check::{to_subty_err, Subtypecheck},
    errors::{Error, ErrorKind},
    language::LanguageType,
    subst::SubstType,
    Label, TypeVar,
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<Ty>
where
    Ty: Type,
{
    pub variants: HashMap<Label, Ty>,
}

impl<Ty> Variant<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(vars: HashMap<Label, Ty1>) -> Variant<Ty>
    where
        Ty1: Into<Ty>,
    {
        Variant {
            variants: vars.into_iter().map(|(lb, ty)| (lb, ty.into())).collect(),
        }
    }

    pub fn new_single<Ty1>(lb: &str, ty: Ty1) -> Variant<Ty>
    where
        Ty1: Into<Ty>,
    {
        Variant {
            variants: HashMap::from([(lb.to_owned(), ty.into())]),
        }
    }
}

impl<Ty> Type for Variant<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Variant<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Variant {
            variants: self
                .variants
                .into_iter()
                .map(|(lb, ty1)| (lb, ty1.subst_type(v, ty)))
                .collect(),
        }
        .into()
    }
}

impl<Ty> Subtypecheck<Ty> for Variant<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sup.clone().into_top() {
            return Ok(());
        }

        let sup_var = sup.clone().into_variant().map_err(to_subty_err)?;
        for (lb, ty) in sup_var.variants.iter() {
            let self_ty = self
                .variants
                .get(lb)
                .ok_or(to_subty_err(ErrorKind::UndefinedLabel(lb.clone())))?;
            self_ty.check_subtype(ty, &mut env.clone())?;
        }
        Ok(())
    }
}

impl<Ty> fmt::Display for Variant<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vars: Vec<(&Label, &Ty)> = self.variants.iter().collect();
        vars.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "< {} >",
            vars.iter()
                .map(|(lb, ty)| format!("{lb} : {ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
