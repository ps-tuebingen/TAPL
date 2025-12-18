use syntax::language::Language;

mod constraints;
pub use constraints::Constraint;

mod definition;
mod program;

pub trait GenerateConstraints {
    type Lang: Language;
    fn generate_constraints(&self) -> Vec<Constraint<Self::Lang>>;
}
