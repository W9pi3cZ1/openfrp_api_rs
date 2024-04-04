# OpenFrp Rust SDK

一个用[***Hyper***](https://hyper.rs),[***Rust***](https://www.rust-lang.org/)和[***Tokio***](https://tokio.rs)实现的第三方[***OpenFrp***](https://www.openfrp.net) OPENAPI的SDK。

## 添加方法

```bash
cargo add --git "https://github.com/W9pi3cZ1/openfrp_sdk.git" openfrp-sdk
```

## 示例
```rust
use openfrp_sdk::{login::*, prelude::*};

pub const EMAIL: &'static str = "xslimenb@xslimenb.eu.org";
pub const PASSWORD: &'static str = "123Tester_";

#[tokio::main]
async fn main() -> Result<()>{
    let mut client = Client::new();
    let account = Account::new(EMAIL, PASSWORD);
    login(&account, &mut client).await?;
    println!("auth: {:#?}",client.auth.get()?);
    Ok(())
}
```