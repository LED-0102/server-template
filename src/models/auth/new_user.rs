use serde::Deserialize;

use crate::schema::users;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}
