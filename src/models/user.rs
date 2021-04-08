use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Queryable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub name: String,
    pub email: String,
    pub gender: String,
    pub birth_date: String,
    pub state: String,
    pub password: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub state: String,
    pub birth_date: String,
    pub gender: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub state: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}