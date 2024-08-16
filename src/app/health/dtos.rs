use utoipa::ToSchema;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
}
