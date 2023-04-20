use std::{error, io::Stderr};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub random_url: String,
    pub oauth_url: String,
    cmd: Cmd,
}
impl TryFrom<Command> for Config {
    fn try_from(cmd: Command) -> Result<Self, Self::Error> {
        match cmd.command {
            Cmd::Quiet => {
                return Ok(Self {
                    client_id: super::env::client_id.to_string(),
                    client_secret: super::env::client_secret.to_string(),
                    random_url: random_urlinator(cmd.args),
                    oauth_url: super::env::oauth_url.to_string(),
                    cmd: Cmd::Quiet,
                });
            }
            Cmd::Interactive => {
                let args = ask_args();
                return Ok(Self {
                    client_id: super::env::client_id.to_string(),
                    client_secret: super::env::client_secret.to_string(),
                    random_url: random_urlinator(args),
                    oauth_url: super::env::oauth_url.to_string(),
                    cmd: Cmd::Interactive,
                });
            }
        }
    }
    type Error = Box<dyn error::Error>;
}

fn ask_args() -> String {
    String::from("nature")
}
fn random_urlinator(s: String) -> String {
    String::from(format!(
        "https://api.unsplash.com/photos/random/?wallpapers&orientation=landscape&query={}",
        s
    ))
}
#[derive(Debug, Deserialize, Clone)]
pub struct Photo {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub urls: Urls,
    pub links: Links,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Links {
    pub download_location: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Urls {
    pub full: String,
    pub raw: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AccessToken {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Command {
    pub command: Cmd,
    pub args: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Cmd {
    Interactive,
    Quiet,
}
