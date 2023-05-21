use std::fmt;

use time::OffsetDateTime;

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

#[derive(Debug, PartialEq)]
pub enum MessageField {
    OrderId(u64),
    Side(Side),
    Quantity(u64),
    Price(f64),
    Status(OrderStatus),
    VolumeAtLimit(f64),
    Trades([(u64, f64, OffsetDateTime); 10]),
    Book([OrderData; 10], [OrderData; 10]),
    MarketPrice(bool),
    MarketTrades(bool),
    MarketBook(bool),
    Reason(String),
}

pub type FixMessage = (MessageType, Vec<MessageField>);
