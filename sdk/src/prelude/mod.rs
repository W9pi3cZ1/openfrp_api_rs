use cookie::Cookie;
use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
pub use crate::error::*;

#[derive(Clone, Debug)]
pub struct Cookies {
    pub cookies: Vec<Cookie<'static>>,
}

impl Cookies {
    pub fn new() -> Self {
        Self {
            cookies: Vec::new(),
        }
    }

    pub fn add_cookie<S: ToString>(&mut self, cookie: S) -> Result<()> {
        self.cookies.push(Cookie::parse(cookie.to_string())?);
        Ok(())
    }

    pub fn extend(&mut self, other: Cookies) {
        self.cookies.extend(other.cookies.clone());
    }
}

impl ToString for Cookies {
    fn to_string(&self) -> String {
        let mut cookies_string: Vec<String> = vec![];
        for cookie in self.cookies.iter() {
            cookies_string.push(format!("{}={}", cookie.name(), cookie.value()));
        }
        cookies_string.join("; ")
    }
}

impl From<String> for Cookies {
    fn from(s: String) -> Self {
        let split_cookies = Cookie::split_parse(s);
        Self {
            cookies: split_cookies
                .map(|cookie| match cookie {
                    Ok(cookie) => cookie,
                    Err(_) => todo!(),
                })
                .collect(),
        }
    }
}

impl From<&str> for Cookies {
    fn from(s: &str) -> Self {
        let split_cookies = Cookie::split_parse(s.to_string());
        Self {
            cookies: split_cookies
                .map(|cookie| match cookie {
                    Ok(cookie) => cookie,
                    Err(_) => todo!(),
                })
                .collect(),
        }
    }
}

#[derive(Clone,Debug)]
pub struct Client {
    pub client: hyper::Client<HttpsConnector<HttpConnector>, Body>,
    pub cookies: Cookies,
    pub auth: Auth,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: hyper::Client::builder().build(HttpsConnector::new()),
            cookies: Cookies::new(),
            auth: Auth::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub user: String,
    pub password: String,
}

impl Account {
    pub fn new<S: ToString>(user: S, password: S) -> Self {
        Self {
            user: user.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Auth {
    pub session: String,
    pub authorization: String,
}

impl Auth {
    pub fn new() -> Self {
        Self {
            session: String::new(),
            authorization: String::new(),
        }
    }
}