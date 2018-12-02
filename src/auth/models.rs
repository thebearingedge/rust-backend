use crate::schema::users;
use chrono::{offset::Utc, DateTime};
use uuid::Uuid;

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct CreatedUser {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub email: String,
}

#[derive(Queryable)]
pub struct ActiveUser {
    pub user_id: Uuid,
    pub email: String,
    pub password: Option<String>,
}
