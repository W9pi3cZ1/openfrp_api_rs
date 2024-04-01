use std::{
    error::Error as StdError,
    fmt::{Debug, Display},
};

use cookie::ParseError;
use hyper::header::InvalidHeaderValue;

pub struct Error {
    pub inner: Box<ErrorImpl>,
}

impl Error {
    pub fn new(code: i32, msg: &str) -> Self {
        Self {
            inner: Box::new(ErrorImpl {
                code: code,
                msg: msg.to_string(),
                cause: None,
            }),
        }
    }
}


impl From<Cause> for Error {
    fn from(err: Cause) -> Self {
        Self {
            inner: Box::new(ErrorImpl {
                code: -1,
                msg: err.to_string(),
                cause: Some(err),
            }),
        }
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Error {
        Self {
            inner: Box::new(ErrorImpl {
                code: -1,
                msg: err.to_string(),
                cause: Some(Box::new(err)),
            }),
        }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Self {
            inner: Box::new(ErrorImpl {
                code: -1,
                msg: err.to_string(),
                cause: Some(Box::new(err)),
            }),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Self {
            inner: Box::new(ErrorImpl {
                code: -1,
                msg: err.to_string(),
                cause: Some(Box::new(err)),
            })
        }
    }
}

impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Error {
        Self {
            inner: Box::new(ErrorImpl {
                code: -1,
                msg: err.to_string(),
                cause: Some(Box::new(err)),
            })
        }
    }
}


impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.inner)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.inner)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.inner.msg.as_str()
    }
}

type Cause = Box<dyn StdError>;

#[derive(Debug)]
pub struct ErrorImpl {
    pub code: i32,
    pub cause: Option<Cause>,
    pub msg: String,
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() -> Result<()> {
        let err = Error {
            inner: Box::new(ErrorImpl {
                code: 1,
                msg: "我超！你居然把程序弄坏了！".to_string(),
                cause: None,
            }),
        };
        Err(err)
    }
}
