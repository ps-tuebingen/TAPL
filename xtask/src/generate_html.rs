use errors::{DirAccess, FileAccess, build_error::BuildError};
use std::{
    fs::{File, read_dir, read_to_string, remove_file},
    io::Write,
    path::PathBuf,
};

const TEMPLATES_PATH: &str = "html/templates";
const OUT_PATH: &str = "html";
const TEMPLATE_FILE: &str = "html/template.html";
const TEMPLATE_PLACEHOLDER: &str = "{{ CONTENT }}";

pub fn generate_html() -> Result<(), BuildError> {
    let template_contents =
        read_to_string(TEMPLATE_FILE).map_err(|err| FileAccess::new("Read html template", err))?;

    let templates_dir =
        read_dir(TEMPLATES_PATH).map_err(|err| DirAccess::new("Read html templates dir", err))?;
    for template_file in templates_dir {
        let template_file =
            template_file.map_err(|err| DirAccess::new("Read html template entry", err))?;
        let template_path = template_file.path();
        let template_file_name = template_path.file_name().ok_or(FileAccess::new(
            "Read template file name",
            "Could not read template file name",
        ))?;
        let template_fill = read_to_string(&template_path)
            .map_err(|err| FileAccess::new("Read template contents", err))?;

        let out_path = PathBuf::from(OUT_PATH).join(template_file_name);
        let out_contents = template_contents
            .clone()
            .replace(TEMPLATE_PLACEHOLDER, &template_fill);

        if out_path.exists() {
            remove_file(&out_path).map_err(|err| FileAccess::new("Delete old html file", err))?;
        }
        let mut out_file =
            File::create(out_path).map_err(|err| FileAccess::new("Create new html file", err))?;
        out_file
            .write_all(out_contents.as_bytes())
            .map_err(|err| FileAccess::new("Write html contents", err))?;
    }
    Ok(())
}
