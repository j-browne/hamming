use clap::{Parser, Subcommand};
use hamming::{decode, encode, Code};
use std::io::{stdin, stdout, Read, Write};

#[derive(Clone, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// The block size of the encoded data.
    /// For Hamming, this must be one less than a power of two.
    /// For Extended Hamming, this must be a power of two.
    #[clap(short, long, default_value_t = 16)]
    block_size: u32,
}

#[derive(Clone, Subcommand)]
enum Command {
    Decode,
    Encode,
}

fn main() {
    let cli = Cli::parse();

    let input = {
        let mut data = Vec::new();
        stdin().lock().read_to_end(&mut data).unwrap();
        data
    };

    let code = Code::from_block_size(cli.block_size).unwrap();
    let output = match cli.command {
        Command::Decode => decode(&input, &code).unwrap(),
        Command::Encode => encode(&input, &code).unwrap(),
    };
    stdout().lock().write_all(&output).unwrap();
}
