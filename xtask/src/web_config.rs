use errors::{FileAccess, build_error::BuildError};
use languages::AllLanguages;
use std::{
    fs::{File, read_to_string},
    io::Write,
};

const CONF_PATH: &str = "web/build.conf";
const OUT_PATH: &str = "lib/web/src/web_langs.rs";

pub fn load_web_config() -> Result<(), BuildError> {
    let conf_contents =
        read_to_string(CONF_PATH).map_err(|err| FileAccess::new("Read web config", err))?;
    let langs = parse_conf(conf_contents)?;
    let out_str = langs_out(&langs);
    let mut out_file =
        File::create(OUT_PATH).map_err(|err| FileAccess::new("Write web config", err))?;
    out_file
        .write_all(out_str.as_bytes())
        .map_err(|err| FileAccess::new("Write web config", err))?;
    Ok(())
}

fn parse_conf(contents: String) -> Result<Vec<AllLanguages>, BuildError> {
    let mut langs = vec![];
    for line in contents.lines() {
        if line.starts_with("#") {
            continue;
        }
        let lang = line.trim().parse::<AllLanguages>()?;
        langs.push(lang);
    }
    Ok(langs)
}

fn langs_out(langs: &[AllLanguages]) -> String {
    format!(
        "//Automatically generated file, run `cargo run -p xtask` to regenerate\n
use languages::AllLanguages;
pub const WEB_LANGUAGES: [AllLanguages;{}] = [
\t{}
];

pub const WEB_LANGUAGES_TYPED: [AllLanguages;{}] = [
\t{}
];
        ",
        langs.len(),
        langs
            .iter()
            .map(|lang| format!("AllLanguages::{}", lang.name()))
            .collect::<Vec<_>>()
            .join(",\n\t"),
        langs.iter().filter(|lang| lang.is_typed()).count(),
        langs
            .iter()
            .filter(|lang| lang.is_typed())
            .map(|lang| format!("AllLanguages::{}", lang.name()))
            .collect::<Vec<_>>()
            .join(",\n\t")
    )
}
