use std::error;
use std::fmt;
use std::convert::From;
use std::io::Error;
use lettre;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn type_of<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// =========================[ ConfigError ]=============================

/// An error that can occur trying to parse a midi message
#[derive(Debug)]
pub enum ConfigError {
    NotFound(String),
    ReadError(String, String),
    Error(Error),
}

impl From<Error> for ConfigError {
    fn from(err: Error) -> ConfigError {
        ConfigError::Error(err)
    }
}

impl error::Error for ConfigError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ConfigError::Error(ref err) => Some(err as &dyn error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::NotFound(ref path,) => write!(f, "ConfigError: No configuration found at {}", path),
            ConfigError::ReadError(ref path, ref error) => write!(f, "ConfigError: Error while reading config at {}: {}", path, error),
            ConfigError::Error(ref e) => write!(f, "{}", e),
        }
    }
}



// =========================[ MailError ]=============================

/// An error that can occur trying to parse a midi message
#[derive(Debug)]
pub enum MailError {
    SMTPError(String),
    Error(Error),
}

impl From<Error> for MailError {
    fn from(err: Error) -> MailError {
        MailError::Error(err)
    }
}

impl From<lettre::transport::smtp::error::Error> for MailError {
    fn from(err: lettre::transport::smtp::error::Error) -> MailError {
        MailError::SMTPError(format!("{}", err))
    }
}

impl error::Error for MailError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            MailError::Error(ref err) => Some(err as &dyn error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for MailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MailError::SMTPError(ref smtp,) => write!(f, "MailError: Broken SMTP Configuration {}", smtp),
            MailError::Error(ref e) => write!(f, "{}", e),
        }
    }
}