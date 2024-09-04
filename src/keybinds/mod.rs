mod file;
mod key;

pub use self::file::read_or_create_keybinds_file;
pub use self::key::{display_key, ParseErrorKind};

use std::collections::HashMap;
use termion::event::Key;

use self::key::parse_keybind;

// Creates `pub struct Keybinds`
macro_rules! define_keybinds {
    { $(
        $name:ident                 // ID
        $($mod:ident)? $key:literal // DEFAULT KEYBIND
        #[doc = $desc:literal]      // DESCRIPTION
    )* $(,)? } => {
        /// Keybind configuration
        #[derive(Debug)]
        pub struct Keybinds { $(
            #[doc = $desc]
            pub $name: Key,
        )* }

        impl Keybinds {
            /// Parse keybinds from configuration file (`.conf`)
            ///
            /// Uses default value if keybind not given
            pub fn parse_from_file(file: &str) -> Result<Self, KeybindsError> {

                // Get key/value pairs as hashmap
                let mut keymap = parse_keymap_file(file)?;

                // Construct self
                Ok(Self { $(
                    $name:
                        // Use default value, if not defined in file
                        keymap.remove(stringify!($name))
                            .unwrap_or_else(|| define_keybinds!(@modifier $($mod)?)($key)),
                )* })
            }

            /// Get contents of keybind file, with default configuration
            pub fn default_file_contents() -> String {
                let mut contents = String::from("# Keybinds for tui-chan\n# https://github.com/tuqqu/tui-chan\n\n");

                $(
                    let key = define_keybinds!(@modifier $($mod)?)($key);

                    contents += &format!("#{}\n{}={}\n",
                        $desc,
                        stringify!($name),
                        display_key(&key),
                    );
                )*

                contents
            }
        }
    };

    // Use `Char` if no modifier given
    (@modifier $mod:ident) => { Key::$mod };
    (@modifier           ) => { Key::Char };
}

define_keybinds! {
 // ID            DEFAULT   DESCRIPTION
    up                 'w'  /// Move up
    down               's'  /// Move down
    left               'a'  /// Move left
    right              'd'  /// Move right
    quick_up      Ctrl 'w'  /// Move up quickly
    quick_down    Ctrl 's'  /// Move down quickly
    quick_left    Ctrl 'a'  /// Move left quickly
    quick_right   Ctrl 'd'  /// Move right quickly
    page_next          'p'  /// Next page
    page_previous Ctrl 'p'  /// Previous page
    copy_thread        'c'  /// Copy the direct url to the selected thread or post
    open_thread        'o'  /// Open the selected thread or post in browser
    copy_media    Ctrl 'c'  /// Copy the selected post media (image/webm) url
    open_media    Ctrl 'o'  /// Open the selected post media (image/webm) in browser
    fullscreen         'z'  /// Toggle fullscreen for the selected panel
    reload             'r'  /// Reload page
    help               'h'  /// Toggle help bar
    quit               'q'  /// Quit
    // Default `quit` keybind must also be updated in `event.rs`
}

/// Error parsing keybind configuration file
#[derive(Debug)]
#[allow(dead_code)]
pub enum KeybindsError {
    /// Failed to parse single keybind
    Parse {
        /// Line number
        line_no: usize,
        /// Parsing error kind
        kind: ParseErrorKind,
        /// Name of keybind
        name: String,
        /// Keybind value
        keybind: String,
    },

    /// Keybind was already defined in file
    KeybindAlreadyDefined {
        /// Line number
        line_no: usize,
        /// Name of keybind
        name: String,
    },

    /// No name in keybind definition
    NoName {
        /// Line number
        line_no: usize,
    },

    /// No value in keybind definition
    NoValue {
        /// Line number
        line_no: usize,
    },
}

/// Map keybind name to key
type KeyMap<'a> = HashMap<&'a str, Key>;

/// Parse keybinds file, as hashmap
fn parse_keymap_file(file: &str) -> Result<KeyMap, KeybindsError> {
    let mut keymap = KeyMap::new();

    // Loop lines
    for (line_no, line) in file.lines().enumerate() {
        let line_no = line_no + 1;

        // Ignore blank lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut split = line.split('=');

        // Name of keybind
        let name = split
            .next()
            .filter(|name| !name.is_empty())
            .ok_or(KeybindsError::NoName { line_no })?
            .trim();

        // Check name not already defined
        if keymap.contains_key(name) {
            return Err(KeybindsError::KeybindAlreadyDefined {
                line_no,
                name: name.to_string(),
            });
        }

        // Value of keybind
        let keybind = split
            .next()
            .filter(|name| !name.is_empty())
            .ok_or(KeybindsError::NoValue { line_no })?
            .trim();

        // Parse as `Key`
        let key = parse_keybind(keybind).map_err(|kind| KeybindsError::Parse {
            line_no,
            kind,
            name: name.to_string(),
            keybind: keybind.to_string(),
        })?;

        keymap.insert(name, key);
    }

    Ok(keymap)
}
