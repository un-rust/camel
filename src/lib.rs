//! Case conversion utilities for strings (PascalCase, camelCase, kebab-case, etc.).

use urlogger::{LogLevel, log};

/// Default characters that split a string into words (e.g. `foo-bar`, `foo_bar`).
const STR_SPLITTERS: &[char] = &['-', '_', '/', '.'];

/// Returns whether `c` is uppercase. Returns `None` for ASCII digits.
pub fn is_uppercase(c: char) -> Option<bool> {
    if c.is_ascii_digit() {
        return None;
    }
    let lower = c.to_lowercase().next().unwrap_or(c);
    Some(c != lower)
}

/// Splits a string into words at case boundaries and separators.
///
/// Handles camelCase, PascalCase, SCREAMING_SNAKE, etc. Uses `separators` (or
/// `STR_SPLITTERS`) to split. Empty segments are preserved (e.g. `foo--bar` → `["foo","","bar"]`).
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
            // Case rising edge: lower -> upper (e.g. camel|Case)
            if previous_upper == Some(false) && is_upper == Some(true) {
                if !buff.is_empty() {
                    parts.push(std::mem::take(&mut buff));
                }
                buff.push(c);
                previous_upper = is_upper;
                previous_splitter = Some(false);
                continue;
            }
            // Case falling edge: upper -> lower, buffer.len() > 1 (e.g. AB|c → A, Bc)
            if previous_upper == Some(true) && is_upper == Some(false) {
                let char_count = buff.chars().count();
                if char_count > 1 {
                    let last_char = buff.chars().last().unwrap();
                    // Byte index where the last character starts
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

/// Capitalizes the first character; rest unchanged.
pub fn upper_first(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        c.to_uppercase().collect::<String>() + chars.as_str()
    } else {
        String::new()
    }
}

/// Lowercases the first character; rest unchanged.
pub fn lower_first(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(c) = chars.next() {
        c.to_lowercase().collect::<String>() + chars.as_str()
    } else {
        String::new()
    }
}

/// Converts a string to PascalCase.
///
/// With `normalize == true`, each part is lowercased before capitalizing.
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

/// Converts a string to camelCase.
///
/// Uses `lower_first(pascal_case(s, normalize))`.
pub fn camel_case(s: &str, normalize: bool) -> String {
    lower_first(&pascal_case(s, normalize))
}

/// Converts a string to kebab-case (lowercase parts joined with `-`).
pub fn kebab_case(s: &str) -> String {
    lower_case_join(s, "-")
}

/// Converts a string to snake_case (lowercase parts joined with `_`).
pub fn snake_case(s: &str) -> String {
    lower_case_join(s, "_")
}

/// Converts a string to flatcase (all lowercase, no separators).
pub fn flat_case(s: &str) -> String {
    lower_case_join(s, "")
}

/// Converts a string to Train-Case (each word capitalized, joined by `-`).
///
/// With `normalize == true`, lowercases each part before capitalizing.
pub fn train_case(s: &str, normalize: bool) -> String {
    split_by_case(s, None)
        .into_iter()
        .filter(|p| !p.is_empty())
        .map(|p| {
            if normalize {
                upper_first(&p.to_lowercase())
            } else {
                upper_first(&p)
            }
        })
        .collect::<Vec<_>>()
        .join("-")
}

/// Splits by case, lowercases each part, joins with `joiner`.
fn lower_case_join(s: &str, joiner: &str) -> String {
    split_by_case(s, None)
        .into_iter()
        .map(|p| p.to_lowercase())
        .collect::<Vec<_>>()
        .join(joiner)
}

/// Minor words that remain lowercase in Title Case.
const TITLE_CASE_EXCEPTIONS: &[&str] = &[
    "a", "an", "and", "as", "at", "but", "by", "for", "if", "in", "is", "nor", "of", "on", "or",
    "the", "to", "with",
];

/// Converts a string to Title Case (like train-case but with spaces, minor words lowercase).
///
/// With `normalize == true`, lowercases each part before capitalizing (except `TITLE_CASE_EXCEPTIONS`).
pub fn title_case(s: &str, normalize: bool) -> String {
    split_by_case(s, None)
        .into_iter()
        .filter(|p| !p.is_empty())
        .map(|p| {
            let lower = p.to_lowercase();
            if TITLE_CASE_EXCEPTIONS.contains(&lower.as_str()) {
                lower
            } else if normalize {
                upper_first(&lower)
            } else {
                upper_first(&p)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Returns a greeting string. Example: `hello("world")` → `"Hello, world!"`.
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
    fn test_snake_case() {
        assert_eq!(snake_case("FooBarBaz"), "foo_bar_baz");
        assert_eq!(snake_case("FOO_BAR"), "foo_bar");
    }

    #[test]
    fn test_flat_case() {
        assert_eq!(flat_case(""), "");
        assert_eq!(flat_case("foo"), "foo");
        assert_eq!(flat_case("foo-bAr"), "foobar");
        assert_eq!(flat_case("FooBARb"), "foobarb");
        assert_eq!(flat_case("foo_bar-baz/qux"), "foobarbazqux");
        assert_eq!(flat_case("FOO_BAR"), "foobar");
        assert_eq!(flat_case("foo--bar-Baz"), "foobarbaz");
    }

    #[test]
    fn test_train_case() {
        assert_eq!(train_case("", false), "");
        assert_eq!(train_case("f", false), "F");
        assert_eq!(train_case("foo", false), "Foo");
        assert_eq!(train_case("foo-bAr", false), "Foo-B-Ar");
        assert_eq!(train_case("AcceptCH", false), "Accept-CH");
        assert_eq!(train_case("foo_bar-baz/qux", false), "Foo-Bar-Baz-Qux");
        assert_eq!(train_case("FOO_BAR", false), "FOO-BAR");
        assert_eq!(train_case("foo--bar-Baz", false), "Foo-Bar-Baz");
        assert_eq!(train_case("WWW-authenticate", false), "WWW-Authenticate");
        assert_eq!(train_case("WWWAuthenticate", false), "WWW-Authenticate");

        assert_eq!(train_case("AcceptCH", true), "Accept-Ch");
        assert_eq!(train_case("FOO_BAR", true), "Foo-Bar");
        assert_eq!(train_case("WWW-authenticate", true), "Www-Authenticate");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("", false), "");
        assert_eq!(title_case("f", false), "F");
        assert_eq!(title_case("foo", false), "Foo");
        assert_eq!(title_case("foo-bar", false), "Foo Bar");
        assert_eq!(title_case("this-IS-aTitle", false), "This is a Title");
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
