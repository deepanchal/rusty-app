use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Person {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invalidated_at: Option<NaiveDateTime>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub user_id: i32,
}
