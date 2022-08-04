use base64::encode;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;

use serde::Deserialize;

#[derive(Deserialize)]
struct ResponseType {
    access_token: String
}

#[derive(Debug)]
pub enum RetrieveTokenError {
    RequestFail(String),
    MalformedResponse(String),
}

pub fn retrieve_token(client_id: &str, client_secret: &str, authroize_code: &str, authorized_port: u16) -> Result<String, RetrieveTokenError> {
    let queries = [
        ("code", authroize_code),
        ("grant_type", "authorization_code"),
        ("scope", "tasks:read tasks:write"),
        ("redirect_uri", &format!("http://localhost:{}", authorized_port))
    ];

    Ok(
        Client::new()
            .post("https://ticktick.com/oauth/token")
            .basic_auth(client_id, Some(client_secret))
            .query(&queries)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .send()
            .map_err(|e| RetrieveTokenError::RequestFail(format!("{:#?}", e)))?
            .json::<ResponseType>()
            .map_err(|e| RetrieveTokenError::MalformedResponse(format!("{:#?}", e)))?
            .access_token
    )
}
