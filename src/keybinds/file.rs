use std::{env, fs, io, path::Path};

use super::Keybinds;

/// Read keybinds file in config directory as string, and create new file if it does not exist
pub fn read_or_create_keybinds_file() -> Result<String, io::Error> {
    // Find config folder or use default
    let Ok(config) = get_config_folder() else {
        eprintln!("Could not find home config folder file. Continuing with default config.");
        return Ok( Keybinds::default_file_contents());
    };

    let folder = format!("{config}/tui-chan");
    let filepath = format!("{folder}/keybinds.conf");

    // Create folder if it does not exist (non-recursive)
    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)?;
    }

    // Create file if it does not exist
    if !Path::new(&filepath).exists() {
        let default_contents = Keybinds::default_file_contents();
        fs::write(&filepath, &default_contents)?;
        // Return contents
        return Ok(default_contents);
    }

    // Read file
    fs::read_to_string(&filepath)
}

/// Get config home folder for Linux
fn get_config_folder() -> Result<String, env::VarError> {
    env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME").map(|home| format!("{}/.config", home)))
}
