use super::{OUT_PATH, get_path_stems};
use errors::{FileAccess, build_error::BuildError};
use std::{
    fs::{File, remove_file},
    io::Write,
    path::PathBuf,
};

fn examples_mod(mod_strs: &[String], hashmap_strs: &[String]) -> String {
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
        hashmap_strs.join("\n")
    )
}

fn get_mod_strs(paths: &[&str]) -> Vec<String> {
    let mut mod_strs = vec![];
    for path in paths.iter() {
        mod_strs.push(format!("pub mod {};", path));
        mod_strs.push(format!("pub use {}::{}_all;", path, path));
        mod_strs.push("".to_owned());
    }
    mod_strs
}

fn get_hashmap_strs(paths: &[&str]) -> Vec<String> {
    let mut hashmap_strs = vec![];
    for path in paths.iter() {
        hashmap_strs.push(format!(
            "        (\"{}\", {}_all()),",
            path.replace('_', "-"),
            path
        ));
    }
    hashmap_strs
}

fn write_examples_mod(mod_strs: &[String], hashmap_strs: &[String]) -> Result<(), BuildError> {
    let mod_path = PathBuf::from(OUT_PATH).join("mod.rs");
    if mod_path.exists() {
        remove_file(&mod_path).map_err(|err| FileAccess::new("remove old examples/mod.rs", err))?;
    }
    let mut mod_file =
        File::create(mod_path).map_err(|err| FileAccess::new("create new examples/mod.rs", err))?;
    mod_file
        .write_all(examples_mod(mod_strs, hashmap_strs).as_bytes())
        .map_err(|err| FileAccess::new("write examples/mod.rs", err))?;
    Ok(())
}

pub fn generate_examples_mod(example_paths: &[PathBuf]) -> Result<(), BuildError> {
    let path_stems = get_path_stems(example_paths)?;
    let mod_strs = get_mod_strs(&path_stems);
    let hashmap_strs = get_hashmap_strs(&path_stems);
    write_examples_mod(&mod_strs, &hashmap_strs)
}
