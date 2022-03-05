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

#[wasm_bindgen(js_name=createWithPolynomial)]
pub fn configure_with_polynom(
    mod_polynom: u64,
    bits: usize,
    min_size: usize,
    max_size: usize,
    window_size: usize,
) -> Rabin {
    Rabin::create_with_polynom(&mod_polynom, bits, min_size, max_size, window_size)
}

#[wasm_bindgen]
pub fn cut(rabin: &Rabin, bytes: &[u8], end: bool) -> Vec<i32> {
    rabin.split(bytes, end)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::fs;

    #[test]
    fn chunks_for_1mib() {
        let mut content = fs::read("./test/1MiB.txt").expect("failed to read file");
        content.append(&mut Vec::from("hello"));

        let rabin = create(18, 87381, 393216, 64);

        assert_eq!(
            cut(&rabin, &content, false),
            [353816, 112050, 147806, 393216]
        );
        assert_eq!(
            cut(&rabin, &content, true),
            [353816, 112050, 147806, 393216, 41693]
        );
    }

    #[test]
    fn should_be_empty() {
        let content = vec![b'a'; 10 * 256];
        let rabin = create(8, 18, 262144, 64);

        assert_eq!(cut(&rabin, &content, false), []);
        assert_eq!(cut(&rabin, &content, true), [10 * 256]);
    }

    #[test]
    fn should_respect_window_size() {
        let mut buffer = vec![b'a'; 2 * 256];
        buffer.append(&mut vec![b'b'; 119]);
        buffer.append(&mut vec![b'c'; 5 * 256]);

        let rabin = create(6, 48, 192, 64);
        assert_eq!(
            cut(&rabin, &buffer, true),
            [192, 192, 157, 64, 78, 192, 192, 192, 192, 192, 192, 76]
        );
    }

    #[test]
    fn chunks_for_rand_5mib_zstd() {
        use zstd;

        let content = fs::read("./test/rand_5MiB.zst").expect("failed to read file");
        let buffer =
            zstd::bulk::decompress(&content, content.len() * 2).expect("failed to decompress");
        let rabin = Rabin::new_with_polynom(&17437180132763653, 524288, 262144, 1048576, 16);

        assert_eq!(
            cut(&rabin, &buffer, false),
            [895059, 686255, 467859, 626819, 280748, 310603, 734239, 499556]
        );

        assert_eq!(
            cut(&rabin, &buffer, true),
            [895059, 686255, 467859, 626819, 280748, 310603, 734239, 499556, 741742]
        )
    }

    #[test]
    fn stateless_api() {
        let mut buffer = fs::read("./test/1MiB.txt").expect("failed to read file");
        buffer.append(&mut Vec::from("hello"));

        let rabin = create(18, 87381, 393216, 64);

        assert_eq!(cut(&rabin, &buffer[0..736976], false), [353816]);
        assert_eq!(
            cut(&rabin, &buffer[0..736976], true),
            [353816, 112050, 147806, 123304]
        );

        assert_eq!(
            cut(&rabin, &buffer, false),
            [353816, 112050, 147806, 393216]
        );

        assert_eq!(
            cut(&rabin, &buffer, true),
            [353816, 112050, 147806, 393216, 41693]
        );
    }

    #[test]
    fn dagger_compat() {
        let buffer = sharbage(524288);
        let rabin = create(18, 87381, 393216, 16);

        assert_eq!(cut(&rabin, &buffer, false), [189236]);
        assert_eq!(cut(&rabin, &buffer, true), [189236, 177457, 157595]);
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
