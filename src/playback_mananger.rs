use reqwest::{Client};
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::{watch};
use tokio::time;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    height: f64,
    width: f64,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    images: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub album: Album,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Playing {
    pub item: Item,
}

pub struct PlaybackManager {
    token_receiver: watch::Receiver<String>,
    sender: watch::Sender<Option<Playing>>,
    receiver: watch::Receiver<Option<Playing>>,
    client: Client,
    state: Option<Playing>,
}

impl PlaybackManager {
    pub fn new(client: Client, token_receiver: watch::Receiver<String>) -> PlaybackManager {
        let (sender, receiver) = watch::channel(None);
        PlaybackManager {
            token_receiver,
            sender,
            receiver,
            client,
            state: None,
        }
    }

    pub fn receiver(&self) -> watch::Receiver<Option<Playing>> {
        self.receiver.clone()
    }

    pub async fn update(mut self) {
        let mut interval = time::interval(time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let token = &self.token_receiver.borrow().to_string();
            let response = self
                .client
                .get("https://api.spotify.com/v1/me/player")
                .bearer_auth(token)
                .send()
                .await
                .unwrap();

            let playing = response.json::<Playing>().await.ok();

            let new_state = compare_state(&self.state, playing);

            match new_state {
                None => {}
                Some(new) => {
                    self.sender.send(new.clone()).unwrap();
                    self.state = new;
                }
            }
        }
    }
}

fn compare_state(previous: &Option<Playing>, current: Option<Playing>) -> Option<Option<Playing>> {
    let new_state: Option<Option<Playing>> = match (previous, current) {
        //Playback started
        (None, Some(started)) => Some(Some(started)),
        //Playback stopped
        (Some(_), None) => Some(None),
        //Check for change
        (Some(previous), Some(current)) => {
            if previous.item.id != current.item.id {
                Some(Some(current))
            } else {
                None
            }
        }
        _ => None,
    };
    new_state
}
