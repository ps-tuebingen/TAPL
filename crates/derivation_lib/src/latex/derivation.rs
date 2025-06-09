use super::{Derivation, LatexConfig, LatexFmt};
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

    if conf.include_tree_env {
        out += "\\begin{prooftree}\n";
    }

    if deriv.premises.is_empty() {
        out += "\\AxiomC{\\quad}\n";
    } else {
        let old_inc = conf.include_tree_env;
        conf.include_tree_env = false;
        for prem in deriv.premises.iter() {
            out += "\t";
            out += &derivation_to_bussproofs(prem, conf);
            out += "\n";
        }
        conf.include_tree_env = old_inc;
    }

    out += "\\RightLabel{";
    out += &deriv.label.to_string();
    out += "}\n";
    out += &conc_str;
    if conf.include_tree_env {
        out += "\n\\end{prooftree}";
    }

    out
}

fn derivation_to_frac_array<T, Ty>(deriv: &Derivation<T, Ty>, conf: &mut LatexConfig) -> String
where
    T: Term + LatexFmt,
    Ty: Type + LatexFmt,
{
    let mut premise_str = "".to_owned();
    if deriv.premises.len() == 0 {
        premise_str = "\\quad".to_owned();
    } else {
        let cs = (0..deriv.premises.len())
            .map(|_| "c")
            .collect::<Vec<&str>>();
        let mut premise_str = format!("\\begin{{array}}{{ {} }}", cs.join(" "));
        for premise in deriv.premises.iter() {
            premise_str += &derivation_to_frac_array(premise, conf);
            premise_str += "&";
        }
        premise_str += "\\end{array}";
    }

    format!(
        "\\frac{{ {} }}{{ {} }} \\quad \\text{{ {} }}",
        premise_str,
        deriv.conc.to_latex(conf),
        deriv.label
    )
}
