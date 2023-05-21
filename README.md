<p align="center">
  <img width="1638" alt="trading client screenshot" src="https://github.com/christian-spooner/trading-server/assets/93479191/ee095aed-547c-469e-a806-498b42f9d90f">
  <br>
  <em>Web-client dashboard: provides market data & ability to send limit orders to local trading server</em>
</p>

### Local Deployment
```
cd server
cargo run

cd client-web
yarn install
yarn dev
```

If you want to quickly send some orders to the server, run
```
cd client-scripts
pipenv shell && pipenv install
python init.py
```

There are also some simple bots in `client-scripts` to simulate market participants,
```
python gaussian_bot.py
python random_walk_bot.py
```
