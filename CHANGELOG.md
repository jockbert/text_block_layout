
# Changelog

[Show diff of unreleased changes on GitHub](https://github.com/jockbert/text_block_layout/compare/v1.2.2...main).

## Release 1.2.2 (2025-12-30) [diff](https://github.com/jockbert/text_block_layout/compare/v1.2.1...v1.2.2)

### Other changes in 1.2.2

* Applies linter suggestions in newer versions of Rust compiler and Clippy.
* Updates to latest version of dependency `unicode-width`.
* Updates to latest version of dev dependency `criterion`.
* Add some property based tests using dev dependency `monkey_test`.

## Release 1.2.1 (2024-11-22) [diff](https://github.com/jockbert/text_block_layout/compare/v1.2.0...v1.2.1)

### Other changes in 1.2.1

* Applies linter suggestions in newer versions of Rust compiler and Clippy.
* Updates to latest version of dependency `unicode-width`.

## Release 1.2.0 (2021-01-28) [diff](https://github.com/jockbert/text_block_layout/compare/v1.1.0...v1.2.0)

### New features in 1.2.0

* Adds generic construction method `Block::of<T: toString>(T) -> Block`. This
  should for example make it easier to create a text block from a number `n`,
  just writing `Block::of(n)` instead of `n.to_string().into()`.

### Other changes in 1.2.0

* Deprecating method `Block::of_text`, which is a more blunt way to create a
  block from text, compared to `Block::of`.

## Release 1.1.0 (2021-01-12) [diff](https://github.com/jockbert/text_block_layout/compare/v1.0.0...v1.1.0)

### New features in 1.1.0

* Adds possibility to overlay blocks using `Block::in_front_of`.
* Adds new method `Block::add_multiple_texts` allowing appedning more lines of
  text to block.

## Release 1.0.0 (2021-01-10) [diff](https://github.com/jockbert/text_block_layout/compare/v0.1.1...v1.0.0)

### New features in 1.0.0

* Make `Block::of_text` handle unicode character count and just not byte length
  of strings, by taking the actual unicode text with into account.
* Add notion of block alignment when joining them.
* Enable padding the blocks in all four directions.

### Breaking changes in 1.0.0

* In the name of block alignment, breaking the API by renaming `left_of` to `beside_top` and `above` to `stack_left`.
* Changing to use references when using a `Block` as argument.

## Release 0.1.1 (2021-01-09) [diff](https://github.com/jockbert/text_block_layout/compare/v0.1.0...v0.1.1)

Mostly contains improvements to the documentation.

## Release 0.1.0 (2021-01-09) [diff](https://github.com/jockbert/text_block_layout/compare/init...v0.1.0)

Add initial embryo for the library.
