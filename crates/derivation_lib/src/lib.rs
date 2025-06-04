use syntax::{terms::Term, types::Type};

pub mod conclusion;
pub mod rules;
pub use conclusion::Conclusion;
pub use rules::TypingRule;

pub struct Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    conc: Conclusion<T, Ty>,
    label: TypingRule,
    premises: Vec<Derivation<T, Ty>>,
}

impl<T, Ty> Derivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn app(
        conc: Conclusion<T, Ty>,
        left: Derivation<T, Ty>,
        right: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::App,
            premises: vec![left, right],
        }
    }

    pub fn ascribe(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Ascribe,
            premises: vec![prem],
        }
    }

    pub fn assign(
        conc: Conclusion<T, Ty>,
        left: Derivation<T, Ty>,
        right: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Assign,
            premises: vec![left, right],
        }
    }

    pub fn cast(conc: Conclusion<T, Ty>, prem: Derivation<T, Ty>) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Cast,
            premises: vec![prem],
        }
    }

    pub fn cons(
        conc: Conclusion<T, Ty>,
        head: Derivation<T, Ty>,
        tail: Derivation<T, Ty>,
    ) -> Derivation<T, Ty> {
        Derivation {
            conc,
            label: TypingRule::Cons,
            premises: vec![head, tail],
        }
    }
}
