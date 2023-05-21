import asyncio
import random
import sys

import aiohttp

MEAN = 100


async def main(num_requests):
    async with aiohttp.ClientSession() as session:
        tasks = [
            asyncio.ensure_future(make_request(session, i)) for i in range(num_requests)
        ]
        await asyncio.gather(*tasks)


async def make_request(session, i):
    url = "http://localhost:3000/order"
    if i % 2 == 0:
        data = {
            "side": "Sell",
            "quantity": 10,
            "price": random.randint(MEAN - 10, MEAN + 20),
        }
    else:
        data = {
            "side": "Buy",
            "quantity": 10,
            "price": random.randint(MEAN - 20, MEAN + 10),
        }
    async with session.post(url, json=data) as response:
        print(await response.text())


if __name__ == "__main__":
    num_requests = int(sys.argv[1]) if len(sys.argv) > 1 else 50
    asyncio.run(main(num_requests))
