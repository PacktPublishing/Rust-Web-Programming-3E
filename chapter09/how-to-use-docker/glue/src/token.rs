/// The houses data from the token in the header.
/// 
/// # Notes
/// At this point in the book authentication is not implemented, so the token is just a string.
/// But authentication logic will be added later on in the book.
/// 
/// # Fields
/// - `message` - The token from the header
pub struct HeaderToken {
    pub message: String,
}

// Actix Web implementation of FromRequest for HeaderToken
#[cfg(feature = "actix")]
mod actix_impl {
    use super::HeaderToken;
    pub use actix_web::{
        dev::Payload,
        FromRequest as ActixFromRequest,
        HttpRequest,
    };
    use futures::future::{Ready, ok, err};
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};

    impl ActixFromRequest for HeaderToken {
        type Error = NanoServiceError;
        type Future = Ready<Result<HeaderToken, NanoServiceError>>;

        fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
            let raw_data = match req.headers().get("token") {
                Some(data) => data,
                None => {
                    return err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not in header under key 'token'".to_string()
                    })
                }
            };
            let message = match raw_data.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not a valid string".to_string()
                    })
                }
            };
            ok(HeaderToken { message })
        }
    }
}

// Rocket implementation of FromRequest for HeaderToken
#[cfg(feature = "rocket")]
mod rocket_impl {
    use super::HeaderToken;
    pub use rocket::request::{self, FromRequest as RocketFromRequest, Request};
    use rocket::outcome::Outcome;
    use rocket::http::Status;
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};

    #[rocket::async_trait]
    impl<'r> RocketFromRequest<'r> for HeaderToken {
        type Error = NanoServiceError;

        async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
            match req.headers().get_one("token") {
                Some(token) => Outcome::Success(HeaderToken {
                    message: token.to_string(),
                }),
                None => Outcome::Error((
                    Status::Unauthorized,
                    NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "token not in header under key 'token'".to_string(),
                    },
                )),
            }
        }
    }
}

// Axum implementation of FromRequestParts for HeaderToken
#[cfg(feature = "axum")]
mod axum_impl {
    use super::HeaderToken;
    pub use axum::{
        async_trait,
        extract::FromRequestParts as AxumFromRequestParts,
    };
    use axum::http::request::Parts;
    use crate::errors::{NanoServiceError, NanoServiceErrorStatus};

    #[async_trait]
    impl<S> AxumFromRequestParts<S> for HeaderToken
    where
        S: Send + Sync,
    {
        type Rejection = NanoServiceError;

        async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
            // Extract the header from parts
            let raw_data = match parts.headers.get("token") {
                Some(data) => data,
                None => {
                    return Err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "Token not found in header under key 'token'".to_string(),
                    });
                }
            };

            // Convert the header value to a string
            let message = match raw_data.to_str() {
                Ok(token) => token.to_string(),
                Err(_) => {
                    return Err(NanoServiceError {
                        status: NanoServiceErrorStatus::Unauthorized,
                        message: "Token is not a valid string".to_string(),
                    });
                }
            };

            // Return the extracted token
            Ok(HeaderToken { message })
        }
    }
}


// Re-export the specific FromRequest implementations depending on the activated feature
#[cfg(feature = "actix")]
pub use actix_impl::ActixFromRequest;

#[cfg(feature = "rocket")]
pub use rocket_impl::RocketFromRequest;

#[cfg(feature = "axum")]
pub use axum_impl::AxumFromRequestParts;
