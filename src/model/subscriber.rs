use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification;

use rocket::serde::json::serde_json; 
use log::{warn};  

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    #[tokio::main]
    pub async fn update(&self, payload: Notification) {
        REQWEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&payload).unwrap()) // Gunakan serde_json langsung
            .send()
            .await
            .ok();
        
        warn!(
            "Sent {} notification of: [{}] {}, to: {}",
            payload.status, payload.product_type, payload.product_title, self.url
        );
    }
}
