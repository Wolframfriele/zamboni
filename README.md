# Zamboni

A playground to test different methods that can speed up text input like: autocorrect, chording and predictive text.

## Features

- [x] A basic terminal environment that allows the user to type and delete text.
- [ ] Spell check based on finding words with the nearest edit distance.
- [ ] Context aware spell check based on n-grams and markov models
- [ ] Chording
- [ ] Transformer based text prediction

## Usage

This is mostly a protopyte application and not very useful. If you still want to try it out, you will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs/) is the simplest way to set this up on either Windows, Mac or Linux. 

To enter the application run the following from a terminal:

```
cargo run
```

This will open up a terminal editor, where you can type text. For now no special input method is added. Deleting a complete word can be done with ctrl + backspace.

Exiting the application can be done with ctrl + q.

## Contribution

Found a problem or have a suggestion? Feel free to open an issue.

