#![feature(int_log)]
mod chunker;
mod polynom;

pub use chunker::Rabin;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn create(bits: usize, min_size: usize, max_size: usize, window_size: usize) -> Rabin {
    Rabin::create(bits, min_size, max_size, window_size)
}

#[wasm_bindgen]
pub fn new_with_polynom(
    mod_polynom: u64,
    avg_size: usize,
    min_size: usize,
    max_size: usize,
    window_size: usize,
) -> Rabin {
    Rabin::new_with_polynom(&mod_polynom, avg_size, min_size, max_size, window_size)
}

#[wasm_bindgen]
pub fn cut(rabin: &mut Rabin, bytes: &[u8]) -> Vec<i32> {
    rabin.split(bytes, true)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;

    #[test]
    fn chunks_for_1mib() {
        let mut content = fs::read("./test/1MiB.txt").expect("failed to read file");
        content.append(&mut Vec::from("hello"));

        let mut rabin = create(18, 87381, 393216, 64);
        let sizes = cut(&mut rabin, &content);

        assert_eq!(sizes, [366598, 239921, 260915]);
    }

    #[test]
    fn sholud_be_empty() {
        let content = vec![b'a'; 10 * 256];
        let mut rabin = create(8, 18, 262144, 64);
        let sizes = cut(&mut rabin, &content);

        assert_eq!(sizes, []);
    }
}
