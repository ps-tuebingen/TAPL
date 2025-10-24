#[derive(serde::Deserialize)]
pub struct TestConfig {
    pub name: String,
    pub ty: String,
    pub evaluated: String,
    #[serde(default)]
    pub contents: String,
}
