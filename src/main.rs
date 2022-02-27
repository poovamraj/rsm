use std::net::Ipv4Addr;
use std::{fmt::Result};
use std::ffi::OsStr;

use futures::StreamExt;
use heim::units::information;
use std::time::Duration;
use std::usize;

#[cfg(unix)]
use heim::cpu::os::unix::loadavg;
use heim::{
    process::{self, Process, ProcessResult},
    units::{ratio, Ratio},
};
use std::env;

use futures_util::{stream::TryStreamExt};

mod websocket;
use websocket::server;
use websocket::node;


#[tokio::main]
async fn main() -> Result<> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() < 1 {
        println!("Please let us know if the agent is node or main")
    }
    match args[0].as_str() {
        "main" => { server::start_server().await; }
        "node" => { client_main(args).await }
        _ => { print!("Provide option main or node. Ex - cargo run main (or) cargo run node") }
    }
    Ok(())
}

async fn client_main(mut args: Vec<String>) {
    args.remove(0);
    if args.len() < 2 {
        println!("Please provide the IP Address and Port to the main agent");
        println!("Ex - cargo run node 192.168.100.32 8080");
    }
    let port = str::parse::<u16>(&args[1]).expect("Provided Port value is not a number");
    let ip = str::parse::<Ipv4Addr>(&args[0]).expect("Provided IPv4 value is not valid");

    println!("running node");
    node::connect(&ip, &port).await;
}

async fn _usage(process: Process) -> ProcessResult<(process::Process, Ratio)> {
    let usage_1 = process.cpu_usage().await?;
    tokio::time::sleep(Duration::from_millis(6000)).await;
    let usage_2 = process.cpu_usage().await?;
    Ok((process, usage_2 - usage_1))
}


async fn _top() -> ProcessResult<()> {
    let (one, five, fifteen) = loadavg().await?;
    println!(
        "Load average: {} {} {}",
        one.get::<ratio::ratio>(),
        five.get::<ratio::ratio>(),
        fifteen.get::<ratio::ratio>()
    );

    let processes = process::processes()
        .await?
        .map_ok(|process| {
            // Note that there is no `.await` here,
            // as we want to pass the returned future
            // into the `.try_buffer_unordered`.
            _usage(process)
        })
        .try_buffer_unordered(usize::MAX);
    futures::pin_mut!(processes);

    println!("| {:6} | {:40} | {:4} % |", "pid", "name", "CPU");

    while let Some(res) = processes.next().await {
        let (process, usage) = res?;

        println!(
            "| {:6} | {:40} | {:.2} |",
            process.pid(),
            process.name().await?,
            usage.get::<ratio::percent>()
        );
    }

    Ok(())
}

async fn _disk_usage() -> heim::Result<()> {
    println!(
        "{:<17} {:<10} {:<10} {:<10} {:<10} Mount",
        "Device", "Total, Mb", "Used, Mb", "Free, Mb", "Type"
    );

    let partitions = heim::disk::partitions_physical().await?;
    futures::pin_mut!(partitions);

    while let Some(part) = partitions.next().await {
        let part = part?;
        let usage = part.usage().await?;

        println!(
            "{:<17} {:<10} {:<10} {:<10} {:<10} {}",
            part.device()
                .unwrap_or_else(|| OsStr::new("N/A"))
                .to_string_lossy(),
            usage.total().get::<information::megabyte>(),
            usage.used().get::<information::megabyte>(),
            usage.free().get::<information::megabyte>(),
            part.file_system().as_str(),
            part.mount_point().to_string_lossy(),
        );
    }

    Ok(())
}
