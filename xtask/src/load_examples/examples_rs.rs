use super::{OUT_PATH, get_path_stems, load_paths};
use errors::{DirAccess, FileAccess, build_error::BuildError};
use std::{
    fs::{File, read_dir, remove_file},
    io::Write,
    path::PathBuf,
};

fn examples_rs(example_strs: &[String], vec_strs: &[String], path_stem: &str) -> String {
    format!(
        "//Automatically generated file, run `cargo run -p xtask` to regenerate\n{}
pub fn {}_all() -> Vec<(&'static str,&'static str)> {{
    vec![
{}
    ]
}}
",
        example_strs.join("\n"),
        path_stem,
        vec_strs.join("\n")
    )
}

fn get_example_file_name(base_path: &PathBuf) -> Result<String, BuildError> {
    let dir_contents =
        read_dir(base_path).map_err(|err| DirAccess::new("Read example dir", err))?;
    for dir_file in dir_contents {
        let dir_file = dir_file.map_err(|err| FileAccess::new("Read example dir", err))?;
        let file_path = dir_file.path();
        if file_path.extension().is_some_and(|nm| nm != "toml") {
            let file_name = file_path.file_name().ok_or(FileAccess::new(
                "Read example file name",
                "Could not get file name",
            ))?;
            return file_name
                .to_str()
                .ok_or(
                    FileAccess::new(
                        "Read example file name as string",
                        "Could not read file name as str",
                    )
                    .into(),
                )
                .map(|s| s.to_owned());
        }
    }
    Err(FileAccess::new("Find example", "Example file does not exist").into())
}

fn get_example_strs(examples: &[PathBuf], example_dir: &str) -> Result<Vec<String>, BuildError> {
    let mut example_strs = vec![];
    for example_path in examples.iter() {
        let example_file_name = get_example_file_name(example_path)?;
        let example_name = example_path.file_name().ok_or(FileAccess::new(
            "Read example name",
            "Could not get example name",
        ))?;
        let example_name_str = example_name.to_str().ok_or(FileAccess::new(
            "Read example name as string",
            "Could not get example name as str",
        ))?;

        example_strs.push(format!(
            "pub const {}: &str = include_str!(\"../../../../examples/{}/{}/{}\");",
            example_name_str.to_uppercase(),
            example_dir,
            example_name_str,
            example_file_name
        ));
        example_strs.push("".to_owned());
    }
    Ok(example_strs)
}

fn get_vec_strs(example_names: &[&str]) -> Vec<String> {
    let mut vec_strs = vec![];
    for example_name in example_names.iter() {
        let example_name_capitalized = example_name
            .chars()
            .enumerate()
            .map(|(ind, ch)| {
                if ind == 0 {
                    ch.to_ascii_uppercase()
                } else {
                    ch
                }
            })
            .collect::<String>();
        vec_strs.push(format!(
            "        (\"{}\", {}),",
            example_name_capitalized,
            example_name.to_uppercase()
        ));
    }
    vec_strs
}

fn write_example_rs(path_stem: &str, examples_str: &str) -> Result<(), BuildError> {
    let mut example_path = PathBuf::from(OUT_PATH).join(path_stem);
    example_path.set_extension("rs");
    if example_path.exists() {
        remove_file(&example_path).map_err(|err| {
            FileAccess::new(&format!("Remove old example file {:?}", example_path), err)
        })?;
    }
    let mut example_file = File::create(&example_path)
        .map_err(|err| FileAccess::new(&format!("Create example path {:?}", example_path), err))?;
    example_file
        .write_all(examples_str.as_bytes())
        .map_err(|err| FileAccess::new(&format!("Write example file {:?}", example_path), err))?;
    Ok(())
}

pub fn generate_examples_rs(paths: &[PathBuf]) -> Result<(), BuildError> {
    for path in paths.iter() {
        let path_stem = path.file_stem().ok_or(DirAccess::new(
            "Read examples path stem",
            "Could not get file stem",
        ))?;
        let path_stem_str = path_stem.to_str().ok_or(DirAccess::new(
            "Read examples path stem as string",
            "Could not get file stem as str",
        ))?;

        let examples = load_paths(path)?;
        let example_names = get_path_stems(&examples)?;

        let example_strs = get_example_strs(&examples, path_stem_str)?;
        let vec_strs = get_vec_strs(&example_names);
        let examples_str = examples_rs(&example_strs, &vec_strs, path_stem_str);
        write_example_rs(path_stem_str, &examples_str)?;
    }
    Ok(())
}
