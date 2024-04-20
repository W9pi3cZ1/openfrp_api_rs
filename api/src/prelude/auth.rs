use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub session: String,
    pub authorization: String,
}

impl Auth {
    pub fn new(session: String, authorization: String) -> Self {
        Self {
            session,
            authorization,
        }
    }
}
