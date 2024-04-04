pub use crate::error::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub session: String,
    pub authorization: String,
}

#[derive(Debug, Clone)]
pub struct OptionAuth(pub Option<Auth>);

impl OptionAuth {
    pub fn get(self) -> Result<Auth> {
        match self.0 {
            Some(auth) => Ok(auth),
            None => Err(Error::new(2, "No Auth found")),
        }
    }

    pub fn set(&mut self, auth: Auth) {
        self.0 = Some(auth);
    }
}

impl Auth {
    pub fn new(session: String, authorization: String) -> Self {
        Self {
            session,
            authorization,
        }
    }
}
