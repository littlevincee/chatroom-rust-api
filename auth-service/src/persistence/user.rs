use chrono::{DateTime, Utc, NaiveDateTime};
use crate::persistence::schema::users;

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct UserEntity {
  pub id: i32,
  pub username: String,
  pub first_name: String,
  pub last_name: String,
  pub is_active: bool,
  pub is_super: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUserEntity {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub is_super: bool,
//    pub created_at: DateTime<Utc>,
    //pub updated_at: DateTime<Utc>
}