use crate::polynom::{Polynom, Polynom64, MOD_POLYNOM};
use wasm_bindgen::prelude::*;

#[macro_export]
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// As per
// https://github.com/whyrusleeping/chunker/blob/fe64bd25879f446bb7e8a4adf5d4a68552211bd3/chunker.go#L12-L26
const KIB: usize = 1024;
const MIB: usize = 1024 * KIB;
pub const MIN_SIZE: usize = 512 * KIB;
pub const MAX_SIZE: usize = 8 * MIB;
pub const WINDOW_SIZE: usize = 16;
// AvgSize is the default average size of a chunk.
pub const AVG_SIZE: usize = 1048576;

#[derive(Debug, Clone, Eq, PartialEq)]
#[wasm_bindgen]
pub struct Rabin {
    // config
    min_size: usize,
    max_size: usize,

    window_size: usize, // The size of the data window used in the hash calculation.
    mask: u64,

    // Precalculations
    polynom_shift: usize,
    out_table: [Polynom64; 256],
    mod_table: [Polynom64; 256],

    init: u64,
    min_sans_preheat: usize,
    target_value: u64,
}

impl Default for Rabin {
    fn default() -> Self {
        Self::new_with_polynom(&MOD_POLYNOM, AVG_SIZE, MIN_SIZE, MAX_SIZE, WINDOW_SIZE)
    }
}

impl Rabin {
    pub fn create(bits: usize, min_size: usize, max_size: usize, window_size: usize) -> Self {
        Self::create_with_polynom(&MOD_POLYNOM, bits, min_size, max_size, window_size)
    }
    pub fn new(avg_size: usize, min_size: usize, max_size: usize, window_size: usize) -> Self {
        Self::create_with_polynom(
            &MOD_POLYNOM,
            avg_size.log2() as usize,
            min_size,
            max_size,
            window_size,
        )
    }
    pub fn new_with_polynom(
        mod_polynom: &Polynom64,
        avg_size: usize,
        min_size: usize,
        max_size: usize,
        window_size: usize,
    ) -> Self {
        Self::create_with_polynom(
            mod_polynom,
            avg_size.log2() as usize,
            min_size,
            max_size,
            window_size,
        )
    }
    pub fn create_with_polynom(
        mod_polynom: &Polynom64,
        bits: usize,
        min_size: usize,
        max_size: usize,
        window_size: usize,
    ) -> Self {
        let out_table = Self::calculate_out_table(window_size, mod_polynom);
        let mod_table = Self::calculate_mod_table(mod_polynom);
        let polynom_shift = (mod_polynom.degree() - 8) as usize;
        let mask = (1 << bits) - 1;

        let init =
            ((out_table[0] << 8) | 1) ^ (mod_table[(out_table[0] >> polynom_shift) as usize]);

        let mut window_data = Vec::with_capacity(window_size);
        window_data.resize(window_size, 0);

        Rabin {
            min_size: min_size,
            max_size: max_size,

            window_size: window_size,
            mask: mask,
            polynom_shift: polynom_shift,

            out_table: out_table,
            mod_table: mod_table,

            min_sans_preheat: if min_size < window_size {
                0
            } else {
                min_size - window_size
            },
            init,
            target_value: 0,
        }
    }

    fn calculate_out_table(window_size: usize, mod_polynom: &Polynom64) -> [Polynom64; 256] {
        let mut out_table = [0; 256];
        for b in 0..256 {
            let mut digest = (b as Polynom64).modulo(mod_polynom);
            for _ in 0..window_size - 1 {
                digest <<= 8;
                digest = digest.modulo(mod_polynom);
            }
            out_table[b] = digest;
        }

        out_table
    }

    fn calculate_mod_table(mod_polynom: &Polynom64) -> [Polynom64; 256] {
        let mut mod_table = [0; 256];
        let k = mod_polynom.degree();
        for b in 0..256 {
            let p = (b as u64) << k;
            mod_table[b] = p.modulo(mod_polynom) | p;
        }

        mod_table
    }

    pub fn split(&self, buffer: &[u8], use_all: bool) -> Vec<i32> {
        let post_buf_idx = buffer.len();

        let mut state;
        let mut cur_idx = 0;
        let mut last_idx;
        let mut next_round_max;
        let mut cuts = Vec::new();
        loop {
            last_idx = cur_idx;
            next_round_max = last_idx + self.max_size;

            // we will be running out of data, but still *could* run a round
            if next_round_max > post_buf_idx {
                // abort early if we are allowed to
                if !use_all {
                    return cuts;
                }
                // otherwise signify where we stop hard
                next_round_max = post_buf_idx
            }

            // in case we will *NOT* be able to run another round at all
            if cur_idx + self.min_size >= post_buf_idx {
                if use_all && post_buf_idx != cur_idx {
                    cuts.push((post_buf_idx - cur_idx) as i32);
                }
                return cuts;
            }

            // reset
            state = self.init;

            // preheat
            cur_idx += self.min_sans_preheat;
            for i in 1..self.window_size + 1 {
                if i == self.window_size {
                    state ^= self.out_table[1];
                } else {
                    state ^= self.out_table[0];
                }

                state = ((state << 8) | buffer[cur_idx] as u64)
                    ^ self.mod_table[(state >> self.polynom_shift) as usize];

                cur_idx += 1;
            }

            // cycle
            while cur_idx < next_round_max && ((state & self.mask) != self.target_value) {
                state ^= self.out_table[buffer[cur_idx - self.window_size] as usize];
                state = ((state << 8) | buffer[cur_idx] as u64)
                    ^ self.mod_table[(state >> self.polynom_shift) as usize];

                cur_idx += 1;
            }

            cuts.push((cur_idx - last_idx) as i32);
        }
    }
}
