fn is_silent(message: &str) -> bool {
    let mut saw_non_silent_char = false;

    for (_i, c) in message.chars().enumerate() {
        if c.to_string() != "\n" && c.to_string() != "\t" &&
            c.to_string() != "\r" && c.to_string() != " " {
                saw_non_silent_char = true;
                break;
            }
    }

    !saw_non_silent_char
}

fn is_question(message: &str) -> bool {
    message.trim_right().chars().last() == Some('?')
}

fn is_shouty(message: &str) -> bool {
    let mut shouty_msg = String::new();

    for (_i, c) in message.trim().chars().enumerate() {
        if c.is_alphabetic() {
            shouty_msg.push(c);
        }
    }

    if shouty_msg.is_empty() {
        return false;
    }

    let mut is_shouty = true;

    for (_i, c) in shouty_msg.chars().enumerate() {
        if c.is_lowercase() {
            is_shouty = false
        }
    }

    is_shouty
}

pub fn reply(message: &str) -> &str {
    if is_silent(message) {
        return "Fine. Be that way!"
    } else if is_question(message) && is_shouty(message) {
        return "Calm down, I know what I'm doing!"
    } else if is_shouty(message) {
        return "Whoa, chill out!"
    } else if is_question(message) {
        return "Sure."
    }

    "Whatever."
}
