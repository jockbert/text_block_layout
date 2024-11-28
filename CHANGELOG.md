
# Changelog

[Show diff of unreleased changes on GitHub](https://github.com/jockbert/text_block_layout/compare/v1.2.1...main).

## Release 1.2.1 (2024-11-22) [diff](https://github.com/jockbert/text_block_layout/compare/v1.2.0...v1.2.1)

* Applies linter suggestions in newer versions of Rust compiler and Clippy.
* Updates to latest version of dependency `unicode-width`.

## Release 1.2.0 (2021-01-28) [diff](https://github.com/jockbert/text_block_layout/compare/v1.1.0...v1.2.0)

### New features

* Adds generic construction method `Block::of<T: toString>(T) -> Block`. This
  should for example make it easier to create a text block from a number `n`,
  just writing `Block::of(n)` instead of `n.to_string().into()`.

### Other changes

* Deprecating method `Block::of_text`, which is a more blunt way to create a
  block from text, compared to `Block::of`.

## Release 1.1.0 (2021-01-12) [diff](https://github.com/jockbert/text_block_layout/compare/v1.0.0...v1.1.0)

### New features

* Adds possibility to overlay blocks using `Block::in_front_of`.
* Adds new method `Block::add_multiple_texts` allowing appedning more lines of
  text to block.

## Release 1.0.0 (2021-01-10) [diff](https://github.com/jockbert/text_block_layout/compare/v0.1.1...v1.0.0)

### New features

* Make `Block::of_text` handle unicode character count and just not byte length
  of strings, by taking the actual unicode text with into account.
* Add notion of block alignment when joining them.
* Enable padding the blocks in all four directions.

### Breaking changes

* In the name of block alignment, breaking the API by renaming `left_of` to `beside_top` and `above` to `stack_left`.
* Changing to use references when using a `Block` as argument.

## Release 0.1.1 (2021-01-09) [diff](https://github.com/jockbert/text_block_layout/compare/v0.1.0...v0.1.1)

Mostly contains improvements to the documentation.

## Release 0.1.0 (2021-01-09) [diff](https://github.com/jockbert/text_block_layout/compare/init...v0.1.0)

Add initial embryo for the library.
