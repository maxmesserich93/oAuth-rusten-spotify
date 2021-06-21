#oAuth-rusten-spotify

##Goal
* Learn about the pkce Authorization Code Flow by implementing it.

* Learn about Rust and the tokio runtime by implementing the pkce Authorization Code Flow.
##About

An application telling you the name of the song you are listening to on spotify.
Prints to the console and published to a websocket at localhost:8081

##Setup
###Set up a Spotify API Client
* Go to https://developer.spotify.com/dashboard/, create a client and copy the Client ID.
* Add http://localhost:8080/callback as Redirect URI in the client settings
* Change the value of CLIENT_ID in constants.rs to your Client ID.

###Starting and using the application
* *cargo run*
* the console shows you a link that directs you to the spotify login site
* once you have allowed the application access, you should be redirected to localhost:8080/callback
* you can now see you current track in the console see it with websocket client listening to *ws://localhost:8081*

