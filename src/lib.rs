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
            parts.push(std::mem::take(&mut buff));
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
                    // Byte index of the start of the last character
                    let new_len = buff
                        .char_indices()
                        .nth(char_count - 1)
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

pub fn upper_first(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        c.to_uppercase().collect::<String>() + chars.as_str()
    } else {
        String::new()
    }
}

pub fn lower_first(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        c.to_lowercase().collect::<String>() + chars.as_str()
    } else {
        String::new()
    }
}

/// Convert a string to PascalCase.
pub fn pascal_case(s: &str, normalize: bool) -> String {
    if s.is_empty() {
        return String::new();
    }
    split_by_case(s, None)
        .into_iter()
        .map(|p| {
            if normalize {
                upper_first(&p.to_lowercase())
            } else {
                upper_first(&p)
            }
        })
        .collect()
}

/// Convert a string to camelCase.
/// Uses `lower_first(pascal_case(s, normalize))` to match scule behavior.
pub fn camel_case(s: &str, normalize: bool) -> String {
    lower_first(&pascal_case(s, normalize))
}

/// Convert a string to kebab-case.
/// Splits by case, lowercases each part, joins with "-". Matches scule kebabCase.
pub fn kebab_case(s: &str) -> String {
    split_by_case(s, None)
        .into_iter()
        .map(|p| p.to_lowercase())
        .collect::<Vec<_>>()
        .join("-")
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

    #[test]
    fn test_upper_first() {
        assert_eq!(upper_first(""), "");
        assert_eq!(upper_first("foo"), "Foo");
        assert_eq!(upper_first("Foo"), "Foo");
    }

    #[test]
    fn test_lower_first() {
        assert_eq!(lower_first(""), "");
        assert_eq!(lower_first("foo"), "foo");
        assert_eq!(lower_first("Foo"), "foo");
    }

    #[test]
    fn test_camel_case() {
        assert_eq!(camel_case("FooBarBaz", true), "fooBarBaz");
        assert_eq!(camel_case("FOO_BAR", true), "fooBar");
    }

    #[test]
    fn test_kebab_case() {
        assert_eq!(kebab_case(""), "");
        assert_eq!(kebab_case("foo"), "foo");
        assert_eq!(kebab_case("foo/Bar"), "foo-bar");
        assert_eq!(kebab_case("foo-bAr"), "foo-b-ar");
        assert_eq!(kebab_case("foo--bar"), "foo--bar");
        assert_eq!(kebab_case("FooBAR"), "foo-bar");
        assert_eq!(kebab_case("ALink"), "a-link");
        assert_eq!(kebab_case("FOO_BAR"), "foo-bar");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(pascal_case("", true), "");
        assert_eq!(pascal_case("foo", true), "Foo");
        assert_eq!(pascal_case("foo-bAr", true), "FooBAr");
        assert_eq!(pascal_case("FooBARb", true), "FooBaRb");
        assert_eq!(pascal_case("foo_bar-baz/qux", true), "FooBarBazQux");
        assert_eq!(pascal_case("FOO_BAR", true), "FooBar");
        assert_eq!(pascal_case("foo--bar-Baz", true), "FooBarBaz");
    }

    #[test]
    fn test_split_by_case() {
        assert_eq!(split_by_case("", None), Vec::<String>::new());
        assert_eq!(split_by_case("foo", None), ["foo"]);
        assert_eq!(split_by_case("fooBar", None), ["foo", "Bar"]);
        assert_eq!(split_by_case("FooBarBaz", None), ["Foo", "Bar", "Baz"]);
        assert_eq!(split_by_case("FooBARb", None), ["Foo", "BA", "Rb"]);
        assert_eq!(
            split_by_case("foo_bar-baz/qux", None),
            ["foo", "bar", "baz", "qux"]
        );
        assert_eq!(
            split_by_case("foo--bar-Baz", None),
            ["foo", "", "bar", "Baz"]
        );
        assert_eq!(split_by_case("FOO_BAR", None), ["FOO", "BAR"]);
        assert_eq!(split_by_case("foo123-bar", None), ["foo123", "bar"]);
        assert_eq!(split_by_case("FOOBar", None), ["FOO", "Bar"]);
        assert_eq!(split_by_case("ALink", None), ["A", "Link"]);

        // Custom separators: ['\\', '.', '-']
        assert_eq!(
            split_by_case(r"foo\Bar.fuzz-FIZz", Some(&['\\', '.', '-'])),
            ["foo", "Bar", "fuzz", "FI", "Zz"]
        );

        // Custom separator: only ['_']
        assert_eq!(
            split_by_case("new-name-value", Some(&['_'])),
            ["new-name-value"]
        );
    }
}
