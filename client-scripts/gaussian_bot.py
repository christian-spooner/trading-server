import random
import time

import requests

MEAN = 100

url = "http://localhost:3000/order"

while True:
    try:
        i = random.randint(1, 10)
        if i % 2 == 0:
            data = {
                "side": "Sell",
                "quantity": random.randint(5, 15),
                "price": abs(random.normalvariate(MEAN, 10)),
            }
        else:
            data = {
                "side": "Buy",
                "quantity": random.randint(5, 15),
                "price": abs(random.normalvariate(MEAN, 10)),
            }
        response = requests.post(url, json=data)
        print(response.text)
        time.sleep(1)
    except KeyboardInterrupt:
        break
