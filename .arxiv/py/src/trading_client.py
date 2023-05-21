from dataclasses import dataclass


@dataclass
class TradingClient:
    id: int
    balance: float
    asset_balance: int

    def submit_order(self, order: dict):
        pass

    def display(self):
        print(f"{self.id} {self.balance} {self.asset_balance}")
