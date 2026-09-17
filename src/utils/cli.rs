use clap::Parser;

#[derive(Parser)]
#[command(name = "rustyfuck")]
pub struct Args {
    /// Path to the Brainfuck source file.
    #[arg(short, long)]
    pub path: String
}