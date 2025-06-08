use super::super::LatexFmt;
use syntax::terms::{Ref, Term};

impl<T> LatexFmt for Ref<T>
where
    T: Term + LatexFmt,
{
    fn to_latex(&self) -> String {
        format!("ref({})", self.term.to_latex())
    }
}
