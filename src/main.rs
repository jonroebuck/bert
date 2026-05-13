#![allow(dead_code)]

mod agent;
mod bootstrap;
mod boxcarve;
mod tramway;

#[tokio::main]
async fn main() {
    agent::run().await;
}
