//! This is a tool to convert the output of snarkjs zkey and witness files to a format
//! that can be used by the Wrencher library for generating benchmark datasets
//! for client side provers.
//!
//! # Usage
//!
//!
//! The tool can be used by running the following command:
//! ```bash
//! wrencher serialize-snarkjs --zkey-path <path-to-zkey-export-file> --witness-path <path-to-witness-file> --output <output-file>     

mod deserialize;
mod models;
mod serialize;

use clap::{Parser, Subcommand};
use deserialize::{deserialize_witness_file, deserialize_zkey_file};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    SerializeSnarkjs {
        /// expects output of snarkjs zkej (exports the zkey file to a JSON file)
        #[arg(short, long)]
        zkey_path: PathBuf,

        /// expects a JSON output with a vector of all the witness values (strings)
        #[arg(short, long)]
        witness_path: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::SerializeSnarkjs {
            zkey_path,
            witness_path,
            output,
        } => {
            let zkey = deserialize_zkey_file(zkey_path)?;
            let witness = deserialize_witness_file(witness_path)?;

            let serialized = serialize::convert_to_serialize_format(&zkey, &witness);

            serialize::serialize_snarkjs(&serialized, output).unwrap();
        }
    }

    Ok(())
}
