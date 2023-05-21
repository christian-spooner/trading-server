/*
1: order id
2: side (buy, sell)
3: quantity
4: price
5: status (new, filled, rejected)
6: volume at limit
7: trades
8: book
9: market price
10: market trades
11: market book
12: reason
*/

use chrono::{DateTime, Utc};

use crate::net::fix::*;

const DELIMITER: char = '|';

pub fn encode_message(message: &FixMessage) -> String {
    let (message_type, fields) = message;
    let message_type_str = match message_type {
        MessageType::NewOrder => "N",
        MessageType::ExecutionReport => "E",
        MessageType::OrderReplaceRequest => "O",
        MessageType::OrderCancelRequest => "C",
        MessageType::OrderStatusRequest => "S",
        MessageType::MarketDataRequest => "M",
        MessageType::MarketData => "D",
        MessageType::Reject => "R",
    };

    let fields_str = fields
        .iter()
        .map(|field| match field {
            MessageField::OrderId(id) => format!("1={}", id),
            MessageField::Side(side) => format!(
                "2={}",
                match side {
                    Side::Buy => "B",
                    Side::Sell => "S",
                }
            ),
            MessageField::Quantity(quantity) => format!("3={}", quantity),
            MessageField::Price(price) => format!("4={}", price),
            MessageField::Status(status) => format!(
                "5={}",
                match status {
                    OrderStatus::New => "N",
                    OrderStatus::Filled => "F",
                    OrderStatus::Rejected => "R",
                }
            ),
            MessageField::VolumeAtLimit(volume_at_limit) => format!("6={}", volume_at_limit),
            MessageField::Trades(trades) => {
                let trades_str = trades
                    .iter()
                    .map(|(quantity, price, datetime)| {
                        let timestamp = datetime.format("%Y%m%d-%H:%M:%S").to_string();
                        format!("{}@{}@{}", quantity, price, timestamp)
                    })
                    .collect::<Vec<_>>()
                    .join(",");
                format!("7={}", trades_str)
            }
            MessageField::Book(bids, asks) => {
                let bids_str = bids
                    .iter()
                    .map(|order| format!("{}@{}", order.quantity, order.price))
                    .collect::<Vec<_>>()
                    .join(",");
                let asks_str = asks
                    .iter()
                    .map(|order| format!("{}@{}", order.quantity, order.price))
                    .collect::<Vec<_>>()
                    .join(",");
                format!("8={}:{}", bids_str, asks_str)
            }
            MessageField::MarketPrice(market_price) => format!("9={}", market_price),
            MessageField::MarketTrades(market_trades) => format!("10={}", market_trades),
            MessageField::MarketBook(market_book) => format!("11={}", market_book),
            MessageField::Reason(reason) => format!("12={}", reason),
        })
        .collect::<Vec<String>>()
        .join(&DELIMITER.to_string());

    format!("{}{}{}", message_type_str, DELIMITER, fields_str)
}

pub fn decode_message(data: &str) -> Option<FixMessage> {
    let parts: Vec<&str> = data.split(DELIMITER).collect();
    if parts.is_empty() {
        return None;
    }

    let message_type = match parts[0] {
        "N" => MessageType::NewOrder,
        "E" => MessageType::ExecutionReport,
        "O" => MessageType::OrderReplaceRequest,
        "C" => MessageType::OrderCancelRequest,
        "S" => MessageType::OrderStatusRequest,
        "M" => MessageType::MarketDataRequest,
        "D" => MessageType::MarketData,
        "R" => MessageType::Reject,
        _ => return None,
    };

    let mut fields = Vec::new();
    for part in parts.iter().skip(1) {
        let field_parts: Vec<&str> = part.split('=').collect();
        if field_parts.len() != 2 {
            return None;
        }
        let tag = field_parts[0];
        let value = field_parts[1];

        let field = match tag {
            "1" => MessageField::OrderId(value.parse::<u64>().ok()?),
            "2" => MessageField::Side(match value {
                "B" => Side::Buy,
                "S" => Side::Sell,
                _ => return None,
            }),
            "3" => MessageField::Quantity(value.parse::<u64>().ok()?),
            "4" => MessageField::Price(value.parse::<f64>().ok()?),
            "5" => MessageField::Status(match value {
                "N" => OrderStatus::New,
                "F" => OrderStatus::Filled,
                "R" => OrderStatus::Rejected,
                _ => return None,
            }),
            "6" => MessageField::VolumeAtLimit(value.parse::<f64>().ok()?),
            "7" => {
                let trades: [(u64, f64, DateTime<Utc>); 10] = {
                    let mut trades_array: [(u64, f64, DateTime<Utc>); 10] = Default::default();
                    for (i, trade_str) in value.split(',').rev().take(10).enumerate() {
                        let trade_parts: Vec<&str> = trade_str.split('@').collect();
                        let quantity = trade_parts[0].parse::<u64>().ok()?;
                        let price = trade_parts[1].parse::<f64>().ok()?;
                        let datetime = Utc::now();
                        trades_array[i] = (quantity, price, datetime);
                    }
                    trades_array
                };
                MessageField::Trades(trades)
            }
            "8" => {
                let book_parts: Vec<&str> = value.split(':').collect();
                if book_parts.len() != 2 {
                    return None;
                }
                let bids = parse_orders(book_parts[0], true)?;
                let asks = parse_orders(book_parts[1], false)?;
                MessageField::Book(bids, asks)
            }
            "9" => MessageField::MarketPrice(value.parse::<bool>().ok()?),
            "10" => MessageField::MarketTrades(value.parse::<bool>().ok()?),
            "11" => MessageField::MarketBook(value.parse::<bool>().ok()?),
            "12" => MessageField::Reason(value.to_owned()),
            _ => return None,
        };
        fields.push(field);
    }

    Some((message_type, fields))
}

fn parse_orders(orders_str: &str, is_buy: bool) -> Option<[OrderData; 10]> {
    let mut orders = [OrderData {
        id: 0,
        side: Side::Buy,
        quantity: 0,
        price: 0.0,
    }; 10];
    let mut i = 0;
    for order_str in orders_str.split(',') {
        if i >= 10 {
            return None;
        }
        let order_parts: Vec<&str> = order_str.split('@').collect();
        if order_parts.len() != 2 {
            return None;
        }
        let quantity = order_parts[0].parse::<u64>().ok()?;
        let price = order_parts[1].parse::<f64>().ok()?;
        let mut side = Side::Buy;
        if !is_buy {
            side = Side::Sell;
        };
        let order = OrderData {
            id: 0,
            side,
            quantity,
            price,
        };
        orders[i] = order;
        i += 1;
    }
    Some(orders)
}
