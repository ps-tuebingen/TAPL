use syntax::language::Language;

mod definition;
mod program;
mod state;
mod terms;
mod types;
mod untyped;
pub use program::generate_constraints_program;
pub use state::GenState;

pub trait GenerateConstraints {
    type Lang: Language;
    type Target;
    fn generate_constraints(&self, state: &mut GenState<Self::Lang>) -> Self::Target;
}
