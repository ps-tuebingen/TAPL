use crate::{Rule, RuleDescribe, symbols::Keyword};
use syntax::types::{Bool, Type};

impl<Ty> RuleDescribe for Bool<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Keyword::Bool.into(), "Bool")
    }
}
