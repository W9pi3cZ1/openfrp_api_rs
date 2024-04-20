use crate::{api_url::*, error::*, prelude::*};
use http_body_util::{BodyExt, Full};
use hyper::{Method, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    /// Account email/name 账户邮箱或名称
    pub user: String,

    /// Account password 账户密码
    pub password: String,
}

impl Account {
    pub fn new<S: ToString>(user: S, password: S) -> Self {
        Self {
            user: user.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {

    /// Request status 登录请求是否成功
    pub flag: bool,

    /// Request message 登录请求返回信息
    pub msg: String,

    /// Request data 登录请求返回数据
    pub data: Option<Value>,

    /// Request status code 登录请求代码
    pub code: Option<i32>,
}

/// Login Natayark ID OAuth2 登录到 Natayark ID OAuth2
pub async fn login_oauth2(account: &Account, api_client: &mut Client) -> Result<Login> {
    // 创建 Headers
    let headers = api_client.make_headers();

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.get_client();

    // 创建对应 API 的 POST 请求
    let mut req = Request::builder().method(Method::POST).uri(OAUTH2_URL);

    // 添加 Headers
    req.headers_mut().unwrap().extend(headers);

    // 添加 Body
    let req = req.body(Full::from(serde_json::to_string(account).unwrap()))?;

    // 用 Hyper Client 发送 Request
    let res = client.request(req).await?;

    // 获取 Headers 和 Data
    let headers = res.headers().clone();
    let data = res.collect().await?.to_bytes();

    // 添加 Cookie
    api_client.cookies.extend_header(&headers).unwrap();

    let json: Login = serde_json::from_slice(&data).unwrap();

    if !json.flag {
        return Err(Error::new(json.code.unwrap_or(-1), &json.msg));
    }

    Ok(json)
}

/// Get code by Natayark callback 通过 Natayark ID 回调，获取 Code
pub async fn oauth2_callback(_login_res: Login, api_client: &mut Client) -> Result<String> {
    // 创建 Headers
    let headers = api_client.make_headers();

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.get_client();

    // 创建对应 API 的 POST 请求
    let mut req = Request::builder().method(Method::POST).uri(OAUTH2_CALLBACK);

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

    let json: Login = serde_json::from_slice(&data).unwrap();

    if !json.flag {
        return Err(Error::new(json.code.unwrap_or(-1), &json.msg));
    } else {
        match json.data {
            Some(data) => {
                return Ok(data["code"].as_str().unwrap().to_string());
            }
            _ => todo!(),
        }
    }
}

/// Login openfrp by code 登录OpenFrp通过Code
pub async fn login_by_code(code: String, api_client: &mut Client) -> Result<()> {
    // 创建 Headers
    let headers = api_client.make_headers();

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.get_client();

    // 创建对应 API 的 POST 请求
    let mut req =
        Request::builder()
            .method(Method::POST)
            .uri(format!("{}{}", LOGIN_CALLBACK, code.as_str()));

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
    let json: Login = serde_json::from_slice(&data).unwrap();

    // Auth
    let auth: Auth;

    if !json.flag {
        return Err(Error::new(json.code.unwrap_or(-1), &json.msg));
    } else {
        // 把 Authorization 写入 auth
        let authorization = headers
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        match json.data {
            Some(data) => {
                let session = data.as_str().unwrap().to_string();
                auth = Auth {
                    session,
                    authorization,
                }
            }
            _ => todo!(),
        }
    }
    api_client.set_auth(auth);
    Ok(())
}

/// Login OpenFrp by account 直接通过账户登录OpenFrp
pub async fn login(account: &Account, api_client: &mut Client) -> Result<()> {
    let login_oa2 = login_oauth2(account, api_client).await?;
    let code = oauth2_callback(login_oa2, api_client).await?;
    login_by_code(code, api_client).await?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::tests;
    use crate::login::*;

    #[tokio::test]
    async fn test_login() -> Result<()> {
        let account = Account::new(tests::EMAIL, tests::PASSWORD);
        let mut client = Client::new();
        login(&account, &mut client).await?;
        println!("auth: {:#?}", client.get_auth()?);
        Ok(())
    }
}
