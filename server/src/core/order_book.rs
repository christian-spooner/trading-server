use serde::{Deserialize, Serialize};

pub trait OrderBook {
    fn new() -> Self;
    fn add_bid_order(&mut self, order: Order);
    fn add_ask_order(&mut self, order: Order);
    fn remove_bid_order(&mut self, id: u64) -> Result<&'static str, &'static str>;
    fn remove_ask_order(&mut self, id: u64) -> Result<&'static str, &'static str>;
    fn amend_bid_order(
        &mut self,
        id: u64,
        quantity: u64,
        price: f64,
    ) -> Result<&'static str, &'static str>;
    fn amend_ask_order(
        &mut self,
        id: u64,
        quantity: u64,
        price: f64,
    ) -> Result<&'static str, &'static str>;
    fn match_orders(&mut self) -> Result<(u64, u64, u64, f64), &'static str>;
    fn display(&self);
    fn get_bids(&self) -> [Order; 10];
    fn get_asks(&self) -> [Order; 10];
    fn get_volume_at_limit(&self, price: f64) -> u64;
    fn get_order_by_id(&self, id: u64) -> Result<Order, &'static str>;
    fn get_market_price(&self) -> Result<f64, &'static str>;
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub quantity: u64,
    pub price: f64,
}

impl Order {
    pub fn new(id: u64, side: Side, quantity: u64, price: f64) -> Order {
        Order {
            id,
            side,
            quantity,
            price,
        }
    }
}

impl Default for Order {
    fn default() -> Order {
        Order {
            id: 0,
            side: Side::Buy,
            quantity: 0,
            price: 0.0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PriceDatum {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub timestamp: u64,
}

impl ToString for Side {
    fn to_string(&self) -> String {
        match self {
            Side::Buy => "Buy".to_string(),
            Side::Sell => "Sell".to_string(),
        }
    }
}
