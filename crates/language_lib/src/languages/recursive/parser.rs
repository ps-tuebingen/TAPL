use super::{errors::Error, terms::Term, types::Type};
use parse::{
    errors::UnexpectedRule, pair_to_n_inner, terms::StringTerm, types::StringTy, GroupParse, Parse,
    Rule,
};
use pest::iterators::Pair;
use syntax::{
    terms::{
        App, Fix, Fold, Fst, If, IsZero, Lambda, Let, Num, Pair as PairT, Pred, Record, RecordProj,
        Snd, Succ, Unfold, Variable, Variant, VariantCase,
    },
    types::{Fun, Mu, Product, Record as RecordTy, TypeVariable, Variant as VariantTy},
};

//mod terms;
//mod types;

impl GroupParse for Term {
    type ParseError = Error;

    const RULE: Rule = Rule::term;
    fn from_pair_nonrec(p: Pair<'_, Rule>) -> Result<Term, Error> {
        match p.as_rule() {
            Rule::const_term => Ok(StringTerm::new()
                .with_unit()
                .with_zero()
                .with_true()
                .with_false()
                .from_pair(p)?),
            Rule::lambda_term => Ok(Lambda::from_pair(p, ())?.into()),
            Rule::fold_term => Ok(Fold::from_pair(p, ())?.into()),
            Rule::unfold_term => Ok(Unfold::from_pair(p, ())?.into()),
            Rule::pair_term => Ok(PairT::from_pair(p, ())?.into()),
            Rule::variant_term => Ok(Variant::from_pair(p, ())?.into()),
            Rule::variantcase_term => Ok(VariantCase::from_pair(p, ())?.into()),
            Rule::succ_term => Ok(Succ::from_pair(p, ())?.into()),
            Rule::pred_term => Ok(Pred::from_pair(p, ())?.into()),
            Rule::iszero_term => Ok(IsZero::from_pair(p, ())?.into()),
            Rule::if_term => Ok(If::from_pair(p, ())?.into()),
            Rule::fix_term => Ok(Fix::from_pair(p, ())?.into()),
            Rule::let_term => Ok(Let::from_pair(p, ())?.into()),
            Rule::record_term => Ok(Record::from_pair(p, ())?.into()),
            Rule::paren_term => Self::from_pair(pair_to_n_inner(p, vec!["Term"])?.remove(0), ()),
            Rule::number => Ok(Num::from_pair(p, ())?.into()),
            Rule::variable => Ok(Variable::from_pair(p, ())?.into()),
            r => Err(UnexpectedRule::new(r, "Non Left-Recursive Term").into()),
        }
    }
    fn from_pair_leftrec(p: Pair<'_, Rule>, t: Term) -> Result<Term, Error> {
        match p.as_rule() {
            Rule::fst_term => Ok(Fst::from_pair(p, t)?.into()),
            Rule::snd_term => Ok(Snd::from_pair(p, t)?.into()),
            Rule::record_proj => Ok(RecordProj::from_pair(p, t)?.into()),
            Rule::term => Ok(App::from_pair(p, t)?.into()),
            r => Err(UnexpectedRule::new(r, "Left Recursive Term").into()),
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
            Rule::mu_type => Ok(Mu::from_pair(p, ())?.into()),
            Rule::prod_type => Ok(Product::from_pair(p, ())?.into()),
            Rule::variant_type => Ok(VariantTy::from_pair(p, ())?.into()),
            Rule::record_type => Ok(RecordTy::from_pair(p, ())?.into()),
            Rule::paren_type => Self::from_pair(pair_to_n_inner(p, vec!["Type"])?.remove(0), ()),
            Rule::type_variable => Ok(TypeVariable::from_pair(p, ())?.into()),
            r => Err(UnexpectedRule::new(r, "Non Left-Recursive Type").into()),
        }
    }
    fn from_pair_leftrec(p: Pair<'_, Rule>, ty: Type) -> Result<Type, Error> {
        match p.as_rule() {
            Rule::fun_type => Ok(Fun::from_pair(p, ty)?.into()),
            r => Err(UnexpectedRule::new(r, "Left Recursive Term").into()),
        }
    }
}
