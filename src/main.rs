use clap::Parser;
use oci_dev_binder_hook::cli::{CLI, CLIExt};
use std::error::Error;
use std::io::{self, IsTerminal, stdin};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CLI::parse();
    let spec_from_stdin = !io::stdin().is_terminal();

    cli.run(stdin(), spec_from_stdin)
}
