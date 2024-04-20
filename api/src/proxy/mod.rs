mod remove;
pub use remove::*;

mod new;
pub use new::*;

mod edit;
pub use edit::*;

use serde::{Deserialize, Serialize};

use crate::user::ProxyInfo;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Proxy {
    pub autoTls: String,
    pub custom: String,
    pub dataEncrypt: bool,
    pub dataGzip: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_bind: Option<String>,
    pub forceHttps: bool,
    pub local_addr: String,
    pub local_port: String,
    pub name: String,
    pub node_id: i32,
    pub proxyProtocolVersion: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_port: Option<i32>,
    pub r#type: String,
}

impl From<ProxyInfo> for Proxy {
    fn from(info: ProxyInfo) -> Self {
        Self {
            autoTls: info.autoTls.unwrap_or("false".to_string()),
            custom: info.custom.unwrap_or(String::new()),
            dataEncrypt: info.useEncryption,
            dataGzip: info.useCompression,
            domain_bind: info.domain,
            forceHttps: info.forceHttps.unwrap_or(false),
            local_addr: info.localIp,
            local_port: info.localPort.to_string(),
            name: info.proxyName,
            node_id: info.nid,
            proxyProtocolVersion: info.proxyProtocolVersion,
            proxy_id: Some(info.id),
            remote_port: info.remotePort,
            r#type: info.proxyType,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::ProxyInfo;

    #[test]
    fn test_proxy_from_proxy_info() {
        let info = ProxyInfo {
            autoTls: Some("false".to_string()),
            connectAddress: "kr-se-cncn-1.of-7af93c01.shop:57854".to_string(),
            custom: None,
            domain: None,
            forceHttps: Some(false),
            friendlyNode: "韩国-1".to_string(),
            id: 546529,
            uid: 95255,
            nid: 4,
            lastUpdate: 1713532699000,
            lastLogin: None,
            localIp: "127.0.0.1".to_string(),
            localPort: 1145,
            online: false,
            proxyName: "gdiaga".to_string(),
            proxyProtocolVersion: true,
            proxyType: "tcp".to_string(),
            status: true,
            useEncryption: true,
            useCompression: true,
            remotePort: Some(57854),
        };
        let pxy = Proxy::from(info.clone());
        println!("{:#?}", pxy);
    }
}
