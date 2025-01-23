use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    Error, HttpMessage,
};
use actix_web_lab::middleware::Next;

use crate::utils::{
    api_response::ApiResponse, 
    jwt::decode_token
};

pub async fn check_auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth = req.headers().get(AUTHORIZATION);

    // if auth.is_none() {
    //     return Err(Error::from(ApiResponse::<()>::new(
    //         401, 
    //         false,
    //         "Unauthorised".to_string(), 
    //         None
    //     )));
    // }

    // let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();

    let token = match auth {
        Some(header_value) => {
            match header_value.to_str() {
                Ok(header_str) => {
                    if header_str.starts_with("Bearer ") {
                        header_str.trim_start_matches("Bearer ").to_owned()
                    } else {
                        return Err(Error::from(ApiResponse::<()>::new(
                            400,
                            false,
                            "Invalid Authorization header format".to_string(),
                            None
                        )));
                    }
                },
                Err(_) => {
                    return Err(Error::from(ApiResponse::<()>::new(400, false, "Invalid Authorization header format".to_string(), None)));
                }
            }
        }
        None => {
            return Err(Error::from(ApiResponse::<()>::new(
                401, 
                false, 
                "Unauthorised: Missing Authorization header".to_string(), 
                None
            )));
        }
    };

    let claim = match decode_token(token) {
        Ok(claim) => claim,
        Err(err) => {
            return Err(Error::from(ApiResponse::<()>::new(
                401, 
                false, 
                format!("Unauthorised: invalid token, {}", err), 
                None
            )));
        }
    };

    req.extensions_mut().insert(claim.claims);

    next.call(req)
        .await
        .map_err(|err| Error::from(ApiResponse::<()>::new(
            500,
            false,
            err.to_string(),
            None, 
        )))
}
