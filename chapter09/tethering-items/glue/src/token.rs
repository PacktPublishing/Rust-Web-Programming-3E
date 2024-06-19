#[cfg(feature = "actix")]
use actix_web::{
    dev::Payload,
    FromRequest,
    HttpRequest,
};
#[cfg(feature = "actix")]
use futures::future::{Ready, ok, err};


use crate::errors::{
    NanoServiceError,
    NanoServiceErrorStatus
};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{
    decode, 
    encode, 
    Algorithm, 
    DecodingKey, 
    EncodingKey, 
    Header, 
    Validation
};


#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderToken {
    pub unique_id: String
}


impl HeaderToken {

    pub fn get_key() -> Result<String, NanoServiceError> {
        std::env::var("JWT_SECRET").map_err(|e| {
            NanoServiceError::new(
                e.to_string(), 
                NanoServiceErrorStatus::Unauthorized
            )
        })
    }

    /// Encodes the struct into a token.
    ///
    /// # Returns
    /// encoded token with fields of the current struct
    pub fn encode(self) -> Result<String, NanoServiceError> {
        let key_str = Self::get_key()?;
        let key = EncodingKey::from_secret(key_str.as_ref());
        return match encode(&Header::default(), &self, &key) {
            Ok(token) => Ok(token),
            Err(error) => Err(
                NanoServiceError::new(
                    error.to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        };
    }

    /// Decodes the token into a struct.
    ///
    /// # Arguments
    /// * `token` - The token to be decoded.
    ///
    /// # Returns
    /// decoded token with fields of the current struct
    pub fn decode(token: &str) -> Result<Self, NanoServiceError> {
        let key_str = Self::get_key()?;
        let key = DecodingKey::from_secret(key_str.as_ref());
        let mut validation = Validation::new(Algorithm::HS256);
        validation.required_spec_claims.remove("exp");

        match decode::<Self>(token, &key, &validation) {
            Ok(token_data) => return Ok(token_data.claims),
            Err(error) => return Err(
                NanoServiceError::new(
                    error.to_string(),
                    NanoServiceErrorStatus::Unauthorized
                )
            )
        };
    }
}


#[cfg(feature = "actix")]
impl FromRequest for HeaderToken {
    type Error = NanoServiceError;
    type Future = Ready<Result<HeaderToken, NanoServiceError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // extract the token from the header
        let raw_data = match req.headers().get("token") {
            Some(data) => data,
            None => {
                return err(NanoServiceError {
                    status: NanoServiceErrorStatus::Unauthorized,
                    message: "token not in header under key 'token'".to_string()
                })
            }
        };
        // convert the token to a string
        let message = match raw_data.to_str() {
            Ok(token) => token.to_string(),
            Err(_) => {
                return err(NanoServiceError {
                    status: NanoServiceErrorStatus::Unauthorized,
                    message: "token not a valid string".to_string()
                })
            }
        };
        // decode the token
        let token = match HeaderToken::decode(&message) {
            Ok(token) => token,
            Err(e) => {
                return err(e)
            }
        };
        return ok(token)
    }
}