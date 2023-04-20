mod config;
mod env;
mod types;
mod utils;
mod wal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd: types::Command = utils::get_command()?;

    let config: types::Config = cmd.try_into()?;

    wal::wallpaper_change(config).await?;

    Ok(())
}
