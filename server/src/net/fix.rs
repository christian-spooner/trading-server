use std::fmt;

use chrono::{DateTime, Utc};

use crate::core::order_book::{self, Order};
use crate::net::fix;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum MessageType {
    NewOrder,
    ExecutionReport,
    OrderReplaceRequest,
    OrderCancelRequest,
    OrderStatusRequest,
    MarketDataRequest,
    MarketData,
    Reject,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Side {
    Buy,
    Sell,
}

impl ToString for Side {
    fn to_string(&self) -> String {
        match self {
            Side::Buy => "Buy".to_string(),
            Side::Sell => "Sell".to_string(),
        }
    }
}

impl From<order_book::Side> for Side {
    fn from(side: order_book::Side) -> Self {
        match side {
            order_book::Side::Buy => Self::Buy,
            order_book::Side::Sell => Self::Sell,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum OrderStatus {
    New,
    Filled,
    Rejected,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderStatus::New => write!(f, "New"),
            OrderStatus::Filled => write!(f, "Filled"),
            OrderStatus::Rejected => write!(f, "Rejected"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct OrderData {
    pub id: u64,
    pub side: Side,
    pub quantity: u64,
    pub price: f64,
}

impl fmt::Display for OrderData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order ID: {}\nSide: {}\nQuantity: {}\nPrice: {:.2}\n",
            self.id,
            self.side.to_string(),
            self.quantity,
            self.price,
        )
    }
}

impl From<Order> for OrderData {
    fn from(order: Order) -> Self {
        Self {
            id: order.id,
            side: fix::Side::from(order.side),
            quantity: order.quantity,
            price: order.price,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageField {
    OrderId(u64),
    Side(Side),
    Quantity(u64),
    Price(f64),
    Status(OrderStatus),
    VolumeAtLimit(f64),
    Trades([(u64, f64, DateTime<Utc>); 10]),
    Book([OrderData; 10], [OrderData; 10]),
    MarketPrice(bool),
    MarketTrades(bool),
    MarketBook(bool),
    Reason(String),
}

pub type FixMessage = (MessageType, Vec<MessageField>);
