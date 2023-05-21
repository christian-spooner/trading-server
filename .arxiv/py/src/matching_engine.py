from src.order_book import OrderBook
from src.trading_client import TradingClient
from src.transaction_ledger import TransactionLedger


class MatchingEngine:
    def __init__(self, clients: list[TradingClient] = []):
        self.orderBook = OrderBook()
        self.clients = clients
        self.ledger = TransactionLedger()

    def _handle_transaction(self, buyer_id, seller_id, price, quantity):
        if buyer_id == seller_id:
            return Exception("Buyer identical to seller: transaction dropped")

        for client in self.clients:
            if buyer_id == client.id:
                client.balance -= price * quantity
                client.asset_balance += quantity
            elif seller_id == client.id:
                client.balance += price * quantity
                client.asset_balance -= quantity

        self.ledger.append_txn(
            {
                "buyer_id": buyer_id,
                "seller_id": seller_id,
                "price": price,
                "quantity": quantity,
            }
        )

    def _validate_txn(self, bid_order, ask_order):
        for client in self.clients:
            if bid_order["client_id"] == client.id:
                if client.balance < bid_order["price"] * bid_order["quantity"]:
                    bid_order["action"] = "cancel"
                    self.orderBook.submit_order(bid_order)
                    return False
            elif ask_order["client_id"] == client.id:
                if client.asset_balance < ask_order["quantity"]:
                    ask_order["action"] = "cancel"
                    self.orderBook.submit_order(ask_order)
                    return False
        return True

    """
    Public Methods
    """

    def add_client(self, client: TradingClient):
        self.clients.append(client)

    def cancel_order(self, order: dict):
        assert order["action"] == "cancel"
        self.orderBook.submit_order(order)

    def submit_order(self, order: dict, client: TradingClient):
        if (
            order["side"] == "bid"
            and client.balance < order["price"] * order["quantity"]
        ):
            return Exception("Insufficient balance")
        if order["side"] == "ask" and client.asset_balance < order["quantity"]:
            return Exception("Insufficient asset balance")
        self.orderBook.submit_order(order)

    def execute(self):
        if len(self.orderBook.bid_list) == 0 or len(self.orderBook.ask_list) == 0:
            return Exception("Order book empty")

        bid_order = self.orderBook.bid_list[0]
        ask_order = self.orderBook.ask_list[0]

        bid_index = 0

        while bid_order["client_id"] == ask_order["client_id"] and bid_index + 1 < len(
            self.orderBook.bid_list
        ):
            bid_index += 1
            bid_order = self.orderBook.bid_list[bid_index]

        if bid_order["price"] >= ask_order["price"]:
            if not self._validate_txn(bid_order, ask_order):
                return Exception("Invalidate transaction: execution dropped")

            self.orderBook.bid_list.pop(bid_index)
            self.orderBook.ask_list.pop(0)
            execution_price = (bid_order["price"] + ask_order["price"]) / 2
            execution_quantity = min(bid_order["quantity"], ask_order["quantity"])
            self._handle_transaction(
                bid_order["client_id"],
                ask_order["client_id"],
                execution_price,
                execution_quantity,
            )

    def display_clients(self):
        for client in self.clients:
            client.display()

    def get_txn_history(self):
        return self.ledger.get_txns()

    def get_total_txn_volume_per_second(self):
        return self.ledger.get_total_volume()

    def get_txn_volume_per_second(self):
        return self.ledger.get_volume_per_second()
