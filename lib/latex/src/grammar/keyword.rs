use crate::{LatexConfig, LatexFmt};
use grammar::symbols::Keyword;

impl LatexFmt for Keyword {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        match self {
            Keyword::As => format!("{env_start} \\text{{ as }} {env_end}"),
            Keyword::In => format!("{env_start} \\text{{ in }} {env_end}"),
            Keyword::Let => format!("{env_start} \\text{{ let }}{env_end}"),
            Keyword::If => format!("{env_start} \\text{{ if }}{env_end}"),
            Keyword::Else => format!("{env_start} \\text{{ else }}{env_end}"),
            Keyword::Try => format!("{env_start}\\text{{ try }} {env_end}"),
            Keyword::Catch => format!("{env_start}\\text{{ catch }} {env_end}"),
            Keyword::Nil => format!("{env_start} \\text{{ nil }}{env_end}"),
            Keyword::Cons => format!("{env_start}\\text{{ cons }} {env_end}"),
            Keyword::Head => format!("{env_start}\\text{{ head }} {env_end}"),
            Keyword::Tail => format!("{env_start}\\text{{ tail }} {env_end}"),
            Keyword::IsNil => format!("{env_start}\\text{{ isnil }} {env_end}"),
            Keyword::Succ => format!("{env_start}\\text{{ succ }} {env_end}"),
            Keyword::Pred => format!("{env_start}\\text{{ pred }} {env_end}"),
            Keyword::IsZero => format!("{env_start}\\text{{ iszero }} {env_end}"),
            Keyword::Something => format!("{env_start}\\text{{ something }} {env_end}"),
            Keyword::Left => format!("{env_start}\\text{{ inl }} {env_end}"),
            Keyword::Right => format!("{env_start}\\text{{ inr }} {env_end}"),
            Keyword::Nothing => format!("{env_start}\\text{{ nothing }} {env_end}"),
            Keyword::Raise => format!("{env_start}\\text{{ raise }} {env_end}"),
            Keyword::Err => format!("{env_start}\\text{{ error }} {env_end}"),
            Keyword::Fold => format!("{env_start}\\text{{ fold }} {env_end}"),
            Keyword::Unfold => format!("{env_start}\\text{{ unfold }} {env_end}"),
            Keyword::Ref => format!("{env_start}\\text{{ ref }} {env_end}"),
            Keyword::Fix => format!("{env_start}\\text{{ fix }} {env_end}"),
            Keyword::Unit => format!("{env_start}\\text{{ unit }} {env_end}"),
            Keyword::True => format!("{env_start}\\text{{ true }} {env_end}"),
            Keyword::False => format!("{env_start}\\text{{ false }} {env_end}"),
            Keyword::Fst => format!("{env_start}\\text{{ fst }} {env_end}"),
            Keyword::Snd => format!("{env_start}\\text{{ snd }} {env_end}"),
            Keyword::Source => format!("{env_start}\\text{{ Source }} {env_end}"),
            Keyword::Sink => format!("{env_start}\\text{{ Sink }} {env_end}"),
            Keyword::Reference => format!("{env_start}\\text{{ Ref }} {env_end}"),
            Keyword::Optional => format!("{env_start}\\text{{ Optional }} {env_end}"),
            Keyword::List => format!("{env_start}\\text{{ List }} {env_end}"),
            Keyword::UnitTy => format!("{env_start}\\text{{ Unit }} {env_end}"),
            Keyword::Nat => format!("{env_start}\\text{{ Nat }} {env_end}"),
            Keyword::Bool => format!("{env_start}\\text{{ Bool }} {env_end}"),
            Keyword::Case => format!("{env_start}\\text{{ case }} {env_end}"),
            Keyword::Of => format!("{env_start} \\text{{ of }} {env_end}"),
        }
    }
}
