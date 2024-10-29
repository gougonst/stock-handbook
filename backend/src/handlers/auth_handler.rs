use crate::app_state::AppState;
use crate::constants;
use crate::database::repository_error::RepositoryError;
use crate::models::user_model::UserModel;
use actix_web::{web, HttpResponse, Responder};
use log::{debug, error, info};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInfo {
    username: String,
    password: String,
}

pub async fn login(info: web::Json<LoginInfo>, data: web::Data<AppState>) -> impl Responder {
    info!("Handle 'login' request");
    let user = UserModel::new(info.username.clone(), info.password.clone());
    debug!(
        "Username: {}, Password: {}",
        user.username(),
        user.password()
    );

    match data.user_repo.check_user(&user).await {
        Ok(true) => HttpResponse::Ok().body(constants::HTTP_OK),
        Ok(false) => HttpResponse::BadRequest().body(constants::HTTP_USER_PASSWORD_INCORRECT),
        Err(RepositoryError::UserNotFound) => {
            debug!("User is not exist");
            HttpResponse::NotFound().body(constants::HTTP_USER_NOT_FOUND)
        }
        Err(e) => {
            error!("Login error: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}

pub async fn logon(info: web::Json<LoginInfo>, data: web::Data<AppState>) -> impl Responder {
    info!(
        "Handle 'logon' request with Username: {}, Password: {}",
        info.username, info.password
    );
    let user = UserModel::new(info.username.clone(), info.password.clone());

    match data.user_repo.create_user(&user).await {
        Ok(result) => {
            if result {
                return HttpResponse::Ok().body(constants::HTTP_OK);
            } else {
                return HttpResponse::BadRequest().body(constants::HTTP_USER_ALREADY_EXIST);
            }
        }
        Err(e) => {
            error!("Logon error: {}", e);
            HttpResponse::InternalServerError().body(constants::HTTP_INTERNAL_ERROR)
        }
    }
}
