use std::str;






use reqwest::{Client, Error, Request};









use crate::authorization_code_callback::await_authorization_code_callback;
use crate::constants::{AUTHORIZATION_URI, CLIENT_ID, TOKEN_URI};
use crate::models::{
    AccessTokenRequestParams, AccessTokenResponse, AuthorizationCodeQueryParams,
    RefreshTokenRequestParams,
};

pub fn build_authorization_code_request(
    client: &Client,
    code_challenge: String,
) -> reqwest::Result<Request> {
    let authorization_code_params = AuthorizationCodeQueryParams {
        client_id: CLIENT_ID.into(),
        response_type: "code".into(),
        redirect_uri: "http://localhost:8080/callback".into(),
        code_challenge_method: "S256".into(),
        code_challenge,
        scope: Some("user-read-playback-state".to_string()),
    };

    client
        .get(AUTHORIZATION_URI)
        .query(&authorization_code_params)
        .build()
}

pub async fn exchange_authorization_code_code_for_access_token(
    client: &Client,
    client_id: &str,
    authorization_code: String,
    code_verifier: String,
) -> Result<AccessTokenResponse, Error> {
    let access_token_request = AccessTokenRequestParams {
        client_id: client_id.into(),
        grant_type: "authorization_code".into(),
        redirect_uri: "http://localhost:8080/callback".into(),
        code: authorization_code,
        code_verifier,
    };

    let response = client
        .post(TOKEN_URI)
        .form(&access_token_request)
        .send()
        .await?;

    response.json::<AccessTokenResponse>().await
}

pub async fn acquire_access_token(client: &Client) -> AccessTokenResponse {
    let code_verifier: Vec<u8> = pkce::code_verifier(128);
    let code_challenge: String = pkce::code_challenge(&code_verifier);

    let authorization_link = build_authorization_code_request(client, code_challenge).unwrap();
    println!("OPEN THIS LINK: {:?}", authorization_link.url().to_string());

    let authorization_code = await_authorization_code_callback().await.unwrap();

    let access_token = exchange_authorization_code_code_for_access_token(
        client,
        CLIENT_ID,
        authorization_code,
        std::str::from_utf8(&code_verifier).unwrap().to_string(),
    )
    .await
    .unwrap();

    access_token
}

pub async fn refresh_token_exchange(client: &Client, refresh_token: &str) -> AccessTokenResponse {
    let a = client
        .post(TOKEN_URI)
        .form(&RefreshTokenRequestParams::new(refresh_token.into()))
        .build()
        .unwrap();

    let response = client.execute(a).await.unwrap();

    response.json::<AccessTokenResponse>().await.unwrap()
}
