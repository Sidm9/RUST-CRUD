use super::models::{NewUser, User};
use super::schema::users::dsl::*;
use super::Pool;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use actix_web::{web, Error, HttpResponse};
use diesel::update;
use diesel::dsl::{delete, insert_into};
use serde::{Deserialize, Serialize};
use std::vec::Vec;
use crate::diesel::ExpressionMethods;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
}

pub async fn get_users(_db: web::Data<Pool>) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || get_all_users(_db))
      .await
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
  let conn = pool.get().unwrap();
  let items = users.load::<User>(&conn)?;
  Ok(items)
}
// Helper functions
fn db_get_user_by_id(pool: web::Data<Pool>, user_id: i32) -> Result<User, diesel::result::Error> {
  let conn = pool.get().unwrap();
  users.find(user_id).get_result::<User>(&conn)
}

pub async fn get_user_by_id(
  db: web::Data<Pool>,
  user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || db_get_user_by_id(db, user_id.into_inner()))
      .await
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}
// Helper functions
fn add_single_user(db: web::Data<Pool> , item: web::Json<InputUser>) -> Result<User,diesel::result::Error> {
  let conn = db.get().unwrap();
  let new_user = NewUser {
    first_name: &item.first_name,
    last_name: &item.last_name,
    email: &item.email,
    created_at: chrono::Utc::now().naive_utc(),
  };
  insert_into(users).values(&new_user).get_result(&conn)
}

pub async fn add_user(
  db: web::Data<Pool>,
  item: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || add_single_user(db, item))
      .await
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}

// Helper functions
fn update_single_user(
  db: web::Data<Pool>, 
  user_id: i32,
  item: web::Json<InputUser>
) -> Result<usize, diesel::result::Error> {
  let conn = db.get().unwrap();
  let count = update(users.find(user_id))
    .set((
      first_name.eq(item.first_name.clone()),
      last_name.eq(item.last_name.clone()),
      email.eq(item.email.clone()),
    ))
    .execute(&conn)?;
  Ok(count) 
}

pub async fn update_user(
  db: web::Data<Pool>,
  user_id: web::Path<i32>,
  item: web::Json<InputUser>
) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || update_single_user(db, user_id.into_inner(), item))
      .await
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}

// Helper functions
fn delete_single_user(db: web::Data<Pool>, user_id: i32) -> Result<usize, diesel::result::Error> {
  let conn = db.get().unwrap();
  let count = delete(users.find(user_id)).execute(&conn)?;
  Ok(count)
}

pub async fn delete_user(
  db: web::Data<Pool>,
  user_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
  Ok(
    web::block(move || delete_single_user(db, user_id.into_inner()))
      .await
      .map(|user| HttpResponse::Ok().json(user))
      .map_err(|_| HttpResponse::InternalServerError())?,
  )
}
