use serde::de::value::Error;

pub fn get_command() -> Result<super::types::Command, Error> {
    let args: Vec<String> = std::env::args().collect();
    let q_args: String = String::from("wallpaper");

    if args.len() <= 1 {
        return Ok(super::types::Command {
            command: super::types::Cmd::Quiet,
            args: q_args,
        });
    }
    let interactive = &args[1];
    if interactive == "i" {
        return Ok(super::types::Command {
            command: super::types::Cmd::Interactive,
            args: q_args,
        });
    }
    let q_args = format!("{} {}", q_args, &args[1]);
    Ok(super::types::Command {
        command: super::types::Cmd::Quiet,
        args: q_args,
    })
}
