# Rustyfuck

**A Brainfuck interpreter written in Rust, created out of boredom.**

Rustyfuck is a simple Brainfuck interpreter written in Rust. It was created as a fun project to explore Rust and provide an efficient way to run Brainfuck code

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
./target/release/rustyfuck <path>
```

- For Windows:
```bash
.\target\release\rustyfuck.exe <path>
```

- `<path>`: Path to the Brainfuck source code file you want to interpret.

Alternatively, you can check the version of the program by running:

- For Linux/macOS:

```bash
./target/release/rustyfuck -V  # or --version
```

- For Windows:
```bash
.\target\release\rustyfuck.exe -V  # or --version
```

This will output the current version of the interpreter.

## See also

[Braingear](https://github.com/honakac/braingear) - Brainfuck interpreter and compiler written in C by honakac

## License

This project is licensed under the Unlicense. Do whatever the fuck you want with it.

> i said gg five times
