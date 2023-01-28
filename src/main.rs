mod utils;
mod types;
mod config;
use utils::utils::{ 
    get_random_photo_url, 
    download_image_to_cache, 
    save_wallpaper_prompt, 
    random_uuid_string,
    backup_path,
};
use wallpaper;
use config::config as settings;



#[tokio::main]
async fn main()->Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Wallie Rallie, Random wallpers from unsplash");
    let config = settings::config();
    let random_url = get_random_photo_url(config.clone()).await;
    let image = download_image_to_cache(&random_url.clone()).await.unwrap();
    println!("{}", image.clone());
    wallpaper::set_from_path(&image).expect("
    Couldn't Set wallpaper!");
    println!("Wallpaper changed to {}", image.clone());

    let mut input = String::new();
    println!("Is wallpaper changed? (y/n)");
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "y" {
        println!("Success!");
    }
    if input.trim() == "n" {
        wallpaper::set_from_path(&image).expect("
        Couldn't Set wallpaper!");
    }
    let image_id = random_uuid_string();
    let change_back: bool = save_wallpaper_prompt(image.clone(), image_id);
    println!("Current wallpaper is saved to ~/Pictures");
    if change_back {
        wallpaper::set_from_path(backup_path().to_str().unwrap()).expect("
        Couldn't Set wallpaper!");
    }
    println!("Thank you for using wallie rallie");
    Ok(())
}
