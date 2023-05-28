use termion::event::Key;

/// Keybind configuration
#[allow(dead_code)]
pub struct Keybinds {
    pub up: Key,
    pub down: Key,
    pub left: Key,
    pub right: Key,

    pub page_next: Key,
    pub page_previous: Key,

    pub copy_thread: Key,
    pub copy_media: Key,
    pub open_thread: Key,
    pub open_media: Key,

    pub fullscreen: Key,
    pub reload: Key,
    pub help: Key,
    pub quit: Key,
}

/// Error parsing keybind
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ParseKeybindError {
    /// No key name was found in keybind
    MissingKeyName,
    /// Keyname character is not between ASCII 33-126.
    ///
    /// Valid characters include all ASCII letters, numbers, and symbols,
    /// but not space, control characters, or multi-byte unicode characters
    InvalidCharacterKeyName,
    /// Invalid name for 'special key', such as 'Backspace' or 'Up'
    InvalidSpecialKeyName,
    /// Too many modifier keys are in keybind.
    ///
    /// To use 'Shift' with characters, use capitalized form ('Ctrl A', not 'Ctrl Shift a')
    TooManyModifiers,
    /// Modifier key is not valid.
    ///
    /// Valid modifier keys include 'Ctrl' and 'Alt'
    ///
    /// To use 'Shift' with characters, use capitalized form ('Ctrl A', not 'Ctrl Shift a')
    UnknownModifier,
    /// Modifier cannot be used with 'special key', such as 'Backspace' or 'Up'
    ModifierWithSpecialKey,
}

/// Parse keybind string as `termion::event::Key`.
///
/// Include modifer key, by separating with a space ('Ctrl a').
/// Valid modifier keys include 'Ctrl' and 'Alt'.
///
/// To use 'Shift' with characters, use capitalized form ('Ctrl A', not 'Ctrl Shift a')
///
/// Space is used as separator, because plus ('+') can be used as key name.
#[allow(dead_code)]
pub fn parse_keybind(keybind: &str) -> Result<Key, ParseKeybindError> {
    let mut parts = keybind.split(' ').rev();

    // Last part is key name (must exist)
    let Some(keyname) = parts.next().filter(|str| !str.is_empty()) else {
        return Err(ParseKeybindError::MissingKeyName);
    };

    // Optional modifier, next from end
    let modifier = parts.next().filter(|str| !str.is_empty());

    // Anything before that is invalid
    if parts.next().is_some() {
        return Err(ParseKeybindError::TooManyModifiers);
    }

    // One character in keyname
    if let Some(ch) = keyname.chars().next() {
        if keyname.len() == 1 {
            // Check character is valid ASCII letter, number or symbol (not space)
            if !(33 as char..=126 as char).contains(&ch) {
                return Err(ParseKeybindError::InvalidCharacterKeyName);
            }

            // No modifier
            let Some(modifier) = modifier else {
                return Ok(Key::Char(ch));
            };

            // Use valid modifier
            let key = match modifier.to_lowercase().as_str() {
                "ctrl" => Key::Ctrl(ch),
                "alt" => Key::Alt(ch),
                _ => return Err(ParseKeybindError::UnknownModifier),
            };

            return Ok(key);
        }
    }

    // Cannot use modifier with special key
    if modifier.is_some() {
        return Err(ParseKeybindError::ModifierWithSpecialKey);
    }

    // Use valid special key name
    let key = match keyname.to_lowercase().as_str() {
        "backspace" => Key::Backspace,
        "left" => Key::Left,
        "right" => Key::Right,
        "up" => Key::Up,
        "down" => Key::Down,
        "home" => Key::Home,
        "end" => Key::End,
        "pageup" => Key::PageUp,
        "pagedown" => Key::PageDown,
        "backtab" => Key::BackTab,
        "delete" => Key::Delete,
        "insert" => Key::Insert,
        "esc" => Key::Esc,

        _ => return Err(ParseKeybindError::InvalidSpecialKeyName),
    };

    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_keybind_works() {
        use parse_keybind as parse;
        use ParseKeybindError::*;

        // Ok

        assert_eq!(parse("a"), Ok(Key::Char('a')));
        assert_eq!(parse("A"), Ok(Key::Char('A')));
        assert_eq!(parse("Ctrl a"), Ok(Key::Ctrl('a')));
        assert_eq!(parse("Ctrl A"), Ok(Key::Ctrl('A')));
        assert_eq!(parse("Alt z"), Ok(Key::Alt('z')));
        assert_eq!(parse("["), Ok(Key::Char('[')));
        assert_eq!(parse("!"), Ok(Key::Char('!')));
        assert_eq!(parse("~"), Ok(Key::Char('~')));
        assert_eq!(parse("Alt ^"), Ok(Key::Alt('^')));
        assert_eq!(parse("Ctrl 6"), Ok(Key::Ctrl('6')));
        assert_eq!(parse("Backspace"), Ok(Key::Backspace));
        assert_eq!(parse("Up"), Ok(Key::Up));

        // Err

        assert_eq!(parse(""), Err(MissingKeyName));
        assert_eq!(parse(" "), Err(MissingKeyName));
        assert_eq!(parse("  "), Err(MissingKeyName));
        assert_eq!(parse("a  "), Err(MissingKeyName));

        assert_eq!(parse("Ctrl Shift a"), Err(TooManyModifiers));
        assert_eq!(parse("Alt  a"), Err(TooManyModifiers));
        assert_eq!(parse("  a"), Err(TooManyModifiers));

        assert_eq!(
            parse(&(1 as char).to_string()),
            Err(InvalidCharacterKeyName)
        );

        assert_eq!(parse("Shift a"), Err(UnknownModifier));
        assert_eq!(parse("f a"), Err(UnknownModifier));

        assert_eq!(parse("Ctrl Backspace"), Err(ModifierWithSpecialKey));
        assert_eq!(parse("Ctrl Shift"), Err(ModifierWithSpecialKey));

        assert_eq!(parse("ä"), Err(InvalidSpecialKeyName));
    }
}
