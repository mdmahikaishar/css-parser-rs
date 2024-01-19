# Css Parser Rs

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

`Css Parser Rs` is a Rust crate for parsing CSS file. It supports reading CSS file in an event-based fashion. This allows you to receive events for different selectors, rules and comments during the parsing process.

## Features

- **Event-Based Parsing:** Receive events for different selectors, rules and comments during parsing.

## Usage

Add this crate to your `Cargo.toml` file:

```toml
[dependencies]
css-parser-rs = "0.1.0"
```

```rs
use css_parser_rs::Lexer;
use std::fs;

fn main() {
    let content = fs::read_to_string("./examples/styles.css").expect("ERROR: couldn't read file.");

    for event in Lexer::new(&content).parse() {
        println!("{event:?}");
    }
}
```

## Events

- `StartSelector(NAMES)`: Triggered when an CSS selector starts.

- `EndSelector(NAMES)`: Triggered when an CSS selector ends.

- `Rule(KEY, VALUE),`: Triggered when an CSS rule is encountered.

- `Comment(VALUE)`: Triggered when a comment is encountered.

## Contributing

Contributions are welcome! I would like you to contribute in this project.

## Roadmap

This project is in its early stages, and there are many missing features that need implementation. Check the [Issues](https://github.com/mdmahikaishar/css-parser-rs/issues) section for a list of features, enhancements, and bug fixes that are planned.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/mdmahikaishar/css-parser-rs/LICENSE) file for details.
