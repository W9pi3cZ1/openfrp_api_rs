pub mod api_url;
pub mod login;
pub mod prelude;
pub mod user;
pub mod info;
pub mod proxy;

pub mod error;

pub use error::*;

#[cfg(test)]
mod tests {
    pub const EMAIL: &'static str = "xslimenb@xslimenb.eu.org";
    pub const PASSWORD: &'static str = "123Tester_";
}