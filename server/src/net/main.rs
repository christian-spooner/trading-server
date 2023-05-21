use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use super::api::start_api;
use super::codec::{decode_message, encode_message};
use super::fix::{FixMessage, MessageField, MessageType, OrderData};
use crate::core::{engine, vector::VectorOrderBook};
use array_init::array_init;

#[tokio::main]
pub async fn start() {
    let n = Arc::new(Mutex::new(engine::Engine::<VectorOrderBook>::new()));
    println!("Engine created");
    let mut handles = vec![];

    let n_1 = n.clone();
    handles.push(tokio::spawn(async move {
        start_api(n_1).await;
    }));

    let n_2 = n.clone();
    handles.push(tokio::spawn(async move {
        start_socket(n_2).await;
    }));

    let n_3 = n.clone();
    handles.push(tokio::spawn(async move {
        start_matching(n_3).await;
    }));

    futures::future::join_all(handles).await;
}

async fn start_socket(n: Arc<Mutex<engine::Engine<VectorOrderBook>>>) {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    loop {
        // the second item contains the IP and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();
        println!("Accepted connection from {}", socket.peer_addr().unwrap());
        process_request(socket, n.clone())
            .await
            .unwrap_or_else(|e| {
                eprintln!("Error processing connection: {}", e);
            });
    }
}

async fn start_matching(n: Arc<Mutex<engine::Engine<VectorOrderBook>>>) {
    loop {
        let mut n_data = n.lock().await;
        let _result = match n_data.match_orders() {
            Ok((match_quantity, match_price)) => {
                println!("Matched {} @ {}", match_quantity, match_price);
            }
            Err(e) => {
                if e != "No matching orders" {
                    eprintln!("Error matching orders: {}", e);
                }
            }
        };
        thread::sleep(Duration::from_millis(1000));
    }
}

async fn process_request(
    mut socket: TcpStream,
    n_locked: Arc<Mutex<engine::Engine<VectorOrderBook>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg_len = socket.read_u32().await?.to_be(); // Read the length of the message
    let mut buf = vec![0u8; msg_len as usize];
    socket.read_exact(&mut buf).await?;

    let str_from_bytes = std::str::from_utf8(&buf)?;
    match decode_message(str_from_bytes) {
        Some(fix_msg) => engine_response(&mut socket, n_locked, fix_msg).await,
        None => {
            eprintln!("Error decoding FixMessage");
            return Err("Error decoding FixMessage".into());
        }
    }
}

async fn engine_response(
    socket: &mut TcpStream,
    n_locked: Arc<Mutex<engine::Engine<VectorOrderBook>>>,
    fix_msg: FixMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut n = n_locked.lock().await;

    let response: FixMessage = match fix_msg.0 {
        MessageType::NewOrder => {
            if let [MessageField::Side(side), MessageField::Quantity(quantity), MessageField::Price(price)] =
                fix_msg.1.as_slice()
            {
                match n.add_order(side.to_string(), *quantity, *price) {
                    Ok(id) => (
                        MessageType::ExecutionReport,
                        vec![MessageField::OrderId(id)],
                    ),
                    Err(e) => {
                        eprintln!("Error adding order: {}", e);
                        (
                            MessageType::Reject,
                            vec![MessageField::Reason(e.to_string())],
                        )
                    }
                }
            } else {
                return Err("Error processing NewOrder message".into());
            }
        }
        MessageType::OrderReplaceRequest => {
            if let [MessageField::OrderId(id), MessageField::Quantity(quantity), MessageField::Price(price)] =
                fix_msg.1.as_slice()
            {
                match n.amend_order(*id, *quantity, *price) {
                    Ok(_) => (
                        MessageType::ExecutionReport,
                        vec![MessageField::OrderId(*id)],
                    ),
                    Err(e) => {
                        eprintln!("Error amending order: {}", e);
                        (
                            MessageType::Reject,
                            vec![MessageField::Reason(e.to_string())],
                        )
                    }
                }
            } else {
                return Err("Error processing OrderReplaceRequest message".into());
            }
        }
        MessageType::OrderCancelRequest => {
            if let [MessageField::OrderId(id)] = fix_msg.1.as_slice() {
                match n.cancel_order(*id) {
                    Ok(_) => (
                        MessageType::ExecutionReport,
                        vec![MessageField::OrderId(*id)],
                    ),
                    Err(e) => {
                        eprintln!("Error cancelling order: {}", e);
                        (
                            MessageType::Reject,
                            vec![MessageField::Reason(e.to_string())],
                        )
                    }
                }
            } else {
                return Err("Error processing OrderCancelRequest message".into());
            }
        }
        MessageType::OrderStatusRequest => {
            if let [MessageField::OrderId(id)] = fix_msg.1.as_slice() {
                match n.get_execution_status(*id) {
                    Ok(status) => (
                        MessageType::ExecutionReport,
                        vec![MessageField::OrderId(*id), MessageField::Status(status)],
                    ),
                    Err(e) => {
                        eprintln!("Error getting order status: {}", e);
                        (
                            MessageType::Reject,
                            vec![MessageField::Reason(e.to_string())],
                        )
                    }
                }
            } else {
                return Err("Error processing OrderStatusRequest message".into());
            }
        }
        MessageType::MarketDataRequest => {
            let mut response_fields = vec![];

            if let Some(MessageField::VolumeAtLimit(price)) = fix_msg
                .1
                .iter()
                .find(|&field| matches!(field, MessageField::VolumeAtLimit(_)))
            {
                let volume_at_limit = MessageField::Quantity(n.get_volume_at_limit(*price));
                response_fields.push(volume_at_limit);
            }

            if let Some(MessageField::MarketPrice(_)) = fix_msg
                .1
                .iter()
                .find(|&field| matches!(field, MessageField::MarketPrice(_)))
            {
                let market_price = MessageField::Price(n.get_market_price().unwrap());
                response_fields.push(market_price);
            }

            if let Some(MessageField::MarketTrades(_)) = fix_msg
                .1
                .iter()
                .find(|&field| matches!(field, MessageField::MarketTrades(_)))
            {
                let trades = n.get_trade_history();
                let trades_field = MessageField::Trades(trades);
                response_fields.push(trades_field);
            }

            if let Some(MessageField::MarketBook(_)) = fix_msg
                .1
                .iter()
                .find(|&field| matches!(field, MessageField::MarketBook(_)))
            {
                let (bids, asks) = n.get_book();
                let bids_data: [OrderData; 10] = array_init(|i| bids[i].into());
                let asks_data: [OrderData; 10] = array_init(|i| asks[i].into());
                let market_book_field = MessageField::Book(bids_data, asks_data);
                response_fields.push(market_book_field);
            }
            (MessageType::MarketData, response_fields)
        }
        _ => (
            MessageType::Reject,
            vec![MessageField::Reason("Invalid message type".into())],
        ),
    };

    let response_buf = encode_message(&response);
    let response_bytes = response_buf.as_bytes().to_vec();
    let response_len = response_bytes.len() as u32;
    socket.write_u32(response_len.to_be()).await?;
    socket.write_all(&response_bytes).await?;

    drop(n);
    Ok(())
}
