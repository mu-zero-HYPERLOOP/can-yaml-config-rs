
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    YamlScanError(yaml_rust::ScanError),
    YamlInvalidFormat(String),
    YamlInvalidType(String),
    ConfigError(can_config_rs::errors::ConfigError),
    Io(std::io::Error),
}

impl From<yaml_rust::ScanError> for Error {
    fn from(value: yaml_rust::ScanError) -> Self {
        Error::YamlScanError(value)
    }
}

impl From<can_config_rs::errors::ConfigError> for Error {
    fn from(value: can_config_rs::errors::ConfigError) -> Self {
        Error::ConfigError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}
