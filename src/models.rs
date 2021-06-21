use serde::Deserialize;
use serde::Serialize;


use crate::constants::CLIENT_ID;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RefreshTokenRequestParams {
    pub client_id: String,
    pub grant_type: String,
    pub refresh_token: String,
}
impl RefreshTokenRequestParams {
    pub fn new(refresh_token: String) -> RefreshTokenRequestParams {
        RefreshTokenRequestParams {
            client_id: CLIENT_ID.into(),
            grant_type: "refresh_token".into(),
            refresh_token,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessTokenRequestParams {
    pub client_id: String,
    pub grant_type: String,
    pub code: String,
    pub redirect_uri: String,
    pub code_verifier: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessTokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: f64,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct AuthorizationCodeQueryParams {
    pub client_id: String,
    pub response_type: String,
    pub redirect_uri: String,
    pub code_challenge_method: String,
    pub code_challenge: String,
    pub scope: Option<String>,
}
