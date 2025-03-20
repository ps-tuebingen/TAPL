use super::errors::Error;
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
    for entry in read_dir(dir).map_err(|_| Error::ReadDir(dir.clone()))? {
        let entry = entry.map_err(|_| Error::ReadDir(dir.clone()))?;
        let path = entry.path();
        let stem = path
            .file_stem()
            .ok_or(Error::FileName(path.clone()))?
            .to_str()
            .ok_or(Error::FileName(path.clone()))?
            .to_owned();

        let mut source_file = path.join(stem.clone());
        source_file.set_extension(src_ext);

        let mut config_file = source_file.clone();
        config_file.set_extension("toml");
        let config_contents =
            read_to_string(&config_file).map_err(|_| Error::ReadFile(config_file.clone()))?;
        let config = basic_toml::from_str(&config_contents)
            .map_err(|_| Error::ParseToml(config_file.clone()))?;

        let contents = read_to_string(&source_file).map_err(|_| Error::ReadFile(source_file))?;

        tests.push(TestContents {
            source_name: stem,
            source_contents: contents,
            conf: config,
        });
    }
    tests.sort_by(|tst1, tst2| tst1.source_name.cmp(&tst2.source_name));
    Ok(tests)
}
