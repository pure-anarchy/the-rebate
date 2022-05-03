use super::Error;
use std::num::{ParseFloatError, ParseIntError};

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::ParseIntErr(err)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Error::ParseFloatErr(err)
    }
}


use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Error::ParseIntErr(error) => {
                write!(f, "the error here is {:#?}", error)
            }

            Error::ReqwestError(error) => {
                if error.is_request() {
                    writeln!(f, "")?;
                    writeln!(f, "")
                } else if error.is_status() {
                    writeln!(f, "")?;
                    writeln!(f, "")
                } else if error.is_builder() {
                    writeln!(f, "")?;
                    writeln!(f, "")
                } else if error.is_redirect() {
                    writeln!(f, "")?;
                    writeln!(f, "")
                } else if error.is_timeout() {
                    writeln!(f, "")?;
                    writeln!(f, "")
                } else if error.is_decode() {
                    writeln!(f, "")?;
                    writeln!(f, "")
                } else {
                    writeln!(f, "")?;
                    writeln!(f, "")
                }
            }

            Error::ParseFloatErr(error) => {
                write!(f, "the error here is {:#?}", error)
            }

            _ => {
                writeln!(f, "sucks for you hahaha, were working on this though")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::DatabaseErr(err)
    }
}
