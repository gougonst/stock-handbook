use crate::app_state::AppState;
use crate::constants;
use crate::database::user_repository::{UserRepository, UserRepositoryError};
use crate::models::user::User;
use actix_web::{web, HttpResponse, Responder};
use log::{debug, info};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

pub async fn login(info: web::Json<LoginInfo>, data: web::Data<AppState>) -> impl Responder {
    info!("Handle 'login' request");
    let user = User::new(info.username.clone(), info.password.clone());
    debug!(
        "Username: {}, Password: {}",
        user.username(),
        user.password()
    );

    match data.user_repo.check_user(&user).await {
        Ok(true) => HttpResponse::Ok().body(constants::HTTP_OK),
        Ok(false) => HttpResponse::BadRequest().body(constants::HTTP_USER_PASSWORD_INCORRECT),
        Err(UserRepositoryError::UserNotFound) => {
            HttpResponse::NotFound().body(constants::HTTP_USER_NOT_FOUND)
        }
        Err(e) => HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR),
    }
}

pub async fn logon(info: web::Json<LoginInfo>, data: web::Data<AppState>) -> impl Responder {
    info!("Handle 'logon' request");
    let user = User::new(info.username.clone(), info.password.clone());
    debug!(
        "Username: {}, Password: {}",
        user.username(),
        user.password()
    );

    match data.user_repo.create_user(&user).await {
        Ok(result) => {
            if result {
                return HttpResponse::Ok().body(constants::HTTP_OK);
            } else {
                return HttpResponse::BadRequest().body(constants::HTTP_USER_ALREADY_EXIST);
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR),
    }
}
