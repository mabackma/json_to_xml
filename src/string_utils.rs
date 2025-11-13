/// Removes Vec< and > from a string.
pub fn remove_vec(s: &str) -> String {
    if s.starts_with("Vec<") {
        s.chars().skip(4).take(s.len() - 5).collect()
    } else {
        s.to_string()
    }
}

/// Converts a string to camel case and adds prefix. 
/// Used for struct names and field types (eg. re:AreaNumber -> ReAreaNumber).
pub fn to_camel_case_with_prefix(s: &str) -> String {
    let mut new_string = String::new();
    let mut char_vec: Vec<char>;

    if s.starts_with("Vec<") {
        new_string.push_str("Vec<");
        for c in s.chars().skip(4).take(s.len() - 5) {
            if c == ':' {
                continue;
            } else {
                new_string.push(c);
            }
        }
        new_string.push('>');

        char_vec = new_string.chars().collect();
        char_vec[4] = char_vec[4].to_uppercase().next().unwrap();
    } else {
        for c in s.chars() {
            if c == ':' {
                continue;
            } else {
                new_string.push(c);
            }
        }

        char_vec = new_string.chars().collect();
        char_vec[0] = char_vec[0].to_uppercase().next().unwrap();
    }

    char_vec.into_iter().collect()
}

/// Converts a string to snake case. 
/// Used for field names (eg. AreaNumber -> area_number).
pub fn to_snake_case(s: &str) -> String {
    let char_vec: Vec<char> = s.chars().collect();
    let mut new_string  = String::new();

    for c in char_vec {
        if c.is_uppercase() && new_string.len() > 0 {
            new_string.push('_');
            new_string.push(c.to_lowercase().next().unwrap());
        } else {
            new_string.push(c);
        }
    }

    new_string.to_lowercase()
}

/// Capitalizes the first letter of a word.
pub fn capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Lowercases the first letter of a word. Changes @ to __.
pub fn lowercase_word(word: &str) -> String {
    if !word.starts_with('@') {
        let mut chars = word.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_lowercase().collect::<String>() + chars.as_str(),
        }
    } else {
        word.to_string().replace("@", "__")
    }
}
