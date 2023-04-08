use crate::types::types::{AccessToken, Config, Photo};
use std::{fs::copy, fs::write, path::PathBuf};
use serde::Serialize;

pub async fn get_access_token(config: Config) -> Result<String, Box<dyn std::error::Error>> {
    let url = config.oauth_url;
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("User-Agent", "reqwest")
        .form(&[
            ("client_id", config.client_id),
            ("client_secret", config.client_secret),
            ("grant_type", "client_credentials".to_string()),
        ])
        .send()
        .await
        .expect(
            "TokenFetchError, Error while fetching access token from unsplash: {}"
        );
    
    let access_token: AccessToken = response.json().await?;
    Ok(access_token.access_token)
}

pub async fn _get_access_token(config: Config) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();


    let response = client.post(&config.oauth_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("username", config.client_id)
        .header("password", config.client_secret)
        .send()
        .await?;

    println!("{:?}", response);
    panic!("END");
    Ok("".to_string())
}
pub async fn get_random_photo_url(config: Config) -> String {
    let url = config.clone().random_url;
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", get_access_token(config).await.expect("TokenError,
            Error while fetching access token from unsplash: {}")),
        )
        .send()
        .await
        .expect(
            "FetchError, Error while fetching random photo from unsplash: {}",
        );
    let photo: Photo = response.json().await.expect(
        "JsonError, Error while parsing json from unsplash: {}",
    );
    photo.urls.raw
}

pub async fn _download_image_unsplash(
    url: String,
    access_token: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    //let response = client.get(url).header("Authorization", format!("Bearer {}", access_token.clone())).send().await.unwrap();
    //let image_url: ImageUrl = response.json().await?;
    let image_bytes = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token.clone()))
        .send()
        .await
        .unwrap();
    write("image.jpg", image_bytes.bytes().await?)?;
    Ok(())
}

fn backup_current_wallpaper() {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
    let backup_path = cache_dir.join("backup.jpg");
    match copy(cache_dir.clone().join("wallpaper.jpg"), backup_path) {
        Ok(_) => {
            println!("Backup created");
        }
        Err(_) => {
            println!("Backup failed");
        }
    }
}

fn revert_current_wallpaper() {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
    let new_path = cache_dir.join("wallpaper");
    match copy(cache_dir.clone().join("backup"), new_path){
        Ok(_) => {
            println!("Backup restored");
        }
        Err(_) => {
            println!("No wallpaper to restore back to! Maybe its your first run ?");
        }
    }
}

pub fn backup_path() -> PathBuf {
    let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
    let backup_path = cache_dir.join("backup.jpg");
    backup_path
}

pub async fn download_image_to_cache(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    backup_current_wallpaper();
    let cache_dir = dirs::cache_dir().ok_or("no cache dir").expect("Cache directory couldn't be found");
    // delete the pre-existing wallpaper.jpg
    match std::fs::remove_file(cache_dir.join("wallpaper.jpg")) {
        Ok(_)=> {
            println!("Removed the previous artificat");
        }
        Err(_)=> {
            println!("Something went wrong");
        }
    }
    // generate new uuid for the wallpaper
    let uuid = random_uuid_string();
    let wallpaper_name = format!("{}.jpg", uuid);
    let file_path = cache_dir.join(wallpaper_name);
    let bytes_ = reqwest::get(url).await?.bytes().await?;
    write(&file_path, bytes_).expect("Failed to write image to cache!");
    Ok(file_path.to_str().to_owned().ok_or("no file path")?.into())
}

pub fn save_wallpaper_prompt(image: String, image_id: String) -> bool {
    let mut input = String::new();
    println!("Do you want to save this wallpaper? (y/n)");
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "y" {
        let picture_dir = dirs::picture_dir().unwrap();
        let file_path = picture_dir.join(format!("{}.jpg", image_id));
        // save the file to caches wallpaper.jpg as well
        let cache_dir = dirs::cache_dir().ok_or("no cache dir").unwrap();
        let wallpaper_path = cache_dir.join("wallpaper.jpg");
        copy(image.clone(), wallpaper_path).unwrap();
        println!("Your wallpaper is saved in ~/Pictures directory!");
        copy(image, file_path).unwrap();
    }
    if input.trim() == "n" {
        let mut input = String::new();
        println!("Go back to previous? (y/n)");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "y" {
            revert_current_wallpaper();
            println!("Wallpaper reverted back to old");
            return true;
        }

        return false;
    }
    false
}

pub fn random_uuid_string() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}
