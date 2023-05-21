import random
import time

import requests

MEAN = 100
DRIFT = 0

url = "http://localhost:3000/order"

while True:
    try:
        DRIFT += random.choice([-1, 1])
        DRIFT += random.choice([-0.1, 0.1])
        DRIFT = max(-99, DRIFT)
        i = random.randint(1, 10)
        if i % 2 == 0:
            data = {
                "side": "Sell",
                "quantity": random.randint(10, 50),
                "price": MEAN + DRIFT,
            }
        else:
            data = {
                "side": "Buy",
                "quantity": random.randint(10, 50),
                "price": MEAN + DRIFT,
            }
        response = requests.post(url, json=data)
        print(response.text)
        time.sleep(1)
    except KeyboardInterrupt:
        break
