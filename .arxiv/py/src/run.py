import linecache
import os
import random
import time
import tracemalloc

from src.matching_engine import MatchingEngine
from src.trading_client import TradingClient


def display_top(snapshot, key_type="lineno", limit=5):
    snapshot = snapshot.filter_traces(
        (
            tracemalloc.Filter(False, "<frozen importlib._bootstrap>"),
            tracemalloc.Filter(False, "<unknown>"),
        )
    )
    top_stats = snapshot.statistics(key_type)

    print("Top %s lines" % limit)
    for index, stat in enumerate(top_stats[:limit], 1):
        frame = stat.traceback[0]
        # replace "/path/to/module/file.py" with "module/file.py"
        filename = os.sep.join(frame.filename.split(os.sep)[-2:])
        print(
            "#%s: %s:%s: %.1f KiB" % (index, filename, frame.lineno, stat.size / 1024)
        )
        line = linecache.getline(frame.filename, frame.lineno).strip()
        if line:
            print("    %s" % line)

    other = top_stats[limit:]
    if other:
        size = sum(stat.size for stat in other)
        print("%s other: %.1f KiB" % (len(other), size / 1024))
    total = sum(stat.size for stat in top_stats)
    print("Total allocated size: %.1f KiB" % (total / 1024))


def get_RNG_order(order_templates):
    order_1 = order_templates[0]
    order_2 = order_templates[1]
    RNG = random.uniform(0, 1)

    if RNG > 0.5:
        if RNG > 0.75:
            new_order = dict(order_1)
            new_order["price"] = int(random.uniform(1, 10))
            if RNG > 0.875:
                new_order["side"] = "ask"
            return [new_order, 1]
        else:
            new_order = dict(order_2)
            new_order["price"] = int(random.uniform(1, 10))
            if RNG > 0.625:
                new_order["side"] = "bid"
            return [new_order, 2]


def main():
    count = 0
    matchingEngine = MatchingEngine()
    client_1 = TradingClient(id="A", balance=100, asset_balance=50)
    client_2 = TradingClient(id="B", balance=100, asset_balance=50)
    matchingEngine.add_client(client_1)
    matchingEngine.add_client(client_2)
    order_1 = {
        "action": "add",
        "side": "bid",
        "price": 5,
        "quantity": 1,
        "client_id": "A",
    }
    order_2 = {
        "action": "add",
        "side": "ask",
        "price": 5,
        "quantity": 1,
        "client_id": "B",
    }

    tracemalloc.start()

    while True:
        print(f"\n  Step: {count}")
        matchingEngine.display_clients()

        if RNG_order := get_RNG_order([order_1, order_2]):
            if RNG_order[1] == 1:
                matchingEngine.submit_order(RNG_order[0], client_1)
            else:
                matchingEngine.submit_order(RNG_order[0], client_2)

        matchingEngine.orderBook.display(5)
        matchingEngine.execute()

        if count % 10000 == 0 and count > 0:
            snapshot = tracemalloc.take_snapshot()
            display_top(snapshot)
            time.sleep(5)

        count += 1


if __name__ == "__main__":
    main()
