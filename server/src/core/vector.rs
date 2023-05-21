use super::order_book::{Order, OrderBook};
use std::cmp::min;

pub struct VectorOrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

impl VectorOrderBook {
    fn sort_bids(&mut self) {
        self.bids
            .sort_unstable_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    }

    fn sort_asks(&mut self) {
        self.asks
            .sort_unstable_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    }
}

impl OrderBook for VectorOrderBook {
    fn new() -> VectorOrderBook {
        VectorOrderBook {
            bids: Vec::new(),
            asks: Vec::new(),
        }
    }

    fn add_bid_order(&mut self, order: Order) {
        self.bids.push(order);
        self.sort_bids();
    }

    fn add_ask_order(&mut self, order: Order) {
        self.asks.push(order);
        self.sort_asks();
    }

    fn remove_bid_order(&mut self, id: u64) -> Result<&'static str, &'static str> {
        match self.bids.iter().position(|x| x.id == id) {
            Some(i) => {
                self.bids.remove(i);
                Ok("Success: order removed")
            }
            None => Err("Failure: order not found"),
        }
    }

    fn remove_ask_order(&mut self, id: u64) -> Result<&'static str, &'static str> {
        match self.bids.iter().position(|x| x.id == id) {
            Some(i) => {
                self.asks.remove(i);
                Ok("Success: order removed")
            }
            None => Err("Failure: order not found"),
        }
    }

    fn amend_bid_order(
        &mut self,
        id: u64,
        quantity: u64,
        price: f64,
    ) -> Result<&'static str, &'static str> {
        let index = self.bids.iter().position(|x| x.id == id);
        if let Some(index) = index {
            self.bids[index].quantity = quantity;
            self.bids[index].price = price;
            self.sort_bids();
            Ok("Success: order amended")
        } else {
            Err("Failure: order not found")
        }
    }

    fn amend_ask_order(
        &mut self,
        id: u64,
        quantity: u64,
        price: f64,
    ) -> Result<&'static str, &'static str> {
        let index = self.asks.iter().position(|x| x.id == id);
        if let Some(index) = index {
            self.asks[index].quantity = quantity;
            self.asks[index].price = price;
            self.sort_asks();
            Ok("Success: order amended")
        } else {
            Err("Failure: order not found")
        }
    }

    fn match_orders(&mut self) -> Result<(u64, u64, u64, f64), &'static str> {
        if self.bids.is_empty() || self.asks.is_empty() {
            return Err("No matching orders");
        }

        let best_bid = &self.bids[0];
        let best_ask = &self.asks[0];

        if best_bid.price >= best_ask.price {
            let match_quantity = min(best_bid.quantity, best_ask.quantity);
            let match_price = best_ask.price;
            let best_bid_id = best_bid.id;
            let best_ask_id = best_ask.id;
            self.bids.remove(0);
            self.asks.remove(0);
            Ok((best_bid_id, best_ask_id, match_quantity, match_price))
        } else {
            Err("No matching orders")
        }
    }

    fn display(&self) {
        println!("Bids:");
        for order in &self.bids {
            println!("  {} @ {} ({})", order.quantity, order.price, order.id);
        }

        println!("Asks:");
        for order in &self.asks {
            println!("  {} @ {} ({})", order.quantity, order.price, order.id);
        }
    }

    fn get_bids(&self) -> [Order; 10] {
        let n = std::cmp::min(10, self.bids.len());
        let mut array: [Order; 10] = Default::default();
        for i in 0..n {
            array[i] = self.bids[i].clone();
        }
        array
    }

    fn get_asks(&self) -> [Order; 10] {
        let n = std::cmp::min(10, self.asks.len());
        let mut array: [Order; 10] = Default::default();
        for i in 0..n {
            array[i] = self.asks[i].clone();
        }
        array
    }

    fn get_volume_at_limit(&self, price: f64) -> u64 {
        let mut volume = 0;
        for order in self.bids.iter().chain(self.asks.iter()) {
            if order.price == price {
                volume += order.quantity;
            }
        }
        volume
    }

    fn get_order_by_id(&self, id: u64) -> Result<Order, &'static str> {
        for order in self.bids.iter().chain(self.asks.iter()) {
            if order.id == id {
                return Ok(order.clone());
            }
        }
        Err("Order not found")
    }

    fn get_market_price(&self) -> Result<f64, &'static str> {
        if self.bids.is_empty() || self.asks.is_empty() {
            Err("No price available")
        } else {
            let best_bid = &self.bids[0];
            let best_ask = &self.asks[0];
            Ok((best_bid.price + best_ask.price) / 2.0)
        }
    }
}
