# camel

<!-- automdrs:badges showCrateVersion="true" showCrateDownloads="true" showCrateDocs="true" showCommitActivity="true" showRepoStars="true" -->
![Crates.io Version](https://img.shields.io/crates/v/camel)
![Crates.io Total Downloads](https://img.shields.io/crates/d/camel)
![docs.rs](https://img.shields.io/docsrs/camel)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/un-rust/camel)
![GitHub Repo stars](https://img.shields.io/github/stars/un-rust/camel)
<!-- /automdrs -->

<!-- automdrs:description -->

Supports bidirectional conversion of variable strings between common naming formats.

<!-- /automdrs -->

**[Full documentation →](https://docs.rs/camel/)**

## Quick start

<!-- automdrs:cargo-add -->

```sh
cargo add camel
```

<!-- /automdrs -->

## Usage

<!-- automdrs:file src="./src/main.rs" -->
```rust
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
```
<!-- /automdrs -->

## License

<!-- automdrs:contributors author="UnRUST" license="Apache-2.0" -->
Published under the [Apache-2.0](./LICENSE) license.
Made by [@UnRUST](https://github.com/un-rust) 💛
<br><br>
<a href="https://github.com/un-rust/camel/graphs/contributors">
<img src="https://contrib.rocks/image?repo=un-rust/camel" />
</a>
<!-- /automdrs -->

<!-- automdrs:with-automdrs -->

---

_🛠️ auto updated with [automd-rs](https://github.com/betterhyq/automd-rs)_

<!-- /automdrs -->