use super::{Exceptions, types::Type};
use latex::{LatexConfig, LatexFmt};
use macros::{Eval, GrammarDescribe, LangDisplay, Typecheck};
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Exception, False, If, IsZero, Lambda, Num, Pred, Raise, Succ, True, Try, TryWithVal,
        Unit, Variable,
    },
};

#[derive(LangDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq)]
#[Lang(Exceptions)]
pub enum Term {
    Variable(Variable<Exceptions>),
    Num(Num<Exceptions>),
    True(True<Exceptions>),
    False(False<Exceptions>),
    Succ(Succ<Exceptions>),
    Pred(Pred<Exceptions>),
    IsZero(IsZero<Exceptions>),
    If(If<Exceptions>),
    Lambda(Lambda<Exceptions>),
    App(App<Exceptions>),
    Unit(Unit<Exceptions>),
    Exception(Exception<Exceptions>),
    Try(Try<Exceptions>),
    Raise(Raise<Exceptions>),
    TryWithVal(TryWithVal<Exceptions>),
}

impl syntax::terms::Term for Term {}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Variable(v) => v.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::Succ(s) => s.to_latex(conf),
            Term::Pred(p) => p.to_latex(conf),
            Term::IsZero(isz) => isz.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Unit(u) => u.to_latex(conf),
            Term::Exception(exc) => exc.to_latex(conf),
            Term::Try(t) => t.to_latex(conf),
            Term::Raise(r) => r.to_latex(conf),
            Term::TryWithVal(t) => t.to_latex(conf),
        }
    }
}

impl SubstTerm for Term {
    type Lang = Exceptions;
    type Target = Term;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst(v, t),
            Term::Num(num) => num.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::Succ(succ) => succ.subst(v, t).into(),
            Term::Pred(pred) => pred.subst(v, t).into(),
            Term::IsZero(isz) => isz.subst(v, t).into(),
            Term::If(ift) => ift.subst(v, t).into(),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::Unit(u) => u.subst(v, t).into(),
            Term::Exception(exc) => exc.subst(v, t).into(),
            Term::Try(tryt) => tryt.subst(v, t).into(),
            Term::Raise(raise) => raise.subst(v, t).into(),
            Term::TryWithVal(tryval) => tryval.subst(v, t).into(),
        }
    }
}

impl SubstType for Term {
    type Lang = Exceptions;
    type Target = Term;
    fn subst_type(self, v: &TypeVar, t: &Type) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst_type(v, t).into(),
            Term::Num(num) => num.subst_type(v, t).into(),
            Term::True(tru) => tru.subst_type(v, t).into(),
            Term::False(fls) => fls.subst_type(v, t).into(),
            Term::Succ(succ) => succ.subst_type(v, t).into(),
            Term::Pred(pred) => pred.subst_type(v, t).into(),
            Term::IsZero(isz) => isz.subst_type(v, t).into(),
            Term::If(ift) => ift.subst_type(v, t).into(),
            Term::Lambda(lam) => lam.subst_type(v, t).into(),
            Term::App(app) => app.subst_type(v, t).into(),
            Term::Unit(u) => u.subst_type(v, t).into(),
            Term::Exception(exc) => exc.subst_type(v, t).into(),
            Term::Try(tryt) => tryt.subst_type(v, t).into(),
            Term::Raise(raise) => raise.subst_type(v, t).into(),
            Term::TryWithVal(tryval) => tryval.subst_type(v, t).into(),
        }
    }
}

impl From<Variable<Exceptions>> for Term {
    fn from(var: Variable<Exceptions>) -> Term {
        Term::Variable(var)
    }
}

impl From<Num<Exceptions>> for Term {
    fn from(num: Num<Exceptions>) -> Term {
        Term::Num(num)
    }
}

impl From<True<Exceptions>> for Term {
    fn from(tru: True<Exceptions>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Exceptions>> for Term {
    fn from(fls: False<Exceptions>) -> Term {
        Term::False(fls)
    }
}

impl From<Succ<Exceptions>> for Term {
    fn from(succ: Succ<Exceptions>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred<Exceptions>> for Term {
    fn from(pred: Pred<Exceptions>) -> Term {
        Term::Pred(pred)
    }
}

impl From<IsZero<Exceptions>> for Term {
    fn from(isz: IsZero<Exceptions>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<If<Exceptions>> for Term {
    fn from(ift: If<Exceptions>) -> Term {
        Term::If(ift)
    }
}

impl From<Lambda<Exceptions>> for Term {
    fn from(lam: Lambda<Exceptions>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<Exceptions>> for Term {
    fn from(app: App<Exceptions>) -> Term {
        Term::App(app)
    }
}

impl From<Unit<Exceptions>> for Term {
    fn from(unit: Unit<Exceptions>) -> Term {
        Term::Unit(unit)
    }
}

impl From<Exception<Exceptions>> for Term {
    fn from(exc: Exception<Exceptions>) -> Term {
        Term::Exception(exc)
    }
}

impl From<Try<Exceptions>> for Term {
    fn from(tryt: Try<Exceptions>) -> Term {
        Term::Try(tryt)
    }
}

impl From<Raise<Exceptions>> for Term {
    fn from(raise: Raise<Exceptions>) -> Term {
        Term::Raise(raise)
    }
}

impl From<TryWithVal<Exceptions>> for Term {
    fn from(tryval: TryWithVal<Exceptions>) -> Term {
        Term::TryWithVal(tryval)
    }
}

#[cfg(test)]
pub mod term_tests {
    use super::{App, Exceptions, Lambda, Raise, Term, Try, TryWithVal, Unit, Variable};
    use syntax::types::Unit as UnitTy;

    pub fn example_term1() -> Term {
        Try::<Exceptions>::new(
            App::<Exceptions>::new(
                Lambda::<Exceptions>::new("x", UnitTy::new(), Variable::<Exceptions>::new("x")),
                Unit::<Exceptions>::new(),
            ),
            Unit::<Exceptions>::new(),
        )
        .into()
    }

    pub fn example_term2() -> Term {
        TryWithVal::<Exceptions>::new(
            Raise::<Exceptions>::new(Unit::<Exceptions>::new(), UnitTy::new(), UnitTy::new()),
            Lambda::<Exceptions>::new("x", UnitTy::new(), Unit::new()),
        )
        .into()
    }
}
#[cfg(test)]
mod eval_tests {
    use super::super::terms::term_tests::{example_term1, example_term2};
    use eval::Eval;
    use syntax::values::Unit;

    #[test]
    fn eval1() {
        let result = example_term1().eval_start().unwrap();
        let expected = Unit::new().into();
        assert_eq!(result.val(), expected)
    }

    #[test]
    fn eval2() {
        let result = example_term2().eval_start().unwrap();
        let expected = Unit::new().into();
        assert_eq!(result.val(), expected)
    }
}
