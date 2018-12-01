use crate::schema::users;
use chrono::{offset::Utc, DateTime};
use uuid::Uuid;

#[derive(Serialize, Queryable)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: Option<String>,
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
