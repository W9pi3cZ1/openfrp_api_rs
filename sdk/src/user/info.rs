use http_body_util::{BodyExt, Full};
use hyper::{HeaderMap, Method, Request};
use serde::{Deserialize, Serialize};

use crate::{api_url::GET_USER_INFO, prelude::*};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoData {
    pub outLimit: i32,
    pub used: i32,
    pub token: String,
    pub realname: bool,
    pub regTime: String,
    pub inLimit: i32,
    pub friendlyGroup: String,
    pub proxies: i32,
    pub id: i32,
    pub email: String,
    pub username: String,
    pub group: String,
    pub traffic: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    pub data: Option<UserInfoData>,
    pub flag: bool,
    pub msg: String,
}

pub async fn get_info(api_client: &mut Client) -> Result<UserInfo> {
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
    let mut req = Request::builder().method(Method::POST).uri(GET_USER_INFO);

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

    let json: UserInfo = serde_json::from_slice(&data).unwrap();

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
        let user_info = user::get_info(&mut client).await?;
        println!("user info: {:#?}", user_info.data.unwrap());
        Ok(())
    }
}
