use std::collections::BTreeMap;

pub struct BTreeOrderBook {
    bids: BTreeMap<f64, Vec<Order>>,
    asks: BTreeMap<f64, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> BTreeOrderBook {
        BTreeOrderBook {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    fn add_bid_order(&mut self, order: Order) {
        // insert the new order into the bids map, sorted by price
        let price = order.price;
        self.bids.entry(price).or_insert(Vec::new()).push(order);
    }

    fn add_ask_order(&mut self, order: Order) {
        // insert the new order into the asks map, sorted by price
        let price = order.price;
        self.asks.entry(price).or_insert(Vec::new()).push(order);
    }

    fn remove_bid_order(&mut self, order: &Order) {
        // remove the specified order from the bids map
        let price = order.price;
        if let Some(orders) = self.bids.get_mut(&price) {
            let index = orders.iter().position(|x| x == order).unwrap();
            orders.remove(index);
        }
    }

    fn remove_ask_order(&mut self, order: &Order) {
        // remove the specified order from the asks map
        let price = order.price;
        if let Some(orders) = self.asks.get_mut(&price) {
            let index = orders.iter().position(|x| x == order).unwrap();
            orders.remove(index);
        }
    }

    fn amend_bid_order(&mut self, order: &Order, price: f64, size: u64) {
        // amend the specified order in the bids map
        self.remove_bid_order(order);
        let mut order = order.clone();
        order.price = price;
        order.size = size;
        self.add_bid_order(order);
    }

    fn amend_ask_order(&mut self, order: &Order, price: f64, size: u64) {
        // amend the specified order in the asks map
        self.remove_ask_order(order);
        let mut order = order.clone();
        order.price = price;
        order.size = size;
        self.add_ask_order(order);
    }

    fn match_orders(&mut self) -> Result<(), &'static str> {
        // get the best bid and ask prices
        let best_bid = self.bids.keys().last().unwrap();
        let best_ask = self.asks.keys().next().unwrap();

        // if the best bid is greater than or equal to the best ask, execute a match
        if best_bid >= best_ask {
            // get the best bid and ask orders
            let best_bid_orders = self.bids.get_mut(best_bid).unwrap();
            let best_ask_orders = self.asks.get_mut(best_ask).unwrap();

            // get the first bid and ask orders
            let best_bid_order = best_bid_orders.first().unwrap();
            let best_ask_order = best_ask_orders.first().unwrap();

            // if the bid order size is greater than or equal to the ask order size, execute a match
            if best_bid_order.size >= best_ask_order.size {
                // remove the ask order from the asks map
                self.remove_ask_order(best_ask_order);

                // if the bid order size is greater than the ask order size, amend the bid order in the bids map
                if best_bid_order.size > best_ask_order.size {
                    self.amend_bid_order(
                        best_bid_order,
                        best_bid_order.price,
                        best_bid_order.size - best_ask_order.size,
                    );
                }
                // if the bid order size is equal to the ask order size, remove the bid order from the bids map
                else {
                    self.remove_bid_order(best_bid_order);
                }
            }
            // if the bid order size is less than the ask order size, execute a match
            else {
                // remove the bid order from the bids map
                self.remove_bid_order(best_bid_order);

                // amend the ask order in the asks map
                self.amend_ask_order(
                    best_ask_order,
                    best_ask_order.price,
                    best_ask_order.size - best_bid_order.size,
                );
            }
        }

        Ok(())
    }

    fn display(&self) {
        // display the bids and asks
        println!("Bids:");
        for (price, orders) in self.bids.iter().rev() {
            for order in orders {
                println!("{} {}", price, order.size);
            }
        }
        println!("Asks:");
        for (price, orders) in self.asks.iter() {
            for order in orders {
                println!("{} {}", price, order.size);
            }
        }
    }
}
