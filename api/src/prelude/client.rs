use super::*;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client as HyperClient},
    rt::TokioExecutor,
};

type APIHyperClient = HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>;

#[derive(Clone, Debug)]
pub struct Client {
    client: APIHyperClient,
    pub cookies: Cookies,
    auth: Option<Auth>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: HyperClient::builder(TokioExecutor::new()).build(HttpsConnector::new()),
            cookies: Cookies::new(),
            auth: None,
        }
    }

    pub fn get_auth(&self) -> Result<Auth> {
        match self.auth.clone() {
            Some(auth) => Ok(auth),
            None => Err(Error::new(2, "No Auth found")),
        }
    }

    pub fn is_auth_empty(&self) -> bool {
        self.auth.is_none()
    }

    pub fn set_auth(&mut self, auth: Auth) {
        self.auth = Some(auth);
    }

    pub fn get_client(&self) -> APIHyperClient {
        self.client.clone()
    }

    pub fn make_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        if !self.cookies.is_empty() {
            headers.insert("Cookie", self.cookies.to_string().parse().unwrap());
        }

        if !self.is_auth_empty() {
            headers.insert(
                "Authorization",
                self.get_auth().unwrap().authorization.parse().unwrap(),
            );
        }

        headers
    }
}
