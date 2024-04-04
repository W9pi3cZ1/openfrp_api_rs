pub use crate::error::*;
use cookie::Cookie;
use hyper::{header::SET_COOKIE, HeaderMap};

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

    pub fn extend_header(&mut self, headers: &HeaderMap) -> Result<()> {
        headers
            .get_all(SET_COOKIE)
            .iter()
            .for_each(|c| self.add_cookie(c.to_str().unwrap()).unwrap());
        Ok(())
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

impl From<HeaderMap> for Cookies {
    fn from(headers: HeaderMap) -> Self {
        let mut cookies = Cookies::new();
        headers
            .get_all(SET_COOKIE)
            .iter()
            .for_each(|c| cookies.add_cookie(c.to_str().unwrap()).unwrap());
        cookies
    }
}
