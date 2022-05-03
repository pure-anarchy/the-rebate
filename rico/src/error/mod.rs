use std::num::{ParseFloatError, ParseIntError};

pub mod impls;



pub type Errable<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    ParseIntErr(ParseIntError),
    ParseFloatErr(ParseFloatError),
    DatabaseErr(sqlx::Error),
}
