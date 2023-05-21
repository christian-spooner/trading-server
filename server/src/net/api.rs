use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{collections::HashMap, time::Duration};
use std::{sync::Arc, time::SystemTime};
use tokio::sync::Mutex;
use tokio::time::sleep;
use tower_http::cors::CorsLayer;

use crate::core::{engine::Engine, order_book::Side, vector::VectorOrderBook};

pub async fn start_api(n: Arc<Mutex<Engine<VectorOrderBook>>>) {
    let app = Router::new()
        .route("/test", get(get_test))
        .route("/book", get(get_book))
        .route("/price", get(get_price))
        .route("/trades", get(get_trades))
        .route("/order", post(post_order))
        .route("/report/:id", get(get_report))
        .layer(CorsLayer::permissive())
        .with_state(n);
    println!("API listening on port 3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_test() -> &'static str {
    "Hello, World!"
}

async fn get_book(State(n): State<Arc<Mutex<Engine<VectorOrderBook>>>>) -> Json<Value> {
    let (bids, asks) = n.lock().await.get_book();
    let book = json!({
        "bids": bids,
        "asks": asks
    });
    Json(book)
}

async fn get_price(State(n): State<Arc<Mutex<Engine<VectorOrderBook>>>>) -> Json<Value> {
    let mut open = 0.0;
    let mut close = 0.0;
    let mut high = 0.0;
    let mut low = f64::MAX;
    let mut timestamp = 0;

    for i in 0..10 {
        {
            let n_lock = n.lock().await;
            let price = n_lock.get_market_price();
            match price {
                Ok(price) => {
                    if i == 0 {
                        open = price;
                        high = price;
                        low = price;
                    } else {
                        high = f64::max(price, high);
                        low = f64::min(price, low);
                    }
                    if i == 9 {
                        close = price;
                        let time = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_millis();
                        timestamp = time as u64;
                    }
                }
                Err(error) => return Json(json!({ "status": 400, "error": error })),
            }
        }

        sleep(Duration::from_millis(100)).await;
    }

    Json(json!({ "open": open, "close": close, "high": high, "low": low, "timestamp": timestamp }))
}

async fn get_trades(State(n): State<Arc<Mutex<Engine<VectorOrderBook>>>>) -> Json<Value> {
    let trades = n.lock().await.get_trade_history();
    Json(json!(trades))
}

#[derive(Deserialize)]
struct OrderRequest {
    pub side: Side,
    pub quantity: u64,
    pub price: f64,
}

async fn post_order(
    State(n): State<Arc<Mutex<Engine<VectorOrderBook>>>>,
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
) -> Json<Value> {
    let order_request: OrderRequest = serde_json::from_value(data).unwrap();
    let mut n_lock = n.lock().await;
    let result = n_lock.add_order(
        order_request.side.to_string(),
        order_request.quantity,
        order_request.price,
    );
    Json(json!({"id": result.unwrap()}))
}

async fn get_report(
    State(n): State<Arc<Mutex<Engine<VectorOrderBook>>>>,
    Path(params): Path<HashMap<String, String>>,
) -> Json<Value> {
    let id_param = params.get("id");
    let id_opt = id_param.map(|s| s.parse::<u64>().unwrap());
    let id = match id_opt {
        Some(i) => i,
        None => 0,
    };
    let n_lock = n.lock().await;
    let status_result = n_lock.get_order_status(id);
    match status_result {
        Ok(status) => Json(json!({ "status": status })),
        Err(e) => Json(json!({ "error": e })),
    }
}
