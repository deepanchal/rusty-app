use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct UserAccount {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invalidated_at: Option<NaiveDateTime>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub email: String,
    pub password: String,
}
