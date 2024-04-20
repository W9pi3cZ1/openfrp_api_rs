use http_body_util::{BodyExt, Full};
use hyper::{Method, Request};
use serde::{Deserialize, Serialize};

use crate::{api_url::NEW_PROXY, prelude::*};

use super::Proxy;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProxy {
    pub flag: bool,
    pub msg: String,
}

pub async fn new_proxy(proxy: Proxy, api_client: &mut Client) -> Result<NewProxy> {
    // 获取 API Client 中的 Auth
    let _ = api_client.get_auth()?;

    // 把 proxy 中的 proxy_id 删除
    let mut proxy = proxy.clone();
    proxy.proxy_id = None;

    // 创建 Headers
    let headers = api_client.make_headers();

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.get_client();

    // 创建对应 API 的 POST 请求
    let mut req = Request::builder().method(Method::POST).uri(NEW_PROXY);

    // 添加 Headers
    req.headers_mut().unwrap().extend(headers);

    // 添加 Body
    let req = req.body(Full::from(serde_json::to_string(&proxy).unwrap()))?;

    // 用 Hyper Client 发送 Request
    let res = client.request(req).await?;

    // 获取 Headers 和 Data
    let headers = res.headers().clone();
    let data = res.collect().await?.to_bytes();

    // 添加 Cookie
    api_client.cookies.extend_header(&headers).unwrap();

    let json: NewProxy = serde_json::from_slice(&data).unwrap();

    if !json.flag {
        return Err(Error::new(-1, &json.msg));
    }

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        login::{login, Account},
        tests,
    };

    #[tokio::test]
    async fn test_new_proxy() -> Result<()> {
        let mut api_client = Client::new();
        let account = Account {
            user: tests::EMAIL.to_string(),
            password: tests::PASSWORD.to_string(),
        };
        login(&account, &mut api_client).await?;
        let pxy = Proxy {
            autoTls: "false".to_string(),
            custom: "".to_string(),
            dataEncrypt: true,
            dataGzip: true,
            domain_bind: None,
            forceHttps: false,
            local_addr: "127.0.0.1".to_string(),
            local_port: "1145".to_string(),
            name: "foo".to_string(),
            node_id: 4,
            proxyProtocolVersion: true,
            proxy_id: None,
            remote_port: Some(50294),
            r#type: "tcp".to_string(),
        };
        let _ = new_proxy(pxy, &mut api_client).await?;
        Ok(())
    }
}
