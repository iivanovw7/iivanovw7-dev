use crate::server::server;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;

mod config;
mod server;

pub mod handlers;
pub mod types;
pub mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server().await
}
