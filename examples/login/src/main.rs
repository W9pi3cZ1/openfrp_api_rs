use openfrp_api::{login::*, prelude::*,user};

pub const EMAIL: &'static str = "xslimenb@xslimenb.eu.org";
pub const PASSWORD: &'static str = "123Tester_";

#[tokio::main]
async fn main() -> Result<()>{
    let mut client = Client::new();
    let account = Account::new(EMAIL, PASSWORD);
    login(&account, &mut client).await?;
    println!("auth: {:#?}",client.get_auth()?);
    let userinfo = user::get_user_info(&mut client).await?;
    println!("userinfo: {:#?}", userinfo);
    let userproxies = user::get_user_proxies(&mut client).await?;
    println!("userproxies: {:#?}", userproxies);
    Ok(())
}