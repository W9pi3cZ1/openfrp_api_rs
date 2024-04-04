// use std::collections::HashMap;

use http_body_util::{BodyExt, Full};
use hyper::{HeaderMap, Method, Request};
use serde::{Deserialize, Serialize};
// use serde_json::Value;

use crate::{api_url::GET_USER_PROXIES, prelude::*};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyInfo {
    autoTls: Option<String>,
    connectAddress: String,
    custom: Option<String>,
    domain: Option<String>,
    forceHttps: Option<bool>,
    friendlyNode: String,
    id: i32,
    uid: i32,
    nid: i32,
    lastUpdate: i64,
    lastLogin: Option<i64>,
    localIp: String,
    localPort: i32,
    online: bool,
    proxyName: String,
    proxyProtocolVersion: bool,
    proxyType: String,
    status: bool,
    useEncryption: bool,
    useCompression: bool,
}

// type ProxyInfo = HashMap<String,Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxiesData {
    pub total: i32,
    pub list: Vec<ProxyInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Proxies {
    pub data: Option<ProxiesData>,
    pub flag: bool,
    pub msg: String,
}

pub async fn get_proxies(api_client: &mut Client) -> Result<Proxies> {
    // 获取 API Client 中的 Auth
    let auth = api_client.auth.clone().get()?;

    // 创建 Headers
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);
    headers.insert("Cookie", api_client.cookies.to_string().parse()?);
    headers.insert("Authorization", auth.authorization.parse()?);

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.client.clone();

    // 创建对应 API 的 POST 请求
    let mut req = Request::builder()
        .method(Method::POST)
        .uri(GET_USER_PROXIES);

    // 添加 Headers
    req.headers_mut().unwrap().extend(headers);

    // 添加 Body
    let req = req.body(Full::default())?;

    // 用 Hyper Client 发送 Request
    let res = client.request(req).await?;

    // 获取 Headers 和 Data
    let headers = res.headers().clone();
    let data = res.collect().await?.to_bytes();

    // 添加 Cookie
    api_client.cookies.extend_header(&headers).unwrap();

    let json: Proxies = serde_json::from_slice(&data).unwrap();

    if !json.flag {
        return Err(Error::new(-1, &json.msg));
    }

    Ok(json)
}

#[cfg(test)]
mod tests {
    use crate::login::*;
    use crate::prelude::*;
    use crate::user;
    use crate::tests;

    #[tokio::test]
    async fn test_user_info() -> Result<()> {
        let account = Account::new(tests::EMAIL, tests::PASSWORD);
        let mut client = Client::new();
        login(&account, &mut client).await?;
        let proxies = user::get_proxies(&mut client).await?;
        println!("proxies: {:#?}", proxies);
        Ok(())
    }
}
