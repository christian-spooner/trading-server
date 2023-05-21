mod app;
mod codec;
mod fix;

use crate::app::{
    amend_order, book, cancel_order, market_price, send_order, status_report, trades,
    volume_at_limit,
};

use clap::{App, Arg};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Trading Client")
        .version("0.1.0")
        .author("Christian Spooner")
        .about("A command line interface for interacting with a trading client")
        .arg(
            Arg::with_name("buy")
                .short("b")
                .long("buy")
                .value_name("ORDER")
                .help("Send a bid order to the trading server")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("sell")
                .short("s")
                .long("sell")
                .value_name("ORDER")
                .help("Send an ask order to the trading server")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("amend")
                .short("a")
                .long("amend")
                .value_name("ORDER")
                .help("Amend an existing order by ID")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cancel")
                .short("c")
                .long("cancel")
                .value_name("ID")
                .help("Cancel an existing order by ID")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("report")
                .short("r")
                .long("report")
                .value_name("ID")
                .help("Get the status of an order by ID")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("book")
                .long("book")
                .help("Displays the order book")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("volume")
                .short("v")
                .long("volume")
                .value_name("LIMIT")
                .help("Get volume at a price level")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("V")
                .long("verbose")
                .help("Prints additional information")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("price")
                .short("p")
                .long("price")
                .value_name("MARKET")
                .help("Get the current market price")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("trades")
                .short("t")
                .long("trades")
                .value_name("TRADES")
                .help("Get recent trade history")
                .takes_value(false),
        )
        .get_matches();

    let mut stream = TcpStream::connect("127.0.0.1:6379").await?;
    println!("Connected to server at {}", stream.peer_addr()?);
    let mut result = Ok(());

    if let Some(order) = matches.value_of("buy") {
        result = send_order(order, &mut stream, true).await;
    }

    if let Some(order) = matches.value_of("sell") {
        result = send_order(order, &mut stream, false).await;
    }

    if let Some(amend) = matches.value_of("amend") {
        result = amend_order(amend, &mut stream).await;
    }

    if let Some(id) = matches.value_of("cancel") {
        result = cancel_order(id, &mut stream).await;
    }

    if let Some(id) = matches.value_of("report") {
        result = status_report(id, &mut stream).await;
    }

    if let Some(volume) = matches.value_of("volume") {
        result = volume_at_limit(volume, &mut stream).await;
    }

    if matches.is_present("price") {
        result = market_price(&mut stream).await;
    }

    if matches.is_present("book") {
        result = book(&mut stream).await;
    }

    if matches.is_present("trades") {
        result = trades(&mut stream).await;
    }

    if matches.is_present("verbose") {
        if result.is_ok() {
            println!("OK");
        } else {
            println!("ERR: {:?}", result);
        }
    }
    Ok(())
}
