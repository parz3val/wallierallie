use crate::types::types::Config;
use dotenv;

pub fn config(query_args: String)-> Config {
    let search_url = String::from("");
    let oauth_url = String::from("https://unsplash.com/oauth/token");
    let random_url = String::from(format!("https://api.unsplash.com/photos/random/?wallpapers&orientation=landscape&query={}", query_args));
    let client_id = dotenv::var("client_id").expect("client_id not in .env");
    let client_secret = dotenv::var("client_secret").expect("client_secret  not in .env");
    Config {
       client_id,
       client_secret, 
       random_url,
       oauth_url,
       search_url,
    }
} 
