use super::*;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client as HyperClient},
    rt::TokioExecutor,
};

#[derive(Clone, Debug)]
pub struct Client {
    pub client: HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>,
    pub cookies: Cookies,
    pub auth: OptionAuth,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: HyperClient::builder(TokioExecutor::new()).build(HttpsConnector::new()),
            cookies: Cookies::new(),
            auth: OptionAuth(None),
        }
    }
}
