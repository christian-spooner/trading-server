#[cfg(test)]
mod tests {
    use server::core::engine::Engine;
    use server::core::order_book::{Order, OrderBook, Side};
    use server::core::vector::VectorOrderBook;

    #[test]
    fn order_book() {
        let mut book: VectorOrderBook = VectorOrderBook::new();
        let order1 = Order::new(1, Side::Buy, 50, 100.0);
        let order2 = Order::new(2, Side::Buy, 100, 99.0);
        let order3 = Order::new(3, Side::Sell, 50, 101.0);
        let order4 = Order::new(4, Side::Sell, 50, 102.0);
        book.add_bid_order(order1);
        book.add_bid_order(order2);
        book.add_ask_order(order3);
        book.add_ask_order(order4);
        book.display();
    }

    #[test]
    #[allow(unused_must_use)]
    fn engine() {
        let mut n = Engine::<VectorOrderBook>::new();
        n.add_order("Buy".to_string(), 50, 100.0);
        n.add_order("Buy".to_string(), 100, 99.0);
        n.add_order("Sell".to_string(), 50, 101.0);
        n.add_order("Sell".to_string(), 50, 102.0);
        n.display();
        n.match_orders();
        n.display();
    }
}
