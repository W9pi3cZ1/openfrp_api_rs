use hyper::{client::HttpConnector, Body};
use hyper_tls::HttpsConnector;
use super::*;

#[derive(Clone, Debug)]
pub struct Client {
    pub client: hyper::Client<HttpsConnector<HttpConnector>, Body>,
    pub cookies: Cookies,
    pub auth: OptionAuth,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: hyper::Client::builder().build(HttpsConnector::new()),
            cookies: Cookies::new(),
            auth: OptionAuth(None),
        }
    }
}