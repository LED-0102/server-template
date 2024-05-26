use actix_web::{HttpResponse, Responder, web};
use bcrypt::{DEFAULT_COST, hash};
use diesel::{debug_query, RunQueryDsl};
use diesel::pg::Pg;

use crate::database::DB;
use crate::models::auth::new_user::NewUser;
use crate::Res;

impl NewUser {
    pub fn new(email: &str, password: &str, username: &str) -> NewUser {
        let hashed_password: String = hash(password, DEFAULT_COST).unwrap();
        return NewUser {
            password: hashed_password,
            email: email.parse().unwrap(),
            username: username.parse().unwrap(),
        };
    }
}
async fn insert(new_user: NewUser, db: &mut DB) -> Res<()> {
    use crate::schema::users::dsl::*;

    let _ = diesel::insert_into(users)
        .values(&new_user)
        .execute(&mut db.connection)?;

    println!(
        "{:?}",
        debug_query::<Pg, _>(&diesel::insert_into(users).values(&new_user))
    );
    Ok(())
}
pub async fn register(new_user: web::Json<NewUser>, mut db: DB) -> impl Responder {
    let new_user = NewUser::new(&new_user.email, &new_user.password, &new_user.username);
    match insert(new_user, &mut db).await {
        Ok(_) => {
            println!("Created");
            HttpResponse::Created()
        }
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::Conflict()
        }
    }
}
