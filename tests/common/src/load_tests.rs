use super::to_test_err;
use common::errors::{Error, ErrorKind};
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub struct TestContents<Conf> {
    pub conf: Conf,
    pub source_name: String,
    pub source_contents: String,
}

pub fn load_dir<Conf>(dir: &PathBuf, src_ext: &str) -> Result<Vec<TestContents<Conf>>, Error>
where
    Conf: for<'a> serde::Deserialize<'a>,
{
    let mut tests = vec![];
    for entry in read_dir(dir).map_err(|err| {
        to_test_err(ErrorKind::DirAccess {
            tried: format!("Read {dir:?}"),
            msg: err.to_string(),
        })
    })? {
        let entry = entry.map_err(|err| {
            to_test_err(ErrorKind::DirAccess {
                tried: format!("Read {dir:?}"),
                msg: err.to_string(),
            })
        })?;
        let path = entry.path();
        let stem = path
            .file_stem()
            .ok_or(to_test_err(ErrorKind::DirAccess {
                tried: format!("Get file stem of {path:?}"),
                msg: "Got None".to_owned(),
            }))?
            .to_str()
            .ok_or(to_test_err(ErrorKind::DirAccess {
                tried: format!("Get file stem as str of {path:?}"),
                msg: "Got None".to_owned(),
            }))?
            .to_owned();

        let mut source_file = path.join(stem.clone());
        source_file.set_extension(src_ext);

        let mut config_file = source_file.clone();
        config_file.set_extension("toml");
        let config_contents = read_to_string(&config_file).map_err(|err| {
            to_test_err(ErrorKind::DirAccess {
                tried: format!("Read file {config_file:?}"),
                msg: err.to_string(),
            })
        })?;
        let config = basic_toml::from_str(&config_contents).map_err(|err| {
            to_test_err(ErrorKind::Toml {
                source: config_contents,
                msg: err.to_string(),
            })
        })?;

        let contents = read_to_string(&source_file).map_err(|err| {
            to_test_err(ErrorKind::DirAccess {
                tried: format!("Read file {source_file:?}"),
                msg: err.to_string(),
            })
        })?;

        tests.push(TestContents {
            source_name: stem,
            source_contents: contents,
            conf: config,
        });
    }
    tests.sort_by(|tst1, tst2| tst1.source_name.cmp(&tst2.source_name));
    Ok(tests)
}
