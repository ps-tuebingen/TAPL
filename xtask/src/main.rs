use errors::{DirAccess, FileAccess, build_error::BuildError};
use std::{
    fs::{File, read_dir, remove_file},
    io::Write,
    path::PathBuf,
};

const EXAMPLES_PATH: &str = "examples";
const OUT_PATH: &str = "apps/web/src/examples";

fn examples_mod(mod_strs: Vec<String>, all_examples_strs: Vec<String>) -> String {
    format!(
        "use std::collections::HashMap;

{}
pub fn all_examples() -> HashMap<&'static str, Vec<(&'static str,&'static str)>> {{
    HashMap::from([
{}
    ])
}}
",
        mod_strs.join("\n"),
        all_examples_strs.join("\n")
    )
}

fn main() -> Result<(), BuildError> {
    let example_dirs =
        read_dir(EXAMPLES_PATH).map_err(|err| DirAccess::new("read examples dir", err))?;
    let mut examples_mod_strs = vec![];
    let mut all_examples_strs = vec![];

    for dir in example_dirs {
        let dir = dir.map_err(|err| DirAccess::new("read example dir", err))?;
        let dir_path = dir.path();
        let dir_stem = dir_path.file_stem().ok_or(DirAccess::new(
            "get example dir name",
            "Could not get file stem",
        ))?;
        let dir_stem_str = dir_stem.to_str().ok_or(DirAccess::new(
            "get example dir name as string",
            "Could not convert OSStr to str",
        ))?;
        examples_mod_strs.push(format!("pub mod {};", dir_stem_str));
        examples_mod_strs.push(format!("pub use {}::{}_all;", dir_stem_str, dir_stem_str));
        examples_mod_strs.push("".to_owned());

        all_examples_strs.push(format!(
            "        (\"{}\", {}_all()),",
            dir_stem_str.replace("_", "-"),
            dir_stem_str
        ));
    }

    let mod_path = PathBuf::from(OUT_PATH).join("mod.rs");
    remove_file(&mod_path).map_err(|err| FileAccess::new("remove old examples/mod.rs", err))?;
    let mut mod_file =
        File::create(mod_path).map_err(|err| FileAccess::new("create new examples/mod.rs", err))?;
    mod_file
        .write_all(examples_mod(examples_mod_strs, all_examples_strs).as_bytes())
        .map_err(|err| FileAccess::new("write examples/mod.rs", err))?;
    Ok(())
}
