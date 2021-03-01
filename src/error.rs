use std::io;
use handlebars;
use serde_yaml;

#[derive(Debug)]
pub enum Error {
    // Not an error but,
    Interrupted,

    // Real errors
    TemplateError(handlebars::TemplateError),
    YamlError(serde_yaml::Error),
    IOError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        Self::TemplateError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Self::YamlError(err)
    }
}
