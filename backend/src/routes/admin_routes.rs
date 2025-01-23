use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::{handlers, middleware};

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/admin/secure")
            .wrap(from_fn(middleware::auth_midllewares::check_auth_middleware))
            
            // departement
            .service(handlers::admin_handlers::all_departement)
            .service(handlers::admin_handlers::update_departement)
            .service(handlers::admin_handlers::delete_departement)

        )
        .service(
            web::scope("/admin")
            .service(handlers::admin_handlers::add_departement)
            // .service(handlers::admin_handlers::update_solde)
        );
}