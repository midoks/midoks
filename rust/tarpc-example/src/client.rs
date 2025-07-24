use std::net::SocketAddr;
use tarpc::{client, context, tokio_serde::formats::Json};
use tarpc_example::rpc::WorldClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr_0 = SocketAddr::new("0.0.0.0".parse().expect("Could not parse"), 8001);

    let transport0 = tarpc::serde_transport::tcp::connect(server_addr_0, Json::default);

    let client0 = WorldClient::new(client::Config::default(), transport0.await?).spawn();

    let hello = client0
        .hello(context::current(), "Stim".to_string())
        .await?;
    println!("{hello}");

    let test = client0.server_to_server(context::current()).await?;
    println!("test: {test}");

    Ok(())
}
