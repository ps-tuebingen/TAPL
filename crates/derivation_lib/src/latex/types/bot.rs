use super::super::{LatexConfig, LatexFmt};
use syntax::types::Bot;

impl LatexFmt for Bot {
    fn to_latex(&self, _: &mut LatexConfig) -> String {
        format!("Bot")
    }
}
