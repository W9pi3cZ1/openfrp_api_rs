use http_body_util::{BodyExt, Full};
use hyper::{Method, Request};
use serde::{Deserialize, Serialize};

use crate::{api_url::GET_USER_PROXIES, prelude::*};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
/// Tunnel info 隧道信息
pub struct ProxyInfo {
    pub autoTls: Option<String>,
    pub connectAddress: String,
    pub custom: Option<String>,
    pub domain: Option<String>,
    pub forceHttps: Option<bool>,
    pub friendlyNode: String,
    pub id: i32,
    pub uid: i32,
    pub nid: i32,
    pub lastUpdate: i64,
    pub lastLogin: Option<i64>,
    pub localIp: String,
    pub localPort: i32,
    pub online: bool,
    pub proxyName: String,
    pub proxyProtocolVersion: bool,
    pub proxyType: String,
    pub remotePort: Option<i32>,
    pub status: bool,
    pub useEncryption: bool,
    pub useCompression: bool,
}

#[derive(Debug, Serialize, Deserialize)]

/// Proxies data 多个隧道的数据
pub struct UserProxiesData {
    /// Proxies count 隧道数量
    pub total: i32,

    /// Proxies list 隧道列表
    pub list: Vec<ProxyInfo>,
}

#[derive(Debug, Serialize, Deserialize)]

pub struct UserProxies {
    /// ProxiesData
    pub data: Option<UserProxiesData>,

    /// Request status 请求是否成功
    pub flag: bool,

    /// Request message 请求返回的信息
    pub msg: String,
}

pub async fn get_user_proxies(api_client: &mut Client) -> Result<UserProxies> {
    // 获取 API Client 中的 Auth
    let _ = api_client.get_auth()?;

    // 创建 Headers
    let headers = api_client.make_headers();

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.get_client();

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

    let json: UserProxies = serde_json::from_slice(&data).unwrap();

    if !json.flag {
        return Err(Error::new(-1, &json.msg));
    }

    Ok(json)
}

#[cfg(test)]
mod tests {
    use crate::login::*;
    use crate::prelude::*;
    use crate::tests;
    use crate::user;

    #[tokio::test]
    async fn test_user_proxies() -> Result<()> {
        let account = Account::new(tests::EMAIL, tests::PASSWORD);
        let mut client = Client::new();
        login(&account, &mut client).await?;
        let proxies = user::get_user_proxies(&mut client).await?;
        println!("proxies: {:#?}", proxies);
        Ok(())
    }
}
