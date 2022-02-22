use crate::{Code, Error};
use bitvec::{order::Lsb0, vec::BitVec};
use std::ops::BitXor;

pub fn encode(input: &[u8], code: &Code) -> Result<Vec<u8>, Error> {
    let input = BitVec::<u8, Lsb0>::from_slice(input);
    let mut output = BitVec::<u8, Lsb0>::new();
    for in_chunk in input.chunks(code.data_bits() as usize) {
        let mut in_chunk = in_chunk.iter().by_vals();

        // To use the same algorithm, we need an extra bit at the start for Hamming
        let block_size_pow_2 = match code {
            Code::Hamming(_) => code.block_bits() + 1,
            Code::EHamming(_) => code.block_bits(),
        } as usize;
        let mut out_chunk = BitVec::<u8, Lsb0>::with_capacity(block_size_pow_2);

        // Create a new chunk with the data in the right place, and 0s for the parity bits
        for i in 0..block_size_pow_2 {
            if i.is_power_of_two() || i == 0 {
                out_chunk.push(false);
            } else {
                out_chunk.push(in_chunk.next().unwrap_or(false));
            }
        }
        assert!(in_chunk.next().is_none());

        // Determine which parity bits need to be flipped
        let flips = out_chunk.iter_ones().reduce(usize::bitxor).unwrap_or(0);
        let mut idx = 1;
        while idx < code.block_bits() as usize {
            if flips & idx != 0 {
                *out_chunk.get_mut(idx).unwrap() = true;
            }
            idx <<= 1;
        }

        // The overall parity bit is only needed for Extended Hamming
        match code {
            Code::Hamming(_) => {
                output.extend(&out_chunk[1..]);
            }
            Code::EHamming(_) => {
                *out_chunk.get_mut(0).unwrap() = out_chunk.iter().fold(false, |a, b| a ^ *b);

                output.extend(out_chunk);
            }
        }
    }
    Ok(output.into_vec())
}
