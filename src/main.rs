use std::fmt::Result;
use std::net::Ipv4Addr;
use std::env;

mod websocket;
use websocket::node;
use websocket::server;

mod storage;
mod sys_info;

#[tokio::main]
async fn main() -> Result {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() < 1 {
        println!("Please let us know if the agent is node or main")
    }
    match args[0].as_str() {
        "main" => {
            server::start_server().await;
        }
        "node" => client_main(args).await,
        _ => {
            print!("Provide option main or node. Ex - cargo run main (or) cargo run node")
        }
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
