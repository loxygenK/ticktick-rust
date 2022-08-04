use crate::auth::{authorize::AuthorizeError, authenticate::RetrieveTokenError};

use self::{authorize::authorize_interactively, authenticate::retrieve_token};

pub mod authenticate;
pub mod authorize;

#[derive(Debug)]
pub enum AuthError {
    AuthorizationError(AuthorizeError),
    AuthenticationError(RetrieveTokenError)
}

pub fn perform_auth(client_id: &str, client_secret: &str, port: u16) -> Result<String, AuthError> {
    let code = authorize_interactively(client_id, port)
        .map_err(AuthError::AuthorizationError)?;

    let token = retrieve_token(client_id, client_secret, &code, port)
        .map_err(AuthError::AuthenticationError)?;

    Ok(token)
}
