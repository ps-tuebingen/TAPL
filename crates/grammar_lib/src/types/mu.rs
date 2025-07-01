use crate::{Rule, RuleDescribe, Symbol};
use syntax::types::{Mu, Type};

impl<Ty> RuleDescribe for Mu<Ty>
where
    Ty: Type,
{
    fn rule() -> Rule {
        Rule::new(Symbol::call("mu", 1, Symbol::Variable), "Mu Type")
    }
}
