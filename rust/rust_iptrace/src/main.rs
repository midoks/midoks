use clap::{Arg, Command};
use std::net::IpAddr;
mod tracer;
mod geo;
mod utils;

use tracer::IpTracer;
use colored::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let matches = Command::new("rust_iptrace")
        .version("0.1.0")
        .author("Your Name")
        .about("IP trace tool implemented in Rust")
        .arg(
            Arg::new("target")
                .help("Target IP address or hostname")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("max-hops")
                .short('m')
                .long("max-hops")
                .value_name("NUMBER")
                .help("Maximum number of hops")
                .default_value("30"),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("SECONDS")
                .help("Timeout for each hop in seconds")
                .default_value("5"),
        )
        .arg(
            Arg::new("geo")
                .short('g')
                .long("geo")
                .help("Show geographical information")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let target = matches.get_one::<String>("target").unwrap();
    let max_hops: u8 = matches.get_one::<String>("max-hops")
        .unwrap()
        .parse()
        .unwrap_or(30);
    let timeout: u64 = matches.get_one::<String>("timeout")
        .unwrap()
        .parse()
        .unwrap_or(5);
    let show_geo = matches.get_flag("geo");

    println!("{}", format!("Tracing route to {} with maximum {} hops", target, max_hops).cyan().bold());
    println!();

    let mut tracer = IpTracer::new(max_hops, timeout, show_geo)?;
    tracer.trace(target).await?;

    Ok(())
}