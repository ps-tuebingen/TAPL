use super::{terms::Term, types::Type};
use errors::{UnexpectedRule, parse_error::ParserError};
use parser::{GroupParse, Parse, Rule, pair_to_n_inner, terms::StringTerm, types::StringTy};
use pest::iterators::Pair;
use syntax::{
    terms::{
        App, Ascribe, Cons, Fix, Fst, Head, If, IsNil, IsZero, Lambda, Left, Let, Nil, Nothing,
        Num, Pair as PairT, Pred, Projection, Record, RecordProj, Right, Snd, SomeCase, Something,
        Succ, SumCase, Tail, Tuple, Variable, Variant, VariantCase,
    },
    types::{Fun, List, Product, Record as RecordTy, Sum, Tuple as TupleTy, Variant as VariantTy},
};

impl GroupParse for Term {
    const RULE: Rule = Rule::term;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Term, ParserError> {
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::new()
                .with_true()
                .with_false()
                .with_zero()
                .with_unit()
                .from_pair(p)?),
            Rule::variable => Ok(Variable::from_pair(p, ())?.into()),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::some_term => Ok(Something::from_pair(p, ())?.into()),
            Rule::none_term => Ok(Nothing::from_pair(p, ())?.into()),
            Rule::fix_term => Ok(Fix::from_pair(p, ())?.into()),
            Rule::if_term => Ok(If::from_pair(p, ())?.into()),
            Rule::let_term => Ok(Let::from_pair(p, ())?.into()),
            Rule::pair_term => Ok(PairT::from_pair(p, ())?.into()),
            Rule::tuple_term => Ok(Tuple::from_pair(p, ())?.into()),
            Rule::record_term => Ok(Record::from_pair(p, ())?.into()),
            Rule::left_term => Ok(Left::from_pair(p, ())?.into()),
            Rule::right_term => Ok(Right::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::variant_term => Ok(Variant::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::iszero_term => Ok(IsZero::from_pair(p, ())?.into()),
            Rule::cons_term => Ok(Cons::from_pair(p, ())?.into()),
            Rule::nil_term => Ok(Nil::from_pair(p, ())?.into()),
            Rule::isnil_term => Ok(IsNil::from_pair(p, ())?.into()),
            Rule::head_term => Ok(Head::from_pair(p, ())?.into()),
            Rule::tail_term => Ok(Tail::from_pair(p, ())?.into()),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            Rule::paren_term => {
                Self::from_pair(pair_to_n_inner(p, vec!["Paren Term Inner"])?.remove(0), ())
            }
            Rule::variantcase_term => Ok(VariantCase::from_pair(p, ())?.into()),
            Rule::sumcase_term => Ok(SumCase::from_pair(p, ())?.into()),
            Rule::somecase_term => Ok(SomeCase::from_pair(p, ())?.into()),
            r => Err(UnexpectedRule::new(&format!("{r:?}"), "Non Left-Recursive Term").into()),
        }
    }
    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, ParserError> {
        match p.as_rule() {
            Rule::ascription => Ok(Ascribe::from_pair(p, t)?.into()),
            Rule::projection => Ok(Projection::from_pair(p, t)?.into()),
            Rule::record_proj => Ok(RecordProj::from_pair(p, t)?.into()),
            Rule::fst_term => Ok(Fst::from_pair(p, t)?.into()),
            Rule::snd_term => Ok(Snd::from_pair(p, t)?.into()),
            Rule::term => Ok(App::from_pair(p, t)?.into()),
            r => {
                Err(UnexpectedRule::new(&format!("{r:?}"), "Ascription, Projection or Term").into())
            }
        }
    }
}

impl GroupParse for Type {
    const RULE: Rule = Rule::r#type;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Type, ParserError> {
        match p.as_rule() {
            Rule::const_type => Ok(StringTy::new()
                .with_bool()
                .with_nat()
                .with_unit()
                .from_pair(p)?),
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            Rule::prod_type => Ok(Product::from_pair(p, ())?.into()),
            Rule::record_type => Ok(RecordTy::from_pair(p, ())?.into()),
            Rule::sum_type => Ok(Sum::from_pair(p, ())?.into()),
            Rule::variant_type => Ok(VariantTy::from_pair(p, ())?.into()),
            Rule::tuple_type => Ok(TupleTy::from_pair(p, ())?.into()),
            Rule::list_type => Ok(List::from_pair(p, ())?.into()),
            r => Err(UnexpectedRule::new(&format!("{r:?}"), "Non Left-Recursive Type").into()),
        }
    }

    fn from_pair_leftrec(p: Pair<'_, Rule>, ty: Type) -> Result<Type, ParserError> {
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, ty)?.into()),
            _ => Err(
                UnexpectedRule::new(&format!("{:?}", p.as_rule()), "Left Recursive Type").into(),
            ),
        }
    }
}
