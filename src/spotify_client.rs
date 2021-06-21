use reqwest::{Client};

use tokio::sync::{watch};


use tokio::time;


use crate::models::{AccessTokenResponse};
use crate::oauth::refresh_token_exchange;

pub struct TokenManager {
    client: Client,
    token_sender: watch::Sender<String>,
    token_receiver: watch::Receiver<String>,
    state: Option<AccessTokenResponse>,
}

impl TokenManager {
    pub async fn new(initial_access_token: AccessTokenResponse, client: Client) -> TokenManager {
        let (tx, rx) = watch::channel(initial_access_token.access_token.clone());
        TokenManager {
            client,
            token_sender: tx,
            token_receiver: rx,
            state: Some(initial_access_token),
        }
    }

    pub fn channel(&self) -> watch::Receiver<String> {
        self.token_receiver.clone()
    }

    pub async fn refresh(mut self) {
        let token_expires_in = self.state.clone().unwrap().expires_in as u64;
        let mut interval = time::interval(time::Duration::from_secs(token_expires_in - 10));
        loop {
            interval.tick().await;
            let refreshed_token =
                refresh_token_exchange(&self.client, &self.state.unwrap().refresh_token).await;
            println!("refreshed: {:?}", refreshed_token);
            self.token_sender.send(refreshed_token.access_token.clone()).unwrap();
            self.state = Some(refreshed_token);
        }
    }
}
