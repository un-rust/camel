//! Usage examples for the camel case conversion library.

use camel::{camel_case, flat_case, kebab_case, pascal_case, snake_case, title_case, train_case};

fn main() {
    let s = "foo_bar-Baz";
    println!("Input:     {}", s);
    println!("camelCase: {}", camel_case(s, true));
    println!("PascalCase: {}", pascal_case(s, true));
    println!("kebab-case: {}", kebab_case(s));
    println!("snake_case: {}", snake_case(s));
    println!("flatcase:  {}", flat_case(s));
    println!("Train-Case: {}", train_case(s, true));
    println!("Title Case: {}", title_case(s, true));
}
