use errors::{FileAccess, build_error::BuildError};
use languages::dispatch::{DispatchLanguage, create_dispatcher};
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

fn parse_conf(contents: String) -> Result<Vec<Box<dyn DispatchLanguage>>, BuildError> {
    let mut langs = vec![];
    for line in contents.lines() {
        if line.starts_with("#")
            || langs
                .iter()
                .any(|disp: &Box<dyn DispatchLanguage>| disp.is_lang(line.trim()))
        {
            continue;
        }
        let disp = create_dispatcher(line.trim())?;
        langs.push(disp);
    }
    Ok(langs)
}

fn langs_out(langs: &[Box<dyn DispatchLanguage>]) -> String {
    let mut names = Vec::with_capacity(langs.len());
    let mut num_typed = 0;
    let mut names_typed = Vec::with_capacity(langs.len());

    for disp in langs {
        let features = disp.features();
        let lang_id = format!("\"{}\"", disp.id());
        names.push(lang_id.clone());
        if features.typed() {
            num_typed += 1;
            names_typed.push(lang_id);
        }
    }

    format!(
        "//Automatically generated file, run `cargo run -p xtask` to regenerate\n
pub const WEB_LANGUAGES: [&str;{}] = [
\t{}
];

pub const WEB_LANGUAGES_TYPED: [&str;{}] = [
\t{}
];
        ",
        langs.len(),
        names.join(",\n\t"),
        num_typed,
        names_typed.join(",\n\t")
    )
}
