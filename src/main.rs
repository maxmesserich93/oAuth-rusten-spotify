

use crate::oauth::{acquire_access_token};
use crate::playback_mananger::PlaybackManager;
use crate::server::websocket;
use crate::spotify_client::TokenManager;

mod authorization_code_callback;
mod constants;
mod models;
mod oauth;
mod playback_mananger;
mod server;
mod spotify_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    //Wait for user permission
    let initial_access_token = acquire_access_token(&client).await;
    //Schedule access_token to be refreshed indefinitely
    let manager = TokenManager::new(initial_access_token, client.clone()).await;
    let token_channel = manager.channel();
    tokio::spawn(async move {
        manager.refresh().await;
    });
    // Detect playback changes
    let playback_manager = PlaybackManager::new(client.clone(), token_channel.clone());
    let playback_channel = playback_manager.receiver();
    tokio::spawn(async move {
        playback_manager.update().await;
    });

    let mut listener = playback_channel.clone();
    tokio::spawn(async move {
        while listener.changed().await.is_ok() {
            let playing = listener.borrow().clone()
                .map(|it| it.item.name)
                .unwrap_or_else(|| "Nothing".into());
            println!(
                "YOU ARE LISTENING TO: {}", playing
            );
        }
    });

    //Expose the currently playing track on a websocket. You can see its output on ws://localhost:8081
    websocket(playback_channel).await;

    Ok(())
}
