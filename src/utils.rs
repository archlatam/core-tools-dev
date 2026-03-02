pub fn check_regular_file(_path: &str) -> bool {
    false
}
pub fn fix_path(_path: &str) -> String {
    String::new()
}
pub fn read_json(_path: &str) -> Result<serde_json::Value, std::io::Error> {
    Ok(serde_json::Value::Null)
}
pub fn write_json(_path: &str, _value: &serde_json::Value) -> Result<(), std::io::Error> {
    Ok(())
}

pub struct PacmanWrapper;
impl PacmanWrapper {
    pub fn new() -> Self {
        Self
    }
}
