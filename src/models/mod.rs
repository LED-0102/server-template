use chrono::NaiveDateTime;

use crate::schema::to_do;

pub mod auth;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = to_do)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct ToDo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
}
