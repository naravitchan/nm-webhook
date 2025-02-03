use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/webhook", post(line_webhook))
        .route(
            "/webhook",
            get(|| {
                println!("hello webhook");
                async { "Hello, Webhook!" }
            }),
        );

    // run our app with hyper, listening globally on port 3031
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3031").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub async fn line_webhook(Json(payload): Json<Value>) -> impl IntoResponse {
    println!("payload: {:#?}", payload);
    let event_type: Result<WebhookPayload, serde_json::Error> =
        serde_json::from_value(payload.clone());
    println!("event_type: {:#?}", event_type);
    (
        StatusCode::OK,
        Json(json!({
            "status": 200,
            "message": "e",
        }))
        .into_response(),
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub destination: String,
    pub events: Vec<Event>,
}

// cast only join event format
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "replyToken")]
    pub reply_token: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub mode: String,
    pub timestamp: u64,
    pub source: Source,
    //     pub webhookEventId: String,
    //     pub deliveryContext: DeliveryContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>, // ทำให้เป็น Option เพราะอาจไม่มีในบางเคส
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct DeliveryContext {
//     pub isRedelivery: bool,
// }
// {
//     "destination": "xxxxxxxxxx",
//     "events": [
//       {
//         "replyToken": "nHuyWiB7yP5Zw52FIkcQobQuGDXCTA",
//         "type": "join",
//         "mode": "active",
//         "timestamp": 1462629479859,
//         "source": {
//           "type": "group",
//           "groupId": "C4af4980629..."
//         },
//         "webhookEventId": "01FZ74A0TDDPYRVKNK77XKC3ZR",
//         "deliveryContext": {
//           "isRedelivery": false
//         }
//       }
//     ]
//   }
