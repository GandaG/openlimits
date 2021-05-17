use serde::Deserialize;
use serde::Serialize;
use super::Paginator;

/// This struct represents a request of fill orders
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GetFillsReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paginator: Option<Paginator>,
}