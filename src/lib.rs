use urlogger::{LogLevel, log};

pub fn is_uppercase(c: char) -> Option<bool> {
    if c.is_ascii_digit() {
        return None;
    }
    let lower = c.to_lowercase().next().unwrap_or(c);
    Some(c != lower)
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
