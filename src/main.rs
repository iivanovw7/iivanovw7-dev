use crate::server::server;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

mod config;
pub mod handlers;
mod server;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server().await
}
