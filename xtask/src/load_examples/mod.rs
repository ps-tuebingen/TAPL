use errors::{DirAccess, FileAccess, build_error::BuildError};
use std::{fs::read_dir, path::PathBuf};

mod examples_mod;
mod examples_rs;
use examples_mod::generate_examples_mod;
use examples_rs::generate_examples_rs;

const EXAMPLES_PATH: &str = "examples";
const OUT_PATH: &str = "lib/web/src/examples";

fn load_paths(dir_name: &PathBuf) -> Result<Vec<PathBuf>, BuildError> {
    let base_dir = read_dir(dir_name).map_err(|err| DirAccess::new("read examples dir", err))?;
    let mut paths = vec![];
    for dir in base_dir {
        let dir = dir.map_err(|err| DirAccess::new("read example dir", err))?;
        paths.push(dir.path());
    }
    Ok(paths)
}

fn get_path_stems(paths: &[PathBuf]) -> Result<Vec<&str>, BuildError> {
    let mut stems = vec![];
    let err_fun = |path: &PathBuf, tried: &str, err: &str| {
        if path.is_file() {
            <FileAccess as Into<BuildError>>::into(FileAccess::new(tried, err))
        } else {
            <DirAccess as Into<BuildError>>::into(DirAccess::new(tried, err))
        }
    };

    for path in paths.iter() {
        let path_stem = path.file_stem().ok_or(err_fun(
            path,
            "get example file stem",
            "Could not get file stem",
        ))?;
        stems.push(path_stem.to_str().ok_or(err_fun(
            path,
            "get example file stem as str",
            "Could not get file stem as str",
        ))?);
    }
    Ok(stems)
}

pub fn load_examples() -> Result<(), BuildError> {
    let example_paths = load_paths(&PathBuf::from(EXAMPLES_PATH))?;
    generate_examples_rs(&example_paths)?;
    generate_examples_mod(&example_paths)
}
