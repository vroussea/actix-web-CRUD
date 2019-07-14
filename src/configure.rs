use crate::handlers::*;
use actix_web::{error, web, HttpResponse};

pub fn add(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/add")
            .data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _| {
                        error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                            .into()
                    }),
            )
            .route(web::post().to_async(create_user)),
    );
}

pub fn get_one(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _| {
                        error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                            .into()
                    }),
            )
            .route(web::get().to_async(get_user)),
    );
}

pub fn get_all(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/all").route(web::get().to_async(get_users)));
}

pub fn update(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/update")
            .data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _| {
                        error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                            .into()
                    }),
            )
            .route(web::put().to_async(update_user)),
    );
}

pub fn delete(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/delete")
            .data(
                web::JsonConfig::default()
                    .limit(4096)
                    .error_handler(|err, _| {
                        error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                            .into()
                    }),
            )
            .route(web::delete().to_async(delete_user)),
    );
}
