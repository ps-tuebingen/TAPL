use super::{Subtypes, terms::Term, types::Type};
use errors::{UnexpectedRule, parse_error::ParserError};
use parser::{
    GroupParse, Parse, Rule, pair_to_n_inner,
    sugar::{Sequence, TopStar},
    terms::StringTerm,
    types::StringTy,
};
use pest::iterators::Pair;
use syntax::{
    terms::{
        App, Assign, Cast, Cons, Deref, Fix, If, Lambda, Let, ListCase, Nil, Num, Pred, Record,
        RecordProj, Ref, Succ, Variable, Variant, VariantCase,
    },
    types::{Fun, List, Record as RecordTy, Reference, Sink, Source, Top, Variant as VariantTy},
};

impl GroupParse for Term {
    const RULE: Rule = Rule::term;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Self, ParserError> {
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::<Subtypes>::new()
                .with_unit()
                .with_true()
                .with_false()
                .with_zero()
                .from_pair(p)?),
            Rule::variable => Ok(Variable::from_pair(p, ())?.into()),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::record_term => Ok(Record::from_pair(p, ())?.into()),
            Rule::variant_term => Ok(Variant::from_pair(p, ())?.into()),
            Rule::variantcase_term => Ok(VariantCase::from_pair(p, ())?.into()),
            Rule::nil_term => Ok(Nil::from_pair(p, ())?.into()),
            Rule::cons_term => Ok(Cons::from_pair(p, ())?.into()),
            Rule::listcase_term => Ok(ListCase::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::ref_term => Ok(Ref::from_pair(p, ())?.into()),
            Rule::deref_term => Ok(Deref::from_pair(p, ())?.into()),
            Rule::if_term => Ok(If::from_pair(p, ())?.into()),
            Rule::let_term => Ok(Let::from_pair(p, ())?.into()),
            Rule::fix_term => Ok(Fix::from_pair(p, ())?.into()),
            Rule::paren_term => Self::from_pair(pair_to_n_inner(p, vec!["Term"])?.remove(0), ()),
            r => Err(UnexpectedRule::new(&format!("{r:?}"), "Non Left-recursive term").into()),
        }
    }
    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Self, ParserError> {
        match p.as_rule() {
            Rule::record_proj => Ok(RecordProj::from_pair(p, t)?.into()),
            Rule::assign => Ok(Assign::from_pair(p, t)?.into()),
            Rule::sequence => Ok(Sequence::<Subtypes>::from_pair(p, t)?.to_term()),
            Rule::cast => Ok(Cast::from_pair(p, t)?.into()),
            Rule::term => Ok(App::from_pair(p, t)?.into()),
            r => Err(UnexpectedRule::new(&format!("{r:?}"), "Left Recursive Term").into()),
        }
    }
}
impl GroupParse for Type {
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Type, ParserError> {
        match p.as_rule() {
            Rule::const_type => Ok(StringTy::<Subtypes>::new()
                .with_bot()
                .with_nat()
                .with_unit()
                .with_bool()
                .from_pair(p)?),
            Rule::top_type_star => Ok(TopStar::from_pair(p, ())?.to_top().into()),
            Rule::top_type => Ok(Top::from_pair(p, ())?.into()),
            Rule::record_type => Ok(RecordTy::from_pair(p, ())?.into()),
            Rule::variant_type => Ok(VariantTy::from_pair(p, ())?.into()),
            Rule::list_type => Ok(List::from_pair(p, ())?.into()),
            Rule::ref_type => Ok(Reference::from_pair(p, ())?.into()),
            Rule::sink_type => Ok(Sink::from_pair(p, ())?.into()),
            Rule::source_type => Ok(Source::from_pair(p, ())?.into()),
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            r => Err(UnexpectedRule::new(
                &format!("{r:?}"),
                &format!("Non Left-Recursive Type ({p:?})",),
            )
            .into()),
        }
    }
    fn from_pair_leftrec(p: Pair<'_, Rule>, ty: Type) -> Result<Type, ParserError> {
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, ty)?.into()),
            r => Err(UnexpectedRule::new(&format!("{r:?}"), "Left Recursive Type").into()),
        }
    }
}
