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
    rabin.split(bytes, false)
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
    fn should_be_empty() {
        let content = vec![b'a'; 10 * 256];
        let mut rabin = create(8, 18, 262144, 64);
        let sizes = cut(&mut rabin, &content);

        assert_eq!(sizes, []);
    }

    #[test]
    fn should_respect_window_size() {
        let mut buffer = vec![b'a'; 2 * 256];
        buffer.append(&mut vec![b'b'; 119]);
        buffer.append(&mut vec![b'c'; 5 * 256]);

        let mut rabin = create(6, 48, 192, 64);
        let sizes = cut(&mut rabin, &buffer);
        assert_eq!(sizes, [192, 192, 192, 65, 192, 192, 192, 192, 192, 192]);
    }

    #[test]
    fn chunks_for_rand_5mib_zstd() {
        use zstd;

        let content = fs::read("./test/rand_5MiB.zst").expect("failed to read file");
        let buffer =
            zstd::bulk::decompress(&content, content.len() * 2).expect("failed to decompress");
        let mut rabin = new_with_polynom(17437180132763653, 524288, 262144, 1048576, 16);
        let sizes = cut(&mut rabin, &buffer);

        assert_eq!(
            sizes,
            [895059, 686255, 467859, 626819, 280748, 310603, 734239, 499556]
        );
    }

    #[test]
    fn stateless_api() {
        let mut buffer = fs::read("./test/1MiB.txt").expect("failed to read file");
        buffer.append(&mut Vec::from("hello"));

        let mut rabin = create(18, 87381, 393216, 64);

        assert_eq!(cut(&mut rabin, &buffer[0..736976]), [366598, 239921]);
        assert_eq!(cut(&mut rabin, &buffer), [366598, 239921, 260915]);
    }

    #[test]
    fn dagger_compat() {
        let buffer = sharbage(524288);
        let mut rabin = create(18, 87381, 393216, 16);

        assert_eq!(cut(&mut rabin, &buffer), [189236, 177457, 157595]);
    }

    fn sharbage(capacity: usize) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        let mut buffer = Vec::new();
        let mut bytes = Vec::from("hello world");
        while buffer.len() < capacity {
            let out = Sha256::digest(&bytes);
            bytes = out.to_vec();
            buffer.extend(out);
        }
        buffer.truncate(capacity);

        return buffer;
    }
}
