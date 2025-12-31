use crate::{LatexConfig, LatexFmt};
use inference::constraints::{
    Constraint, EqualityConstraint, IndexConstraint, KindConstraint, RecordConstraint,
    SubtypeConstraint, VariantConstraint,
};
use syntax::language::Language;

mod definition;
mod program;

impl<Lang> LatexFmt for Constraint<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Self::Equality(eq) => eq.to_latex(conf),
            Self::Subtyping(sub) => sub.to_latex(conf),
            Self::Kinding(knd) => knd.to_latex(conf),
            Self::Indexing(ind) => ind.to_latex(conf),
            Self::Record(rec) => rec.to_latex(conf),
            Self::Variant(var) => var.to_latex(conf),
        }
    }
}

impl<Lang> LatexFmt for EqualityConstraint<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        let left_str = self.left.to_latex(conf);
        conf.include_envs = false;
        let right_str = self.right.to_latex(conf);
        format!("{env_start}{left_str} == {right_str}{env_end}")
    }
}

impl<Lang> LatexFmt for SubtypeConstraint<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        let sub_str = self.sub_type.to_latex(conf);
        conf.include_envs = false;
        let super_str = self.super_type.to_latex(conf);
        format!("{env_start}{sub_str} <: {super_str}{env_end}")
    }
}

impl LatexFmt for KindConstraint {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        let left_str = self.left.to_latex(conf);
        conf.include_envs = false;
        let right_str = self.right.to_latex(conf);
        format!("{env_start}{left_str} == {right_str}{env_end}")
    }
}

impl<Lang> LatexFmt for IndexConstraint<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        let ty_str = self.ty.to_latex(conf);
        conf.include_envs = false;
        let ind_ty_str = self.ind_ty.to_latex(conf);
        format!("{env_start}{ty_str}[{}] == {ind_ty_str}{env_end}", self.ind)
    }
}

impl<Lang> LatexFmt for RecordConstraint<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        let ty_str = self.ty.to_latex(conf);
        conf.include_envs = false;
        let label_ty_str = self.label_ty.to_latex(conf);
        format!(
            "{env_start}{ty_str} == \\{{ {} : {label_ty_str},\\dots \\}}{env_end}",
            self.label
        )
    }
}

impl<Lang> LatexFmt for VariantConstraint<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        let ty_str = self.ty.to_latex(conf);
        conf.include_envs = false;
        let label_ty_str = self.label_ty.to_latex(conf);
        format!(
            "{env_start}{ty_str} == \\langle {} : {label_ty_str},\\dots \\rangle{env_end}",
            self.label
        )
    }
}
