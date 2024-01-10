use std::{io::Bytes, path::PathBuf};
use std::process::Command;

type Error = Box<dyn std::error::Error>;
pub async fn wallpaper_change(c: super::types::Config) -> Result<(), Error> {
    let random_photo_url = get_random_photo_url(c.clone()).await?;
    let image = download_image_to_cache(&random_photo_url).await?;
    wallpaper::set_from_path(&image).expect(
        "WallpaperError, Error while setting wallpaper, please check if the image exists",
    );

    println!("Wallpaper changed to {}", image);
    Command::new("wal")
        .arg("-i")
        .arg(image)
        .output()
        .expect("Failed to execute pywal");
    Ok(())
}

async fn get_random_photo_url(c: super::types::Config) -> Result<String, Error> {
    let url = c.clone().random_url;
    let client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header(
            "Authorization",
            format!("Bearer {}", get_access_token(c).await?),
        )
        .send()
        .await
        .expect("FetchError, Error while fetching random photo from unsplash: {}");

    let p: super::types::Photo = resp
        .json()
        .await
        .expect("JsonError, Error while parsing json from unsplash: {}");
    Ok(p.urls.raw)
}

async fn get_access_token(c: super::types::Config) -> Result<String, Error> {
    let url = c.oauth_url;
    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .header("User-Agent", "WallpaperChanger")
        .form(&[
            ("client_id", c.client_id),
            ("client_secret", c.client_secret),
            ("grant_type", "client_credentials".to_string()),
        ])
        .send()
        .await
        .expect("TokenFetchError, Error while fetching access token from unsplash: {}");

    let access_token: super::types::AccessToken = resp.json().await?;
    Ok(access_token.access_token)
}

async fn download_image_to_cache(url: &str) -> Result<String, Error> {
    let cache_dir = dirs::cache_dir()
        .ok_or("CacheDirError, Error while fetching cache directory")
        .expect("CacheDirError, Error while fetching cache directory");
    remove_artifacts("wallpaper.jpg");
    let bytes = reqwest::get(url).await?.bytes().await?;

    let uid = uuid::Uuid::new_v4();
    let file_path = cache_dir.join(format!("{}.jpg",uid ));
    std::fs::write(&file_path, bytes).expect("Error while writing image to cache");
    Ok(file_path.to_str().to_owned().ok_or("No file")?.into())
}

fn remove_artifacts(url: &str) {
    match std::fs::remove_file(
        dirs::cache_dir()
            .ok_or("Error while fetching cache dirs")
            .expect("Error while fetching cache dir")
            .join(url),
    ) {
        Ok(_) => println!("artifacts removed!"),
        Err(_) => println!("no artifacts to remove or err"),
    }
}
