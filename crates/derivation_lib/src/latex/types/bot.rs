use super::super::LatexFmt;
use syntax::types::Bot;

impl LatexFmt for Bot {
    fn to_latex(&self) -> String {
        format!("Bot")
    }
}
