use crate::ProgTypes;
use std::{collections::HashMap, fmt};
use syntax::{Name, TypeVar, language::Language, subst::SubstType};

#[derive(Clone, Debug)]
pub struct DefSubst<Lang>
where
    Lang: Language,
{
    pub ty_vars: HashMap<TypeVar, Lang::Type>,
    pub ty_no_subst: Lang::Type,
}

#[derive(Clone, Debug)]
pub struct ProgSubst<Lang>
where
    Lang: Language,
{
    pub def_substs: HashMap<Name, DefSubst<Lang>>,
    pub main_subst: DefSubst<Lang>,
}

impl<Lang> DefSubst<Lang>
where
    Lang: Language,
{
    pub fn apply(self) -> Lang::Type {
        self.ty_no_subst
            .subst_sim(self.ty_vars.iter().collect::<Vec<_>>().as_slice())
    }
}

impl<Lang> ProgSubst<Lang>
where
    Lang: Language,
{
    pub fn apply(self) -> ProgTypes<Lang> {
        let mut def_tys = HashMap::new();
        for (name, subst) in self.def_substs {
            let ty = subst.apply();
            def_tys.insert(name, ty);
        }
        ProgTypes {
            main_ty: self.main_subst.apply(),
            def_tys,
        }
    }
}

impl<Lang> fmt::Display for ProgSubst<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, subst) in &self.def_substs {
            writeln!(f, "{name}:\n{subst}")?;
        }
        writeln!(f)?;
        writeln!(f, "{}", self.main_subst)
    }
}

impl<Lang> fmt::Display for DefSubst<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (v, ty) in &self.ty_vars {
            writeln!(f, "{v} --> {ty}")?;
        }
        writeln!(f)?;
        writeln!(f, "Final Type: {}", self.ty_no_subst)
    }
}
