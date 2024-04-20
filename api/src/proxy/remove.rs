use http_body_util::{BodyExt, Full};
use hyper::{Method, Request};
use serde::{Serialize,Deserialize};
use serde_json::json;
use crate::{api_url::REMOVE_PROXY, prelude::*};

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveProxy {
    pub flag: bool,
    pub msg: String,
}

pub async fn remove_proxy(proxy_id:i32,api_client: &mut Client) -> Result<RemoveProxy>{
    // 获取 API Client 中的 Auth
    let _ = api_client.get_auth()?;

    // 创建 Headers
    let headers = api_client.make_headers();

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.get_client();

    // 创建对应 API 的 POST 请求
    let mut req = Request::builder().method(Method::POST).uri(REMOVE_PROXY);

    // 添加 Headers
    req.headers_mut().unwrap().extend(headers);

    // 添加 Body
    let req = req.body(Full::from(json!({
        "proxy_id": proxy_id,
    }).to_string()))?;

    // 用 Hyper Client 发送 Request
    let res = client.request(req).await?;

    // 获取 Headers 和 Data
    let headers = res.headers().clone();
    let data = res.collect().await?.to_bytes();

    // 添加 Cookie
    api_client.cookies.extend_header(&headers).unwrap();

    let json: RemoveProxy = serde_json::from_slice(&data).unwrap();

    if !json.flag {
        return Err(Error::new(-1, &json.msg));
    }

    Ok(json)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_remove_proxy() {
        
    }
}
