use errors::{DirAccess, FileAccess, build_error::BuildError};
use std::{
    fs::{File, read_dir, remove_file},
    io::Write,
    path::PathBuf,
};

const EXAMPLES_PATH: &str = "examples";
const OUT_PATH: &str = "apps/web/src/examples";

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

fn load_paths() -> Result<Vec<PathBuf>, BuildError> {
    let examples_dir =
        read_dir(EXAMPLES_PATH).map_err(|err| DirAccess::new("read examples dir", err))?;
    let mut paths = vec![];
    for dir in examples_dir {
        let dir = dir.map_err(|err| DirAccess::new("read example dir", err))?;
        paths.push(dir.path());
    }
    Ok(paths)
}

fn get_path_stems(paths: &[PathBuf]) -> Result<Vec<&str>, BuildError> {
    let mut stems = vec![];
    for path in paths.iter() {
        let path_stem = path.file_stem().ok_or(DirAccess::new(
            "get example dir name",
            "Could not get file stem",
        ))?;
        stems.push(path_stem.to_str().ok_or(DirAccess::new(
            "get example dir name",
            "Could not get file stem as string",
        ))?);
    }
    Ok(stems)
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
    remove_file(&mod_path).map_err(|err| FileAccess::new("remove old examples/mod.rs", err))?;
    let mut mod_file =
        File::create(mod_path).map_err(|err| FileAccess::new("create new examples/mod.rs", err))?;
    mod_file
        .write_all(examples_mod(mod_strs, hashmap_strs).as_bytes())
        .map_err(|err| FileAccess::new("write examples/mod.rs", err))?;
    Ok(())
}

pub fn load_examples() -> Result<(), BuildError> {
    let example_paths = load_paths()?;
    let path_stems = get_path_stems(&example_paths)?;
    let mod_strs = get_mod_strs(&path_stems);
    let hashmap_strs = get_hashmap_strs(&path_stems);
    write_examples_mod(&mod_strs, &hashmap_strs)
}
