use super::{LatexConfig, LatexFmt};
use derivation::Derivation;
use syntax::{terms::Term, types::Type};

impl<T, Ty> LatexFmt for Derivation<T, Ty>
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        if conf.use_frac_array {
            derivation_to_frac_array(self, conf)
        } else {
            derivation_to_bussproofs(self, conf)
        }
    }
}

fn derivation_to_bussproofs<T, Ty>(deriv: &Derivation<T, Ty>, conf: &mut LatexConfig) -> String
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    let conc_str = match deriv.premises.len() {
        0 => format!("\\UnaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        1 => format!("\\UnaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        2 => format!("\\BinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        3 => format!("\\TrinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        4 => format!("\\QuaternaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        5 => format!("\\QuinaryInfC{{${}$}}", deriv.conc.to_latex(conf)),
        _ => panic!("Derivations with more than 5 premises are not supported"),
    };

    let mut out = "".to_owned();

    if conf.include_envs {
        out += "\\begin{prooftree}\n";
    }

    if deriv.premises.is_empty() {
        out += "\\AxiomC{\\quad}\n";
    } else {
        let old_inc = conf.include_envs;
        conf.include_envs = false;
        for prem in deriv.premises.iter() {
            out += "\t";
            out += &derivation_to_bussproofs(prem, conf);
            out += "\n";
        }
        conf.include_envs = old_inc;
    }

    out += "\\RightLabel{";
    out += &deriv.label.to_string();
    out += "}\n";
    out += &conc_str;
    if conf.include_envs {
        out += "\n\\end{prooftree}";
    }

    out
}

fn derivation_to_frac_array<T, Ty>(deriv: &Derivation<T, Ty>, conf: &mut LatexConfig) -> String
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    let mut premise_str;
    if deriv.premises.is_empty() {
        premise_str = "\\quad".to_owned();
    } else {
        let cs = (0..deriv.premises.len())
            .map(|_| "c")
            .collect::<Vec<&str>>();
        premise_str = format!("\\begin{{array}}{{ {} }}", cs.join(" "));
        let inc_old = conf.include_envs;
        conf.include_envs = false;

        let mut premise_strs = vec![];
        for premise in deriv.premises.iter() {
            premise_strs.push(format!(
                "\\displaystyle {}",
                derivation_to_frac_array(premise, conf)
            ));
        }
        premise_str += &premise_strs.join("&");
        conf.include_envs = inc_old;
        premise_str += "\\end{array}";
    }

    let mut out = format!(
        "\n\\frac{{ \\displaystyle {} }}{{ {} }} \\quad \\text{{ {} }}\n",
        premise_str,
        deriv.conc.to_latex(conf),
        deriv.label
    );
    if conf.include_envs {
        out = format!("\\[{out}\\]");
    }
    out
}
