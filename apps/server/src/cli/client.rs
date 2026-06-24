use engine::Side;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const BASE: &str = "http://localhost:8080/api/v1";

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Deserialize)]
pub struct SigninData {
    pub token: String,
}

#[derive(Serialize)]
struct SigninBody {
    name: String,
    email: String,
    image: Option<String>,
}

#[derive(Serialize)]
struct LimitOrderBody {
    market_id: Uuid,
    side: Side,
    tick: u64,
    quantity: u64,
}

#[derive(Serialize)]
struct MarketOrderBody {
    market_id: Uuid,
    side: Side,
    quantity: u64,
}

pub struct ApiClient {
    http: Client,
    pub token: String,
    pub market_id: Uuid,
}

impl ApiClient {
    pub async fn signin(email: String, name: String, market_id: Uuid) -> anyhow::Result<Self> {
        let http = Client::new();
        let res = http
            .post(format!("{BASE}/signin"))
            .json(&SigninBody { name, email, image: None })
            .send()
            .await?
            .json::<ApiResponse<SigninData>>()
            .await?;

        let token = res
            .data
            .ok_or_else(|| anyhow::anyhow!("signin failed: {:?}", res.message))?
            .token;

        Ok(Self { http, token, market_id })
    }

    pub async fn place_limit(&self, side: Side, tick: u64, quantity: u64) -> anyhow::Result<String> {
        let res = self
            .http
            .post(format!("{BASE}/order/limit/place"))
            .bearer_auth(&self.token)
            .json(&LimitOrderBody { market_id: self.market_id, side, tick, quantity })
            .send()
            .await?
            .json::<ApiResponse<serde_json::Value>>()
            .await?;

        Ok(res.message.unwrap_or_else(|| "ok".into()))
    }

    pub async fn place_market(&self, side: Side, quantity: u64) -> anyhow::Result<String> {
        let res = self
            .http
            .post(format!("{BASE}/order/market/place"))
            .bearer_auth(&self.token)
            .json(&MarketOrderBody { market_id: self.market_id, side, quantity })
            .send()
            .await?
            .json::<ApiResponse<serde_json::Value>>()
            .await?;

        Ok(res.message.unwrap_or_else(|| "ok".into()))
    }
}
