use std::convert::TryFrom;
use std::str::FromStr;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use anyhow::{Context, Ok};
use clap::{Parser, Subcommand};
use pngme::chunk::Chunk;
use pngme::chunk_type::ChunkType;
use pngme::png::Png;
use pngme::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes a secret message into the given PNG file.
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
    },
    /// Tries to decode a secret message from the given PNG file.
    Decode { path: PathBuf, chunk_type: String },
    /// Tries to removes a secret message from the given PNG file.
    Remove { path: PathBuf, chunk_type: String },
    /// Prints given PNG file.
    Print { path: PathBuf },
}

fn read_bytes_from_file(path: &PathBuf) -> Result<Vec<u8>> {
    let mut f =
        File::open(&path).with_context(|| format!("File not found: '{}'", path.display()))?;
    let metadata = fs::metadata(&path).with_context(|| "unable to read metadata")?;
    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    Ok(buffer)
}

fn write_bytes_to_file(path: &PathBuf, bytes: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(bytes)
        .with_context(|| format!("Error when writing to: '{}'", path.display()))?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode {
            path,
            chunk_type,
            message,
        } => {
            let bytes = read_bytes_from_file(&path)?;
            let mut png = Png::try_from(bytes.as_slice())?;
            let chunk = Chunk::new(ChunkType::from_str(&chunk_type)?, message.into_bytes());
            png.append_chunk(chunk);
            write_bytes_to_file(&path, &png.as_bytes())?;
        }
        Commands::Decode { path, chunk_type } => {
            let bytes = read_bytes_from_file(&path)?;
            let png = Png::try_from(bytes.as_slice())?;
            if let Some(secret_chunk) = png.chunk_by_type(&chunk_type) {
                println!(
                    "{}",
                    String::from_utf8(secret_chunk.data().to_vec())
                        .with_context(|| "Error converting chunk data to UTF8!")?
                )
            } else {
                println!("No secret found :(");
            }
        }
        Commands::Remove { path, chunk_type } => {
            let bytes = read_bytes_from_file(&path)?;
            let mut png = Png::try_from(bytes.as_slice())?;
            png.remove_chunk(&chunk_type)?;
            write_bytes_to_file(&path, &png.as_bytes())?;
        }
        Commands::Print { path } => {
            let bytes = read_bytes_from_file(&path)?;
            let png = Png::try_from(bytes.as_slice())?;
            println!("{}", png);
        }
    }

    Ok(())
}
