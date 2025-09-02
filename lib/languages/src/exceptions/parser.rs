use super::{Exceptions, terms::Term, types::Type};
use errors::{UnexpectedRule, parse_error::ParserError};
use parser::{GroupParse, Parse, Rule, pair_to_n_inner, terms::StringTerm, types::StringTy};
use pest::iterators::Pair;
use syntax::{
    terms::{
        App, Exception, If, IsZero, Lambda, Num, Pred, Raise, Succ, Try, TryWithVal, Variable,
    },
    types::Fun,
};

impl GroupParse for Term {
    const RULE: Rule = Rule::term;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::<Exceptions>::new()
                .with_unit()
                .with_true()
                .with_false()
                .from_pair(p)?),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            Rule::variable => Ok(Variable::new(p.as_str().trim()).into()),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::iszero_term => Ok(IsZero::from_pair(p, ())?.into()),
            Rule::if_term => Ok(If::from_pair(p, ())?.into()),
            Rule::try_term => Ok(Try::from_pair(p, ())?.into()),
            Rule::try_catch => Ok(TryWithVal::from_pair(p, ())?.into()),
            Rule::raise_term => Ok(Raise::from_pair(p, ())?.into()),
            Rule::err_term => Ok(Exception::from_pair(p, ())?.into()),
            Rule::paren_term => Self::from_pair(pair_to_n_inner(p, vec!["Term"])?.remove(0), ()),
            r => Err(UnexpectedRule::new(&format!("{r:?}"), "Non Left-recursive Term").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, left_rec: Self) -> Result<Self, ParserError> {
        match p.as_rule() {
            Rule::term => Ok(App::from_pair(p, left_rec)?.into()),
            r => Err(UnexpectedRule::new(&format!("{r:?}"), "Term").into()),
        }
    }
}

impl GroupParse for Type {
    const RULE: Rule = Rule::r#type;

    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
        match p.as_rule() {
            Rule::const_type => Ok(StringTy::<Exceptions>::new()
                .with_unit()
                .with_nat()
                .with_bool()
                .from_pair(p)?),
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            r => Err(UnexpectedRule::new(&format!("{r:?}"), "Non Left-Recursive Type").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, left_rec: Self) -> Result<Self, ParserError> {
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, left_rec)?.into()),
            _ => Err(
                UnexpectedRule::new(&format!("{:?}", p.as_rule()), "Left Recursive Type").into(),
            ),
        }
    }
}
