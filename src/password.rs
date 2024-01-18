use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "ssr")]
#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Password {
    pub id: Uuid,
    pub key: String,
    pub password: String,
}

#[cfg(not(feature = "ssr"))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Password {
    pub id: Uuid,
    pub key: String,
    pub password: String,
}
