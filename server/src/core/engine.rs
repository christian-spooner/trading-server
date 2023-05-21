use crate::net::fix::OrderStatus;

use super::order_book::{Order, OrderBook, Side};
use chrono::{DateTime, Utc};

pub struct Engine<T: OrderBook> {
    pub book: T,
    pub id: u64,
    pub history: Vec<u64>,
    pub trade_history: Vec<(u64, f64, DateTime<Utc>)>,
}

impl<T: OrderBook> Engine<T> {
    pub fn new() -> Engine<T> {
        Engine {
            book: T::new(),
            id: 0,
            history: Vec::new(),
            trade_history: Vec::new(),
        }
    }

    pub fn add_order(
        &mut self,
        side: String,
        quantity: u64,
        price: f64,
    ) -> Result<u64, &'static str> {
        let side = match side.as_str() {
            "Buy" => Side::Buy,
            "Sell" => Side::Sell,
            _ => return Err("Invalid order side"),
        };
        self.id += 1;
        let price_2dp = format!("{:.2}", price).parse().unwrap();
        let order = Order::new(self.id, side, quantity, price_2dp);
        match order.side {
            Side::Buy => self.book.add_bid_order(order),
            Side::Sell => self.book.add_ask_order(order),
        };
        Ok(self.id)
    }

    pub fn amend_order(&mut self, id: u64, quantity: u64, price: f64) -> Result<u64, String> {
        let price_2dp = format!("{:.2}", price).parse().unwrap();
        let result = self.book.amend_bid_order(id, quantity, price_2dp);
        if result.is_ok() {
            return Ok(id);
        } else {
            let result = self.book.amend_ask_order(id, quantity, price);
            if result.is_ok() {
                return Ok(id);
            } else {
                return Err(format!("Order not found for id: {}", id));
            }
        }
    }

    pub fn cancel_order(&mut self, id: u64) -> Result<(), String> {
        let result = self.book.remove_bid_order(id);
        if result.is_ok() {
            return Ok(());
        } else {
            let result = self.book.remove_ask_order(id);
            if result.is_ok() {
                return Ok(());
            } else {
                return Err(format!("Order not found for id: {}", id));
            }
        }
    }

    pub fn match_orders(&mut self) -> Result<(u64, f64), &'static str> {
        let result = self.book.match_orders();
        match result {
            Ok((bid_id, ask_id, quantity, price)) => {
                self.history.push(bid_id);
                self.history.push(ask_id);
                self.trade_history.push((quantity, price, Utc::now()));
                Ok((quantity, price))
            }
            Err(e) => Err(e),
        }
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        self.book.display();
    }

    pub fn get_book(&self) -> ([Order; 10], [Order; 10]) {
        let bids = self.book.get_bids();
        let asks = self.book.get_asks();
        (bids, asks)
    }

    pub fn get_volume_at_limit(&self, price: f64) -> u64 {
        self.book.get_volume_at_limit(price)
    }

    pub fn get_order_status(&self, id: u64) -> Result<String, &str> {
        let result = self.book.get_order_by_id(id);
        match result {
            Ok(order) => {
                let status = format!(
                    "New -- {} {} @ {} ({})",
                    order.side.to_string(),
                    order.quantity,
                    order.price,
                    order.id
                );
                Ok(status)
            }
            Err(e) => {
                let result = self.history.contains(&id);
                if result {
                    Ok("Filled".to_string())
                } else {
                    Err(e)
                }
            }
        }
    }

    pub fn get_execution_status(&self, id: u64) -> Result<OrderStatus, &str> {
        let result = self.book.get_order_by_id(id);
        match result {
            Ok(_) => Ok(OrderStatus::New),
            Err(_) => {
                let result = self.history.contains(&id);
                if result {
                    Ok(OrderStatus::Filled)
                } else {
                    Ok(OrderStatus::Rejected)
                }
            }
        }
    }

    pub fn get_market_price(&self) -> Result<f64, &str> {
        self.book.get_market_price()
    }

    pub fn get_trade_history(&self) -> [(u64, f64, DateTime<Utc>); 10] {
        let n = self.trade_history.len();
        let m = std::cmp::min(10, n);
        let mut array: [(u64, f64, DateTime<Utc>); 10] = Default::default();
        for i in 0..m {
            array[i] = self.trade_history[n - 1 - i].clone();
        }
        array
    }
}
