use clap::{App, Arg};
use futures::{future, prelude::*};
use std::net::SocketAddr;
use tarpc::{
    client, context,
    serde_transport::tcp,
    server::{self, Channel},
    tokio_serde::formats::Json,
};
use tarpc_example::rpc::{World, WorldClient};

/// This is the type that implements the generated World trait. It is the business logic
/// and is used to start the server.
#[derive(Clone)]
struct HelloServer {
    server_addr: SocketAddr,
}

#[tarpc::server]
impl World for HelloServer {
    async fn hello(self, _: context::Context, name: String) -> String {
        println!("{name}");
        format!("return Hello, {name}!")
    }

    async fn server_to_server(self, _: context::Context) -> String {
        let transport = tarpc::serde_transport::tcp::connect(self.server_addr, Json::default)
            .await
            .unwrap();

        let client = WorldClient::new(client::Config::default(), transport).spawn();

        let hello = client
            .hello(context::current(), "YOOO".to_string())
            .await
            .unwrap();

        format!("Test, {hello}!")
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = App::new("Server")
        .version("0.1")
        .about("Test server")
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .value_name("NUMBER")
                .help("Location of JSON config file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let sid = flags.value_of("server").unwrap().parse().unwrap();

    let port = match sid {
        0 => 8001,
        1 => 8071,
        _ => panic!("Oh no!"),
    };

    let server_addr = SocketAddr::new("0.0.0.0".parse().expect("Could not parse"), port);

    println!("Running: {:?}", server_addr);
    let listener = tcp::listen(&server_addr, Json::default).await?;
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = HelloServer {
                server_addr: SocketAddr::new("0.0.0.0".parse().expect("Could not parse"), 8071),
            };
            channel.execute(server.serve())
        })
        .buffer_unordered(1)
        .for_each(|_| async {})
        .await;

    Ok(())
}
