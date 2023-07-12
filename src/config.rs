#[derive(Debug, PartialEq)]
pub struct Config {
    pub name: String,
    pub key: String,
    pub value: u8,
}

impl Config {
    pub fn to_command_string(&self) -> String {
        format!("{}={}", self.key, self.value)
    }
}