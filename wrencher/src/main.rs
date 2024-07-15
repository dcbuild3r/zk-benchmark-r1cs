mod deserialize;
mod models;
mod serialize;

use clap::{Parser, Subcommand};
use deserialize::deserialize_zkey_file;
use serialize::serialize_zkey_file;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ConvertZkey {
        /// expects output of snarkjs zkej (exports the zkey file to a JSON file)
        #[arg(short, long)]
        zkey: PathBuf,

        #[arg(short, long)]
        witness: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConvertZkey {
            zkey,
            witness,
            output,
        } => {
            let zkey_file = deserialize_zkey_file(zkey)?;

            serialize_zkey_file(&zkey_file, output)?;
        }
    }

    Ok(())
}
