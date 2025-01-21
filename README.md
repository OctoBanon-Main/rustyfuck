# Rustyfuck
A Brainfuck interpreter written in Rust, created out of boredom.

## Prerequisites
Before building the project, ensure you have the following installed:

[Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## Steps to Build

1. Clone the repository:
First, clone the project repository to your local machine:
```bash
git clone https://github.com/OctoBanon-Main/rustyfuck.git
cd rustyfuck
```

2. Build the project:
To compile the project, run the following command:
```bash
cargo build --release
```
The compiled executable will be available in the `/target/release/` directory.

## Usage
Once the project is built, you can run the Brainfuck interpreter with the following command:
- For Linux/macOS:
```bash
./target/release/rustyfuck <brainfuck-file> [input]
```
For Windows:
```bash
.\target\release\rustyfuck.exe <brainfuck-file> [input]
```
- `<brainfuck-file>`: Path to the Brainfuck source code file you want to interpret.
- `[input]`: Optional input string that the Brainfuck program will use for its input (if applicable).

Alternatively, you can check the version of the program by running:

- For Linux/macOS:
```bash
./target/release/rustyfuck -v  # or --version
```
For Windows:
```bash
.\target\release\rustyfuck.exe -v  # or --version
```
This will output the current version of the interpreter.

## License
This project is licensed under the Unlicense. Do whatever the fuck you want with it.
