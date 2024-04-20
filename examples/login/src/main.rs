use openfrp_sdk::{login::*, prelude::*,user};

pub const EMAIL: &'static str = "xslimenb@xslimenb.eu.org";
pub const PASSWORD: &'static str = "123Tester_";

#[tokio::main]
async fn main() -> Result<()>{
    let mut client = Client::new();
    let account = Account::new(EMAIL, PASSWORD);
    login(&account, &mut client).await?;
    println!("auth: {:#?}",client.auth.clone().get()?);
    let userinfo = user::get_info(&mut client).await?;
    println!("userinfo: {:#?}", userinfo);
    let userproxies = user::get_proxies(&mut client).await?;
    println!("userproxies: {:#?}", userproxies);
    Ok(())
}