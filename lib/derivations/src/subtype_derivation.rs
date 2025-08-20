use crate::{Derivation, SubtypeConclusion, SubtypeRule};
use std::fmt;
use syntax::{
    env::Environment,
    kinds::Kind,
    terms::Term,
    types::{Bot, OpApp, Top, Type},
};

#[derive(Debug)]
pub struct SubtypeDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub conc: SubtypeConclusion<Ty>,
    pub label: SubtypeRule,
    pub premises: Vec<Derivation<T, Ty>>,
}

impl<T, Ty> SubtypeDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn ret_ty(&self) -> Ty {
        self.conc.sup.clone()
    }

    pub fn sub_top<Ty1>(env: Environment<Ty>, sub: Ty1, top_knd: Kind) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Top<Ty>: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, Top::new(top_knd)),
            label: SubtypeRule::Top,
            premises: vec![],
        }
    }

    pub fn refl<Ty1>(env: Environment<Ty>, ty: Ty1) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty> + Clone,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, ty.clone(), ty),
            label: SubtypeRule::Refl,
            premises: vec![],
        }
    }

    pub fn sup_bot<Ty1>(env: Environment<Ty>, sup: Ty1) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Bot<Ty>: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sup, Bot::new()),
            label: SubtypeRule::Bot,
            premises: vec![],
        }
    }

    pub fn exists_bounded<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        prem: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Exists,
            premises: vec![prem],
        }
    }

    pub fn forall_bounded<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        bound_deriv: Derivation<T, Ty>,
        inner_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::All,
            premises: vec![bound_deriv, inner_deriv],
        }
    }

    pub fn fun<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        from_deriv: Derivation<T, Ty>,
        to_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Fun,
            premises: vec![from_deriv, to_deriv],
        }
    }

    pub fn list<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::List,
            premises: vec![inner_deriv],
        }
    }

    pub fn op_app<Ty1, Ty2, Ty3>(
        env: Environment<Ty>,
        sub_fun: Ty1,
        sup_fun: Ty2,
        arg: Ty3,
        fun_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
        Ty3: Into<Ty> + Clone,
        OpApp<Ty>: Into<Ty>,
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
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Abs,
            premises: vec![inner_deriv],
        }
    }

    pub fn op_lambda_sub<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        body_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::AbsSub,
            premises: vec![body_deriv],
        }
    }

    pub fn record<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_derivs: Vec<Derivation<T, Ty>>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Record,
            premises: inner_derivs,
        }
    }

    pub fn ref_source<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::RefSource,
            premises: vec![inner_deriv],
        }
    }

    pub fn ref_sink<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::RefSink,
            premises: vec![inner_deriv],
        }
    }

    pub fn ref_ref<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Ref,
            premises: vec![inner_deriv],
        }
    }

    pub fn sink<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Sink,
            premises: vec![inner_deriv],
        }
    }

    pub fn source<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_deriv: Derivation<T, Ty>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Source,
            premises: vec![inner_deriv],
        }
    }

    pub fn variant<Ty1, Ty2>(
        env: Environment<Ty>,
        sub: Ty1,
        sup: Ty2,
        inner_derivs: Vec<Derivation<T, Ty>>,
    ) -> SubtypeDerivation<T, Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeDerivation {
            conc: SubtypeConclusion::new(env, sub, sup),
            label: SubtypeRule::Variant,
            premises: inner_derivs,
        }
    }
}

impl<T, Ty> From<SubtypeDerivation<T, Ty>> for Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(deriv: SubtypeDerivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation::SubtypeDerivation(deriv)
    }
}

impl<T, Ty> fmt::Display for SubtypeDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
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
