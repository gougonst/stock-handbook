use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::app_state::AppState;
use crate::constants;
use crate::database::repository::{UserRepository, UserRepositoryError};
use crate::models::user;

#[derive(Deserialize)]
pub struct LoginInfo {
    username: String, 
    password: String, 
}

pub async fn login(
    info: web::Json<LoginInfo>, 
    data: web::Data<AppState>
) -> impl Responder {
    let user = user::UserModel::new(info.username.clone(), info.password.clone());
    match data.user_repo.check_user(&user).await {
        Ok(true) => {
            HttpResponse::Ok().body(constants::HTTP_OK)
        }, 
        Ok(false) => {
            HttpResponse::BadRequest().body(constants::HTTP_USER_PASSWORD_INCORRECT)
        }
        Err(UserRepositoryError::UserNotFound) => {
            HttpResponse::NotFound().body(constants::HTTP_USER_NOT_FOUND)
        }, 
        Err(e) => {
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
    
}

pub async fn logon(
    info: web::Json<LoginInfo>, 
    data: web::Data<AppState>
) -> impl Responder {
    let user = user::UserModel::new(info.username.clone(), info.password.clone());
    match data.user_repo.create_user(&user).await {
        Ok(result) => {
            if result {
                return HttpResponse::Ok().body(constants::HTTP_OK);
            } else {
                return HttpResponse::BadRequest().body(constants::HTTP_USER_ALREADY_EXIST);
            }
        }, 
        Err(e) => {
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}
