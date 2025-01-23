use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::{handlers, middleware};



pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/secure")
                .wrap(from_fn(middleware::auth_midllewares::check_auth_middleware))
                
        )
        .service(
            web::scope("/auth")
            .service(handlers::auth_handlers::register)
            .service(handlers::auth_handlers::login)
        );
}