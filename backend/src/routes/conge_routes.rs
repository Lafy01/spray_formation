use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::{handlers, middleware};

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service( web::scope("/conge/secure")
            .wrap(from_fn(middleware::auth_midllewares::check_auth_middleware))
            .service(handlers::conge_handlers::add_my_conge)
            .service(handlers::conge_handlers::add_conge)
            .service(handlers::conge_handlers::all_my_conge)
            .service(handlers::conge_handlers::all_my_employe_conge)
            .service(handlers::conge_handlers::approve_conge)
            .service(handlers::conge_handlers::decline_conge)
        )
        .service(web::scope("/conge")
            .service(handlers::conge_handlers::all_conge)
        );
}