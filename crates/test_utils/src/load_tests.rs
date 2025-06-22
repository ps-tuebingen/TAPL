use super::{errors::Error, test::TestConfig};
use std::{
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

pub fn load_dir<Conf>(dir: &PathBuf, src_ext: &str) -> Result<Vec<Conf>, Error>
where
    Conf: TestConfig,
{
    let mut tests = vec![];
    for entry in read_dir(dir).map_err(|err| Error::dir_access(&format!("read {dir:?}"), err))? {
        let entry = entry.map_err(|err| Error::dir_access(&format!("read {dir:?}"), err))?;
        let path = entry.path();
        let stem = path
            .file_stem()
            .ok_or(Error::file_access(
                &format!("get file stem of {path:?}"),
                "Got None",
            ))?
            .to_str()
            .ok_or(Error::file_access(
                &format!("get file stem of {path:?} as string"),
                "Got None",
            ))?
            .to_owned();

        let mut source_file = path.join(stem.clone());
        source_file.set_extension(src_ext);

        let contents = read_to_string(&source_file)
            .map_err(|err| Error::file_access(&format!("read file {source_file:?}"), err))?;

        let mut config_file = source_file.clone();
        config_file.set_extension("toml");
        let config_contents = read_to_string(&config_file)
            .map_err(|err| Error::file_access(&format!("Read File {config_file:?}"), err))?;
        let mut config: Conf = basic_toml::from_str(&config_contents)
            .map_err(|err| Error::toml(&config_contents, err))?;
        config.set_contents(contents);

        tests.push(config);
    }
    tests.sort_by(|tst1, tst2| tst1.name().cmp(tst2.name()));
    Ok(tests)
}
