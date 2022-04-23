# cargo-quickbench

Simple command to quickly create `cargo bench` packages.

## Installation

`cargo install cargo-quickbench`

## Usage

`cargo quickbench` is a small wrapper around `cargo new` and `cargo init` and works exactly the same way, just prepend `quickbench` to your commands, like so:

```bash
$ cargo quickbench new package-name
$ mkdir quick-benchmark && cd quick-benchmark && cargo quickbench init --verbose
```
