use super::{terms::Term, types::Type};
use errors::{UnexpectedRule, parse_error::ParserError};
use parse::{
    GroupParse, Parse, Rule, pair_to_n_inner, sugar::ForallUnbounded, terms::StringTerm,
    types::StringTy,
};
use pest::iterators::Pair;
use syntax::{
    terms::{App, Lambda, Num, TyApp, TyLambda, Variable},
    types::{Forall, Fun, OpApp, OpLambda, TypeVariable},
};

impl GroupParse for Term {
    const RULE: Rule = Rule::term;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Term, ParserError> {
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::new()
                .with_unit()
                .with_true()
                .with_false()
                .from_pair(p)?),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::ty_lambda_kinded_term => Ok(TyLambda::from_pair(p, ())?.into()),
            Rule::paren_term => Self::from_pair(pair_to_n_inner(p, vec!["Term"])?.remove(0), ()),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            Rule::variable => Ok(Variable::from_pair(p, ())?.into()),
            r => Err(UnexpectedRule::new(&format!("{:?}", r), "Non Left-Recusrive Term").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, ParserError> {
        match p.as_rule() {
            Rule::tyapp => Ok(TyApp::from_pair(p, t)?.into()),
            Rule::term => Ok(App::from_pair(p, t)?.into()),
            r => Err(UnexpectedRule::new(&format!("{:?}", r), "Application").into()),
        }
    }
}
impl GroupParse for Type {
    const RULE: Rule = Rule::r#type;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Type, ParserError> {
        match p.as_rule() {
            Rule::const_type => Ok(StringTy::new()
                .with_unit()
                .with_nat()
                .with_bool()
                .from_pair(p)?),
            Rule::forall_unbounded_type => {
                Ok(ForallUnbounded::from_pair(p, ())?.to_forall_kinded().into())
            }
            Rule::forall_kinded_type => Ok(Forall::from_pair(p, ())?.into()),
            Rule::op_lambda_type => Ok(OpLambda::from_pair(p, ())?.into()),
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            Rule::type_variable => Ok(TypeVariable::from_pair(p, ())?.into()),
            r => Err(UnexpectedRule::new(&format!("{:?}", r), "Non Left-Recursive Type").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, ty: Type) -> Result<Type, ParserError> {
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, ty)?.into()),
            Rule::r#type => Ok(OpApp::from_pair(p, ty)?.into()),
            r => Err(UnexpectedRule::new(&format!("{:?}", r), "Left Recursive Type").into()),
        }
    }
}
