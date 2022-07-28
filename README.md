# Compression

[![.github/workflows/rust.yml](https://github.com/I-mikan-I/rust-compression/actions/workflows/rust.yml/badge.svg)](https://github.com/I-mikan-I/rust-compression/actions/workflows/rust.yml)

Reference implementation for different compression algorithms in rust.

## Features

- Huffman coding

## Structure

[/src/algorithm](./src/algorithm) - Algorithm implementations.

[/src/algorithm/huffman.rs](./src/algorithm/huffman.rs) - Huffman two-phase implementation.

[/src/algorithm/movetofront.rs](./src/algorithm/movetofront.rs) - Move to front transformation.

[/src/algorithm/bwt.rs](./src/algorithm/bwt.rs) - Burrows wheeler transformation.
