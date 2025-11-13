use super::buss_conc;
use crate::{LatexConfig, LatexFmt};
use derivations::SubtypeDerivation;
use syntax::language::Language;

impl<Lang> LatexFmt for SubtypeDerivation<Lang>
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        if conf.use_frac_array {
            derivation_to_frac_array(self, conf)
        } else {
            derivation_to_bussproofs(self, conf)
        }
    }
}

fn derivation_to_bussproofs<Lang>(deriv: &SubtypeDerivation<Lang>, conf: &mut LatexConfig) -> String
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
{
    let (env_start, env_end) = if conf.include_envs {
        ("\\begin{prooftree}", "\\end{prooftree}")
    } else {
        ("", "")
    };

    conf.include_envs = false;
    let conc_str = buss_conc(&deriv.conc.to_latex(conf), deriv.premises.len());

    let label_str = format!("\\RightLabel{{ {} }}\n", deriv.label);

    let start_str = if deriv.premises.is_empty() {
        "\\AxiomC{\\quad}\n"
    } else {
        ""
    };

    let mut prem_strs = vec![];
    for prem in &deriv.premises {
        prem_strs.push(prem.to_latex(conf));
        conf.include_envs = false;
    }

    format!(
        "{env_start}{start_str}\n{}\n{label_str}\n{conc_str}\n{env_end}",
        prem_strs.join("\n")
    )
}

fn derivation_to_frac_array<Lang>(deriv: &SubtypeDerivation<Lang>, conf: &mut LatexConfig) -> String
where
    Lang: Language,
    Lang::Type: LatexFmt,
    Lang::Term: LatexFmt,
{
    let (env_start, env_end) = conf.mathenv_strs();
    let mut prem_strs = vec![];

    for prem in &deriv.premises {
        conf.include_envs = false;
        prem_strs.push(format!("\\displaystyle {}", prem.to_latex(conf)));
    }
    if prem_strs.is_empty() {
        prem_strs.push("\\quad".to_owned());
    }

    let cs = (0..prem_strs.len()).map(|_| "c").collect::<Vec<&str>>();

    let array_start = format!("\\begin{{array}}{{ {} }}", cs.join(" "));
    let array_end = "\\end{array}\n";

    let array_str = format!("{array_start}\n{}\n{array_end}", prem_strs.join(" &\n "));

    conf.include_envs = false;
    let conc_str = deriv.conc.to_latex(conf);

    format!(
        "{env_start}\n\\frac{{\n{array_str}}}{{\n{conc_str}
        }}\n{env_end}"
    )
}
