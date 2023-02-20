use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidFilesState {
    details: String,
}

impl InvalidFilesState {
    pub fn new(msg: &str) -> InvalidFilesState {
        InvalidFilesState {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InvalidFilesState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for InvalidFilesState {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct UnhandledFiletype {
    details: String,
}

impl UnhandledFiletype {
    pub fn new(msg: &str) -> UnhandledFiletype {
        UnhandledFiletype {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for UnhandledFiletype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for UnhandledFiletype {
    fn description(&self) -> &str {
        &self.details
    }
}
