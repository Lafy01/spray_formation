use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::{handlers, middleware};

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service( web::scope("/employe/secure")
            .wrap(from_fn(middleware::auth_midllewares::check_auth_middleware))
            .service(handlers::employe_handlers::me)
            .service(handlers::employe_handlers::delete_user)
            .service(handlers::employe_handlers::update_employe)
            .service(handlers::employe_handlers::all_employe)
            .service(handlers::employe_handlers::all_user)
            .service(handlers::employe_handlers::all_my_employe)
            .service(handlers::employe_handlers::all_my_employe_planning)
            .service(handlers::employe_handlers::all_employe_planning)
        )
        .service(web::scope("/employe")
        );
}