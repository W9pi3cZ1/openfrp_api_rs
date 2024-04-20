use http_body_util::{BodyExt, Full};
use hyper::{Method, Request};
use serde::{Deserialize, Serialize};
use serde_repr::*;

use crate::{api_url::GET_NODE_LIST, prelude::*};

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum NodeRegion {
    /// 中国大陆
    CN = 1,

    /// 港澳台
    HMT = 2,

    /// 海外
    OTHER = 3,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeProtocols {
    pub tcp: bool,
    pub udp: bool,
    pub http: bool,
    pub https: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NodePort {
    Number(i32),
    String(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct Node {
    /// 节点是否允许弹性隧道
    pub allowEc: bool,

    /// 节点端口限制 eg: (1500,5000) 限制1500-5000，如果为空字符串或None，则不限制
    pub allowPort: Option<String>,

    /// 节点总宽带
    pub bandwidth: i32,

    /// 节点宽带倍率
    pub bandwidthMagnification: f64,

    /// 节点区域
    pub classify: NodeRegion,

    /// 节点标签
    pub comments: String,

    /// 默认启用 TLS
    pub enableDefaultTLS: bool,

    /// 允许创建此节点的权限组 eg: vip;svip;admin;dev 表示仅vip、svip、admin、dev权限组可创建
    pub group: String,

    /// 节点域名，但如果不被允许创建则隐藏
    pub hostname: String,

    /// 节点ID
    pub id: i32,

    /// 最高在线倍率
    pub maxOnlineMagnification: f64,

    /// 节点名称
    pub name: String,

    /// 是否需实名
    pub needRealname: bool,

    /// 节点frps连接端口
    pub port: NodePort,

    /// 节点在线状态码
    pub status: i32,

    /// 隧道单价
    pub unitcostEc: i32,

    /// 节点详情
    pub description: String,

    /// 节点支持协议
    pub protocolSupport: NodeProtocols,

    /// 节点是否满载
    pub fullyLoaded: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodesData {
    /// 节点总数
    pub total: i32,

    /// 节点列表
    pub list: Vec<Node>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nodes {
    /// 请求数据
    pub data: Option<NodesData>,

    /// 请求状态
    pub flag: bool,

    /// 请求消息
    pub msg: String,
}

pub async fn get_nodes(api_client: &mut Client) -> Result<Nodes> {
    // 获取 API Client 中的 Auth
    let _ = api_client.get_auth()?;

    // 创建 Headers
    let headers = api_client.make_headers();

    // 克隆 API Client 中的 Hyper Client
    let client = api_client.get_client();

    // 创建对应 API 的 POST 请求
    let mut req = Request::builder().method(Method::POST).uri(GET_NODE_LIST);

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

    let json: Nodes = serde_json::from_slice(&data).unwrap();

    if !json.flag {
        return Err(Error::new(-1, &json.msg));
    }

    Ok(json)
}

#[cfg(test)]
mod tests {
    use crate::info;
    use crate::login::*;
    use crate::prelude::*;
    use crate::tests;

    #[tokio::test]
    async fn test_node_list() -> Result<()> {
        let account = Account::new(tests::EMAIL, tests::PASSWORD);
        let mut client = Client::new();
        login(&account, &mut client).await?;
        let nodes = info::get_nodes(&mut client).await?;
        println!("nodes: {:?}", nodes);

        Ok(())
    }
}
