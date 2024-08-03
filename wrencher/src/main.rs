//! This is a tool to convert the output of snarkjs zkey and witness files to a format
//! that can be used by the wrencher library for generating benchmark datasets
//! for client side provers.
//!
//! # Usage
//!
//! Note that the tool accepts all of the `.wtns.json` files that are available in the `witness-dir` directory.
//!
//! The tool can be used by running the following command:
//!
//! - To serialize a zkey and witness file to a format that can be used by the wrencher library:
//! ```bash
//! wrencher ser-zkey --zkey-path <path-to-zkey-export-file> --witness-dir <path-to-witness-directory> --output <output-file>     
//!
//! - To serialize a r1cs and witness file to a format that can be used by the wrencher library:
//! ```bash
//! wrencher ser-r1cs --r1cs-path <path-to-r1cs-export-file> --witness-dir <path-to-witness-dir> --output <output-file>
//! ```

mod deserialize;
mod models;
mod r1cs;
mod serialize;

use clap::{Parser, Subcommand};
use deserialize::{deserialize_r1cs_json, deserialize_witnesses_json, deserialize_zkey_json};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "ser-zkey")]
    SerializeSnarkJsWithWitnessesZkey {
        /// expects output of snarkjs zkej (exports the zkey file to a JSON file)
        #[arg(short, long)]
        zkey_path: PathBuf,

        /// expects a JSON output with a vector of all the witness values (strings)
        #[arg(short, long)]
        witness_dir: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    #[command(name = "ser-r1cs")]
    SerializeSnarkJsWithWitnessesR1cs {
        /// expects output of snarkjs zkej (exports the zkey file to a JSON file)
        #[arg(short, long)]
        r1cs_path: PathBuf,

        /// expects a JSON output with a vector of all the witness values (strings)
        #[arg(short, long)]
        witness_dir: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::SerializeSnarkJsWithWitnessesZkey {
            zkey_path,
            witness_dir,
            output,
        } => {
            let zkey = deserialize_zkey_json(zkey_path)?;
            let witnesses = deserialize_witnesses_json(witness_dir)?;

            let serialized =
                serialize::convert_zkey_witnesses_to_serialize_format(&zkey, &witnesses);

            serialize::serialize_snarkjs(&serialized, output).unwrap();
        }
        Commands::SerializeSnarkJsWithWitnessesR1cs {
            r1cs_path,
            witness_dir,
            output,
        } => {
            let r1cs = deserialize_r1cs_json(r1cs_path)?;
            let witnesses = deserialize_witnesses_json(witness_dir)?;

            let serialized =
                serialize::convert_r1cs_witnesses_to_serialize_format(&r1cs, &witnesses);

            serialize::serialize_snarkjs(&serialized, output).unwrap();
        }
    }

    Ok(())
}
