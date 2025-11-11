use super::{LatexConfig, LatexFmt};
use grammar::{ConclusionRule, DerivationRule};

impl LatexFmt for DerivationRule {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        if conf.use_frac_array {
            derivation_to_frac_array(self, conf)
        } else {
            derivation_to_buss(self, conf)
        }
    }
}

impl LatexFmt for ConclusionRule {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        let (env_start, env_end) = conf.mathenv_strs();
        conf.include_envs = false;
        format!(
            "{env_start}{} \\vdash {} {} {}{env_end}",
            self.env.to_latex(conf),
            self.input.to_latex(conf),
            self.separator.to_latex(conf),
            self.output.to_latex(conf)
        )
    }
}

fn derivation_to_frac_array(deriv: &DerivationRule, conf: &mut LatexConfig) -> String {
    let (env_start, env_end) = conf.mathenv_strs();
    conf.include_envs = false;

    let mut premise_strs = Vec::with_capacity(deriv.premises.len());
    for prem in deriv.premises.iter() {
        premise_strs.push(prem.to_latex(conf));
    }
    let cs = (0..=premise_strs.len()).map(|_| "c").collect::<Vec<_>>();
    let premise_str = format!(
        "\\begin{{array}}{{ {} }}\n {} \\end{{array}}",
        cs.join(" "),
        premise_strs.join("&")
    );
    let conc_str = deriv.conclusion.to_latex(conf);

    format!("{env_start}\n\\frac{{ {premise_str} }}{{ {conc_str} }}{env_end}",)
}

fn derivation_to_buss(deriv: &DerivationRule, conf: &mut LatexConfig) -> String {
    let (env_start, env_end) = if conf.include_envs {
        ("\\begin{prooftree}", "\\end{prooftree}")
    } else {
        ("", "")
    };
    conf.include_envs = false;

    let mut premise_strs = Vec::with_capacity(deriv.premises.len());
    for prem in deriv.premises.iter() {
        premise_strs.push(format!("\\AxiomC{{${}$}}", prem.to_latex(conf)));
        conf.include_envs = false;
    }

    let start_str = if deriv.premises.is_empty() {
        "\\AxiomC{\\quad}"
    } else {
        ""
    };

    let label_str = format!("\\RightLabel{{ {} }}", deriv.label);

    let mut conc_str = deriv.conclusion.to_latex(conf);
    conc_str = match deriv.premises.len() {
        0 => format!("\\UnaryInfC{{${conc_str}$}}",),
        1 => format!("\\UnaryInfC{{${conc_str}$}}",),
        2 => format!("\\BinaryInfC{{${conc_str}$}}",),
        3 => format!("\\TrinaryInfC{{${conc_str}$}}",),
        4 => format!("\\QuaternaryInfC{{${conc_str}$}}",),
        5 => format!("\\QuinaryInfC{{${conc_str}$}}",),
        _ => panic!("Derivations with more than 5 premises are not supported"),
    };

    format!(
        "{env_start}{start_str}{}\n{label_str}{conc_str}{env_end}",
        premise_strs.join("\n")
    )
}
