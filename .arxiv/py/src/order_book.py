"""
What is an order book?
An order book is a list of all buy and sell orders for a particular security.

Feature requirements
    Add - O(log M) for the first order at a particular limit, O(1) for all others
    Cancel - O(1)
    Execute - O(1)
    GetVolumeAtLimit - O(1)
    GetBestBid/Offer - O(1)

    M: number of price limits
    N: number of orders
"""


class OrderBook:
    """
    Limit Order Book: order submitted at maximum/minimum price participant is willing to buy/sell
    """

    def __init__(self):
        self.bid_list = []
        self.ask_list = []
        self.count = 0

    def _get_order_list(self, order: dict):
        if order["side"] == "bid":
            order_list = self.bid_list
        elif order["side"] == "ask":
            order_list = self.ask_list
        else:
            raise Exception("Unable to get order list")
        return order_list

    def _handle_add(self, order: dict):
        if order["side"] == "bid":
            self.bid_list.append(order)
            self.bid_list.sort(key=lambda x: x["price"], reverse=True)
        elif order["side"] == "ask":
            self.ask_list.append(order)
            self.ask_list.sort(key=lambda x: x["price"])

    def _handle_modify(self, order: dict):
        order_list = self._get_order_list(order)
        for _order in order_list:
            if _order["id"] == order["id"]:
                _order["id"] = order["id"]
                break

    def _handle_cancel(self, order: dict):
        order_list = self._get_order_list(order)
        try:
            order_list.remove(order)
        except Exception:
            raise Exception("Unable to cancel order")

    def _append_id(self, order: dict) -> dict:
        order["id"] = self.count
        self.count += 1
        return order

    """
    Public Methods
    """

    def submit_order(self, order: dict):
        try:
            assert (
                "action" in order
                and "side" in order
                and "price" in order
                and "quantity" in order
                and "client_id" in order
            )
        except Exception:
            raise Exception(
                "Order is not in correct format, must contain 'action', 'side', 'price', 'quantity', 'client_id'"
            )

        if "id" not in order:
            order = self._append_id(order)

        if order["action"] == "add":
            self._handle_add(order)
        elif order["action"] == "modify":
            self._handle_modify(order)
        elif order["action"] == "cancel":
            self._handle_cancel(order)
        else:
            raise Exception("Unable to process order")
        return order["id"]

    def best_bid(self) -> float:
        return self.bid_list[0]["price"]

    def best_ask(self) -> float:
        return self.ask_list[0]["price"]

    def mid_price(self) -> float:
        return self.best_bid() + self.best_ask() / 2

    def display(self, display_limit):
        i, j = 0, 0
        print("\nBIDS")
        for order in self.bid_list:
            if i < display_limit:
                print("%d %d %d" % (order["id"], order["price"], order["quantity"]))
                i += 1
            else:
                break
        print("OFFERS")
        for order in self.ask_list:
            if j < display_limit:
                print("%d %d %d" % (order["id"], order["price"], order["quantity"]))
                j += 1
            else:
                break
