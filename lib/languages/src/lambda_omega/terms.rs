use super::{LambdaOmega, types::Type};
use latex::{LatexConfig, LatexFmt};
use macros::{Eval, GrammarDescribe, LangDisplay, Typecheck};
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{App, False, Lambda, Num, True, TyApp, TyLambda, Unit, Variable},
};

#[derive(LangDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq)]
#[Lang(LambdaOmega)]
pub enum Term {
    Variable(Variable<LambdaOmega>),
    Num(Num<LambdaOmega>),
    True(True<LambdaOmega>),
    False(False<LambdaOmega>),
    Unit(Unit<LambdaOmega>),
    Lambda(Lambda<LambdaOmega>),
    TyLambda(TyLambda<LambdaOmega>),
    App(App<LambdaOmega>),
    TyApp(TyApp<LambdaOmega>),
}

impl syntax::terms::Term for Term {}

impl SubstType for Term {
    type Lang = LambdaOmega;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst_type(v, ty).into(),
            Term::Num(num) => num.subst_type(v, ty).into(),
            Term::True(tru) => tru.subst_type(v, ty).into(),
            Term::False(fls) => fls.subst_type(v, ty).into(),
            Term::Lambda(lam) => lam.subst_type(v, ty).into(),
            Term::App(app) => app.subst_type(v, ty).into(),
            Term::Unit(u) => u.subst_type(v, ty).into(),
            Term::TyLambda(tylam) => tylam.subst_type(v, ty).into(),
            Term::TyApp(tyapp) => tyapp.subst_type(v, ty).into(),
        }
    }
}

impl SubstTerm for Term {
    type Lang = LambdaOmega;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst(v, t),
            Term::Num(num) => num.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::Unit(u) => u.subst(v, t).into(),
            Term::TyLambda(tylam) => tylam.subst(v, t).into(),
            Term::TyApp(tyapp) => tyapp.subst(v, t).into(),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Variable(var) => var.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Unit(u) => u.to_latex(conf),
            Term::TyLambda(tylam) => tylam.to_latex(conf),
            Term::TyApp(tyapp) => tyapp.to_latex(conf),
        }
    }
}

impl From<Variable<LambdaOmega>> for Term {
    fn from(var: Variable<LambdaOmega>) -> Term {
        Term::Variable(var)
    }
}

impl From<Num<LambdaOmega>> for Term {
    fn from(num: Num<LambdaOmega>) -> Term {
        Term::Num(num)
    }
}

impl From<True<LambdaOmega>> for Term {
    fn from(tru: True<LambdaOmega>) -> Term {
        Term::True(tru)
    }
}

impl From<False<LambdaOmega>> for Term {
    fn from(fls: False<LambdaOmega>) -> Term {
        Term::False(fls)
    }
}

impl From<Lambda<LambdaOmega>> for Term {
    fn from(lam: Lambda<LambdaOmega>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<LambdaOmega>> for Term {
    fn from(app: App<LambdaOmega>) -> Term {
        Term::App(app)
    }
}

impl From<Unit<LambdaOmega>> for Term {
    fn from(u: Unit<LambdaOmega>) -> Term {
        Term::Unit(u)
    }
}

impl From<TyLambda<LambdaOmega>> for Term {
    fn from(tylam: TyLambda<LambdaOmega>) -> Term {
        Term::TyLambda(tylam)
    }
}

impl From<TyApp<LambdaOmega>> for Term {
    fn from(tyapp: TyApp<LambdaOmega>) -> Term {
        Term::TyApp(tyapp)
    }
}
