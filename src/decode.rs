use crate::{Code, Error};
use bitvec::{order::Lsb0, vec::BitVec};
use std::ops::BitXor;

pub fn decode(input: &[u8], code: &Code) -> Result<Vec<u8>, Error> {
    let mut input = BitVec::<u8, Lsb0>::from_slice(input);
    let mut output = BitVec::<u8, Lsb0>::new();
    for in_chunk in input.chunks_mut(code.block_bits() as usize) {
        let mut out_chunk = BitVec::<u8, Lsb0>::with_capacity(code.data_bits() as usize);

        // The flipped bit (if there is one) is given by the xor of the indices of all set bits
        // If there is no error, the result is 0.
        // This is for Extended Hamming. For regular Hamming, there is no overall parity bit at
        // index 0, so all indices are off by 1.
        match code {
            Code::Hamming(_) => {
                let idx = in_chunk
                    .iter_ones()
                    .map(|x| x + 1)
                    .reduce(usize::bitxor)
                    .unwrap_or(0);
                if idx != 0 {
                    let mut bit = in_chunk.get_mut(idx - 1).unwrap();
                    bit.set(!*bit);
                }

                for (i, x) in in_chunk.iter().enumerate() {
                    if !(i + 1).is_power_of_two() {
                        out_chunk.push(*x);
                    }
                }
            }
            Code::EHamming(_) => {
                let idx = in_chunk.iter_ones().reduce(usize::bitxor).unwrap_or(0);
                if idx != 0 {
                    let mut bit = in_chunk.get_mut(idx).unwrap();
                    bit.set(!*bit);
                    drop(bit); // so that we can use in_chunk later

                    // If the overall parity is wrong, but idx == 0, then the overall parity bit
                    // was flipped. If that's the case, we don't care
                    if in_chunk.iter().fold(false, |a, b| a ^ *b) {
                        return Err(Error::Decode);
                    }
                }

                for (i, x) in in_chunk.iter().enumerate() {
                    if !i.is_power_of_two() && i != 0 {
                        out_chunk.push(*x);
                    }
                }
            }
        };
        output.extend(out_chunk);
    }
    Ok(output.into_vec())
}
