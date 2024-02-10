use libset::{Config, Error, FileType, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    title: String,
}

fn main() -> Result<(), Error> {
    let config = Config::new("org.example.Demo", 1, None)?;
    config.set(
        "config",
        Settings {
            title: "Demo".into(),
        },
        FileType::Toml,
    )?;
    Ok(())
}
