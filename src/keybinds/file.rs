use std::{env, fs, io, path::Path};

/// Read keybinds file in config directory as string, and create new file if it does not exist
pub fn read_or_create_keybinds_file() -> Result<String, io::Error> {
    let config = get_config_folder().expect("Cannot find config home folder");

    let folder = format!("{config}/tui-chan");
    let filepath = format!("{folder}/keybinds.conf");

    // Create folder if it does not exist (non-recursive)
    if !Path::new(&folder).exists() {
        fs::create_dir(&folder)?;
    }

    // Create file if it does not exist
    if !Path::new(&filepath).exists() {
        let default_contents = String::from("# Keybinds for tui-chan\n");
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
