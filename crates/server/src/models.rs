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

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub invalidated_at: Option<NaiveDateTime>,
    pub created_by: Option<i32>,
    pub updated_by: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub user_id: i32,
}
