use sha2;
use sha2::Digest;
use std::path;
use std::fs;
use std::io;
use std::cmp::{Eq, PartialEq};

const Sha256_size: usize = 32;

#[derive(std::hash::Hash, Eq, PartialEq)]
pub struct Hash {
    bytes: [u8; Sha256_size]
}

impl Hash {
    pub fn from_sha2_hasher(hasher: sha2::Sha256) -> Hash {
        let digest = hasher.finalize();
        return Self::from_raw_hash(&digest.as_slice());
    }

    // fn from_sha2_digest(digest: sha2::Digest<OutputSize=u32>) -> Hash {
    //     panic!("Not implemented");
    // }

    pub fn from_text(slice: &str) -> Hash {
        if slice.len() != 2 * Sha256_size {
            panic!("Textal representation of a hash hash invalid size of {} instead of {}", slice.len(), 2 * Sha256_size);
        }

        let mut raw_hash = [0u8; Sha256_size];
        let mut slice_it = slice.bytes().into_iter();

        for next in raw_hash.iter_mut() {
            let hi: u8 = slice_it.next().unwrap();
            let lo: u8 = slice_it.next().unwrap();
            *next = hexcharvalue(hi, lo);
        }

        return Self::from_raw_hash(&raw_hash);
    }

    pub fn from_raw_hash(slice: &[u8]) -> Hash {
        // Note(wistrandj): Change the type to slice of 32 bytes.
        let mut hash: [u8; Sha256_size] = [0; Sha256_size];

        let mut i = 0;
        for b in slice {
            hash[i] = *b;
            i = i + 1;
        }

        if i != Sha256_size {
            // Note(wistrandj): The sha2 library provides GenericArray
            // which includes the information about hash size. This function should
            // use that type information and consume a Digest instead
            // of sha2::Sha256.
            panic!("Invalid digest size. Expect 32 but found {}", i);
        }

        Hash {
            bytes: hash
        }
    }

    pub fn from_str(slice: &str) -> Hash {
        panic!("NIY");
    }

    pub fn to_string(&self) -> String {
        return hexstring(&self.bytes);
    }

    pub fn file(file: &path::Path) -> Result<Hash, io::Error> {
        let mut hasher = sha2::Sha256::new();
        let content = fs::read(file)?;
        hasher.update(content);
        return Ok(Self::from_sha2_hasher(hasher));
    }
}

fn hexchar(c: u8) -> [char; 2] {
    let set = b"0123456789abcdef";
    let hi = c >> 4;
    let lo = (c << 4) >> 4;
    let hi = set[hi as usize] as char;
    let lo = set[lo as usize] as char;
    return [hi, lo];
}

fn hexcharvalue(hi: u8, lo: u8) -> u8 {
    // Note: Opposite of hexchar(..)
    let set = b"0123456789abcdef";
    panic!("NIY");
}

fn hexstring(slice: &[u8]) -> String {
    let mut s = String::new();
    for b in slice {
        let byte: [char; 2] = hexchar(*b);
        s.push(byte[0]);
        s.push(byte[1]);
    }
    return s;
}

fn hexslice(slice: &str) -> [u8; Sha256_size] {
    // Note(wistradj): This len() is in bytes, right?
    if slice.len() != 2 * Sha256_size {
        panic!("The slice is not exactly 64 bytes");
    }
    return [0; Sha256_size]
}

// fn main() {
//     let input = b"helloworld";
//     let mut hasher = sha2::Sha256::new();
//     hasher.update(input);
//     let result = hasher.finalize();
//     let result = result.as_slice();
//     let result = hexstring(result);
//     println!("{}", result);
// 
// 
//     let input = b"helloworld";
//     let mut hasher = sha2::Sha256::new();
//     hasher.update(input);
//     let hash: Hash = Hash::from_sha2_hasher(hasher);
//     println!("{}", hash.to_string());
// 
//     let hash = Hash::file(&path::PathBuf::from("./Cargo.toml")).unwrap();
//     println!("{}", hash.to_string());
// }
