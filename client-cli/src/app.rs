use super::codec::{decode_message, encode_message};
use crate::fix::{self, Side};
use fix::{FixMessage, MessageField, MessageType};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

async fn send_fix_message(
    fix_msg: FixMessage,
    stream: &mut TcpStream,
) -> Result<FixMessage, Box<dyn std::error::Error>> {
    let buf = encode_message(&fix_msg);
    let bytes = buf.as_bytes().to_vec();
    let len = bytes.len() as u32;

    stream.write_u32(len.to_be()).await?;
    stream.write_all(&bytes).await?;

    let response_len = stream.read_u32().await?.to_be();
    let mut response = vec![0u8; response_len as usize];
    stream.read_exact(&mut response).await?;
    let response_str = String::from_utf8(response)?;

    decode_message(&response_str).ok_or_else(|| {
        eprintln!("Error decoding FixMessage");
        "Error decoding FixMessage".into()
    })
}

pub async fn send_order(
    order: &str,
    stream: &mut TcpStream,
    is_buy: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<&str> = order.split(",").collect();
    let quantity = args[0].parse::<u64>().unwrap();
    let price = args[1].parse::<f64>().unwrap();
    let side = if is_buy { Side::Buy } else { Side::Sell };

    println!("Sending order: {}", order);
    let fix_msg = (
        MessageType::NewOrder,
        vec![
            MessageField::Side(side),
            MessageField::Quantity(quantity),
            MessageField::Price(price),
        ],
    );

    let response_msg = send_fix_message(fix_msg, stream).await?;
    println!("Response: {:?}", response_msg);
    Ok(())
}

pub async fn amend_order(
    order: &str,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<&str> = order.split(",").collect();
    let id = args[0].parse::<u64>().unwrap();
    let quantity = args[1].parse::<u64>().unwrap();
    let price = args[2].parse::<f64>().unwrap();
    println!("Amending order: {}", order);
    let fix_msg: FixMessage = (
        MessageType::OrderReplaceRequest,
        vec![
            MessageField::OrderId(id),
            MessageField::Quantity(quantity),
            MessageField::Price(price),
        ],
    );

    let response_msg = send_fix_message(fix_msg, stream).await?;
    println!("Response: {:?}", response_msg);
    Ok(())
}

pub async fn cancel_order(
    id: &str,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Cancelling order with ID: {}", id);
    let fix_msg: FixMessage = (
        MessageType::OrderCancelRequest,
        vec![MessageField::OrderId(id.parse::<u64>().unwrap())],
    );

    let response_msg = send_fix_message(fix_msg, stream).await?;
    println!("Response: {:?}", response_msg);
    Ok(())
}

pub async fn status_report(
    id: &str,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting status report for order with ID: {}", id);
    let fix_msg: FixMessage = (
        MessageType::OrderStatusRequest,
        vec![MessageField::OrderId(id.parse::<u64>().unwrap())],
    );

    let response_msg = send_fix_message(fix_msg, stream).await?;
    println!("Response: {:?}", response_msg);
    Ok(())
}

pub async fn volume_at_limit(
    price: &str,
    stream: &mut TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting volume at limit: {}", price);
    let price = price.parse::<f64>().unwrap();
    let fix_msg: FixMessage = (
        MessageType::MarketDataRequest,
        vec![MessageField::VolumeAtLimit(price)],
    );

    let response_msg = send_fix_message(fix_msg, stream).await?;
    println!("Response: {:?}", response_msg);
    Ok(())
}

pub async fn market_price(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting market price...");
    let fix_msg: FixMessage = (
        MessageType::MarketDataRequest,
        vec![MessageField::MarketPrice(true)],
    );

    let response_msg = send_fix_message(fix_msg, stream).await?;
    println!("Response: {:?}", response_msg);
    Ok(())
}

pub async fn trades(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting recent trades...");
    let fix_msg: FixMessage = (
        MessageType::MarketDataRequest,
        vec![MessageField::MarketTrades(true)],
    );

    let response_msg = send_fix_message(fix_msg, stream).await?;
    println!("Response: {:?}", response_msg);
    Ok(())
}

pub async fn book(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    println!("Getting order book...");
    let fix_msg: FixMessage = (
        MessageType::MarketDataRequest,
        vec![MessageField::MarketBook(true)],
    );

    let response_msg = send_fix_message(fix_msg, stream).await?;
    println!("Response: {:?}", response_msg);
    Ok(())
}
