use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {

  let args = Cli::parse();

  exit(0);
}