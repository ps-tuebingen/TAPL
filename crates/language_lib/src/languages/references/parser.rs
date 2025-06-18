use super::{errors::Error, terms::Term, types::Type};
use parse::{
    errors::UnexpectedRule, pair_to_n_inner, sugar::Sequence, terms::StringTerm, types::StringTy,
    GroupParse, Parse, Rule,
};
use pest::iterators::Pair;
use syntax::{
    terms::{App, Assign, Deref, Fix, If, IsZero, Lambda, Let, Num, Pred, Ref, Succ, Variable},
    types::{Fun, Reference as RefTy},
};

impl GroupParse for Term {
    type ParseError = Error;

    const RULE: Rule = Rule::term;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Term, Error> {
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::new()
                .with_unit()
                .with_true()
                .with_false()
                .from_pair(p)?),
            Rule::variable => Ok(Variable::from_pair(p, ())?.into()),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            Rule::paren_term => Self::from_pair(pair_to_n_inner(p, vec!["Term"])?.remove(0), ()),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::iszero_term => Ok(IsZero::from_pair(p, ())?.into()),
            Rule::ref_term => Ok(Ref::from_pair(p, ())?.into()),
            Rule::deref_term => Ok(Deref::from_pair(p, ())?.into()),
            Rule::let_term => Ok(Let::from_pair(p, ())?.into()),
            Rule::if_term => Ok(If::from_pair(p, ())?.into()),
            Rule::fix_term => Ok(Fix::from_pair(p, ())?.into()),
            r => Err(UnexpectedRule::new(r, "Term (non-left recursive)").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
        match p.as_rule() {
            Rule::assign => Ok(Assign::from_pair(p, t)?.into()),
            Rule::sequence => Ok(Sequence::from_pair(p, t)?.to_term()),
            Rule::term => Ok(App::from_pair(p, t)?.into()),
            r => Err(UnexpectedRule::new(r, "Assign or Application").into()),
        }
    }
}

impl GroupParse for Type {
    type ParseError = Error;
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Type, Error> {
        match p.as_rule() {
            Rule::const_type => Ok(StringTy::new()
                .with_unit()
                .with_nat()
                .with_bool()
                .from_pair(p)?),
            Rule::ref_type => Ok(RefTy::from_pair(p, ())?.into()),
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            _ => Err(UnexpectedRule::new(p.as_rule(), "Non Left-Recursive Type").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, ty)?.into()),
            _ => Err(UnexpectedRule::new(p.as_rule(), "Left Recursive Type").into()),
        }
    }
}
