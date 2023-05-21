from src.order_book import OrderBook


def test_price_methods():
    orderBook = OrderBook()
    order_1 = {
        "action": "add",
        "side": "bid",
        "price": 10,
        "quantity": 1,
        "client_id": 1,
    }
    order_2 = {
        "action": "add",
        "side": "bid",
        "price": 50,
        "quantity": 2,
        "client_id": 1,
    }
    order_3 = {
        "action": "add",
        "side": "ask",
        "price": 100,
        "quantity": 3,
        "client_id": 1,
    }
    order_4 = {
        "action": "add",
        "side": "ask",
        "price": 75,
        "quantity": 4,
        "client_id": 1,
    }
    order_5 = {
        "action": "add",
        "side": "bid",
        "price": 20,
        "quantity": 5,
        "client_id": 1,
    }
    for order in [order_1, order_2, order_3, order_4, order_5]:
        orderBook.submit_order(order)
    assert orderBook.best_bid() == 50
    assert orderBook.best_ask() == 75
    assert orderBook.mid_price() == 87.5


def test_order_error_handling():
    orderBook = OrderBook()
    try:
        orderBook.submit_order(
            {"action": "add", "foo": "bid", "price": 10, "quantity": 1, "client_id": 1}
        )
    except Exception as err:
        assert (
            str(err)
            == "Order is not in correct format, must contain 'action', 'side', 'price', 'quantity', 'client_id'"
        )
