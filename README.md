# Todo CLI (Rust)

A simple command-line todo application built in Rust.

## Features
- Add tasks
- List tasks
- Complete tasks
- Update tasks
- Delete tasks
- Persistent file storage

## Usage

- cargo run add "Learn Rust"
- cargo run list
- cargo run check 1
- cargo run update 1 "Master Rust"
- cargo run delete 1

## Build

cargo build --release

Binary will be located in:
target/release/todo_app

## Example

$ cargo run add Learn Rust
Task added successfully.

$ cargo run list
1. [ ] Learn Rust