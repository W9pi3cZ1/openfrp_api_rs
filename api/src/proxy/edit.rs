
use http_body_util::{BodyExt, Full};
use hyper::{Method, Request};
use serde::{Deserialize, Serialize};

use crate::{api_url::EDIT_PROXY, prelude::*};

use super::Proxy;

#[derive(Serialize, Deserialize, Debug)]
pub struct EditProxy {
    pub flag: bool,
    pub msg: String,
}

pub async fn edit_proxy(proxy: Proxy, api_client: &mut Client) -> Result<EditProxy> {
    // 获取 API Client 中的 Auth
    let _ = api_client.get_auth()?;

    // 验证是否有 proxy_id
    if proxy.proxy_id.is_none() {
        return Err(Error::new(3, "proxy_id is None"));
    }

    // 创建 Headers
    let headers = api_client.make_headers();

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.get_client();

    // 创建对应 API 的 POST 请求
    let mut req = Request::builder().method(Method::POST).uri(EDIT_PROXY);

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

    let json: EditProxy = serde_json::from_slice(&data).unwrap();

    if !json.flag {
        return Err(Error::new(-1, &json.msg));
    }

    Ok(json)
}
