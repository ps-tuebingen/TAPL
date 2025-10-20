use crate::{Derivation, SubtypeConclusion, SubtypeRule};
use std::fmt;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    types::{Bot, OpApp, Top},
};

#[derive(Debug)]
pub struct SubtypeDerivation<Lang>
where
    Lang: Language,
{
    pub conc: SubtypeConclusion<Lang>,
    pub label: SubtypeRule,
    pub premises: Vec<Derivation<Lang>>,
}

impl<Lang> SubtypeDerivation<Lang>
where
    Lang: Language,
{
    pub fn ret_ty(&self) -> Lang::Type {
        self.conc.sup.clone()
    }

    pub fn sub_top<Ty1>(
        env: Environment<Lang>,
        sub: Ty1,
        top_knd: Kind,
        premises: Vec<Derivation<Lang>>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Top<Lang>: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, Top::new(top_knd)),
            label: SubtypeRule::Top,
            premises,
        }
    }

    pub fn refl<Ty1>(
        env: Environment<Lang>,
        ty: Ty1,
        premises: Vec<Derivation<Lang>>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type> + Clone,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, ty.clone(), ty),
            label: SubtypeRule::Refl,
            premises,
        }
    }

    pub fn sup_bot<Ty1>(env: Environment<Lang>, sup: Ty1) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Bot<Lang>: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sup, Bot::new()),
            label: SubtypeRule::Bot,
            premises: vec![],
        }
    }

    pub fn exists_bounded<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        premises: Vec<Derivation<Lang>>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Exists,
            premises,
        }
    }

    pub fn forall_bounded<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        premises: Vec<Derivation<Lang>>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::All,
            premises: premises,
        }
    }

    pub fn fun<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        from_deriv: Derivation<Lang>,
        to_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Fun,
            premises: vec![from_deriv, to_deriv],
        }
    }

    pub fn list<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::List,
            premises: vec![inner_deriv],
        }
    }

    pub fn op_app<Ty1, Ty2, Ty3>(
        env: Environment<Lang>,
        sub_fun: Ty1,
        sup_fun: Ty2,
        arg: Ty3,
        fun_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
        Ty3: Into<Lang::Type> + Clone,
        OpApp<Lang>: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(
                env,
                OpApp::new(sub_fun, arg.clone()),
                OpApp::new(sup_fun, arg),
            ),
            label: SubtypeRule::App,
            premises: vec![fun_deriv],
        }
    }

    pub fn op_lambda<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Abs,
            premises: vec![inner_deriv],
        }
    }

    pub fn op_lambda_sub<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        premises: Vec<Derivation<Lang>>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::AbsSub,
            premises,
        }
    }

    pub fn record<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_derivs: Vec<Derivation<Lang>>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Record,
            premises: inner_derivs,
        }
    }

    pub fn ref_source<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::RefSource,
            premises: vec![inner_deriv],
        }
    }

    pub fn ref_sink<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::RefSink,
            premises: vec![inner_deriv],
        }
    }

    pub fn ref_ref<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Ref,
            premises: vec![inner_deriv],
        }
    }

    pub fn sink<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Sink,
            premises: vec![inner_deriv],
        }
    }

    pub fn source<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<Lang>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Source,
            premises: vec![inner_deriv],
        }
    }

    pub fn variant<Ty1, Ty2>(
        env: Environment<Lang>,
        sub: Ty1,
        sup: Ty2,
        inner_derivs: Vec<Derivation<Lang>>,
    ) -> SubtypeDerivation<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Variant,
            premises: inner_derivs,
        }
    }
}

impl<Lang> From<SubtypeDerivation<Lang>> for Derivation<Lang>
where
    Lang: Language,
{
    fn from(deriv: SubtypeDerivation<Lang>) -> Derivation<Lang> {
        Derivation::SubtypeDerivation(deriv)
    }
}

impl<Lang> fmt::Display for SubtypeDerivation<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prem in self.premises.iter() {
            writeln!(f, "{prem}")?;
            writeln!(f,)?;
        }
        writeln!(f, "=== {} ===", self.label)?;
        write!(f, "{}", self.conc)
    }
}
