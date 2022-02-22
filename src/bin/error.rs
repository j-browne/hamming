use bitvec::{order::Lsb0, vec::BitVec};
use clap::{Parser, Subcommand};
use rand::{distributions::Uniform, thread_rng, Rng};
use std::io::{stdin, stdout, Read, Write};

#[derive(Clone, Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Subcommand)]
enum Command {
    Prob { probability: f64 },
    Total { total: usize },
}

fn main() {
    let cli = Cli::parse();

    let mut bits = {
        let mut data = Vec::new();
        stdin().lock().read_to_end(&mut data).unwrap();
        BitVec::<u8, Lsb0>::from_vec(data)
    };

    if bits.is_empty() {
        return;
    }

    let mut rng = thread_rng();
    match cli.command {
        Command::Prob { probability: p } => {
            assert!(p > 0.0);
            assert!(p < 1.0);
            let distr = Uniform::new(0.0, 1.0);
            for mut bit in &mut bits {
                if rng.sample(distr) < p {
                    bit.set(!*bit);
                }
            }
        }
        Command::Total { total: t } => {
            let distr = Uniform::new(0, bits.len());
            for idx in rng.sample_iter(distr).take(t) {
                let mut bit = bits.get_mut(idx).unwrap();
                bit.set(!*bit);
            }
        }
    }

    stdout().lock().write_all(&bits.into_vec()).unwrap();
}
