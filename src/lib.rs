use urlogger::{LogLevel, log};

const STR_SPLITTERS: &[char] = &['-', '_', '/', '.'];

pub fn is_uppercase(c: char) -> Option<bool> {
    if c.is_ascii_digit() {
        return None;
    }
    let lower = c.to_lowercase().next().unwrap_or(c);
    Some(c != lower)
}

pub fn split_by_case(s: &str, separators: Option<&[char]>) -> Vec<String> {
    let splitters: std::collections::HashSet<char> = separators
        .unwrap_or(STR_SPLITTERS)
        .iter()
        .copied()
        .collect();

    if s.is_empty() {
        return vec![];
    }

    let mut parts = Vec::new();
    let mut buff = String::new();
    let mut previous_upper: Option<bool> = None;
    let mut previous_splitter: Option<bool> = None;

    for c in s.chars() {
        let is_splitter = splitters.contains(&c);
        if is_splitter {
            if !buff.is_empty() {
                parts.push(std::mem::take(&mut buff));
            }
            previous_upper = None;
            previous_splitter = Some(true);
            continue;
        }

        let is_upper = is_uppercase(c);
        if previous_splitter == Some(false) {
            // Case rising edge: 小写 -> 大写 (e.g. camel|Case)
            if previous_upper == Some(false) && is_upper == Some(true) {
                if !buff.is_empty() {
                    parts.push(std::mem::take(&mut buff));
                }
                buff.push(c);
                previous_upper = is_upper;
                previous_splitter = Some(false);
                continue;
            }
            // Case falling edge: 大写 -> 小写，且 buffer > 1 (e.g. AB|c -> A, Bc)
            if previous_upper == Some(true) && is_upper == Some(false) {
                let char_count = buff.chars().count();
                if char_count > 1 {
                    let last_char = buff.chars().last().unwrap();
                    let new_len = buff
                        .char_indices()
                        .nth(char_count - 2)
                        .map(|(i, _)| i)
                        .unwrap_or(0);
                    let rest = buff[..new_len].to_string();
                    parts.push(rest);
                    buff.clear();
                    buff.push(last_char);
                    buff.push(c);
                    previous_upper = is_upper;
                    previous_splitter = Some(false);
                    continue;
                }
            }
        }

        buff.push(c);
        previous_upper = is_upper;
        previous_splitter = Some(false);
    }

    if !buff.is_empty() {
        parts.push(buff);
    }

    parts
}

pub fn hello(name: &str) -> String {
    log!(LogLevel::Info, "lib.rs");
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hello() {
        assert_eq!(hello("world"), "Hello, world!");
    }
}
