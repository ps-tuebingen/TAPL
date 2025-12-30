use crate::constraints::Constraint;
use syntax::{Name, language::Language};

mod definition;
mod program;
mod state;
mod terms;
mod types;
mod untyped;
pub use program::generate_constraints_program;
pub use state::GenState;

pub struct DefConstraints<Lang>
where
    Lang: Language,
{
    pub name: Name,
    pub constraints: Vec<Constraint<Lang>>,
    pub ret_ty: Lang::Type,
}

pub trait GenerateConstraints {
    type Lang: Language;
    type Target;
    fn generate_constraints(&self, state: &mut GenState<Self::Lang>) -> Self::Target;
}
