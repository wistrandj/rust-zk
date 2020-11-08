
use sha2;
use sha2::Digest;
use std::path;
use std::fs;
use std::io;

struct Hash {
    bytes: [u8; 32]
}

impl Hash {
    fn from_sha2_hasher(hasher: sha2::Sha256) -> Hash {
        let digest = hasher.finalize();
        return Self::from_slice(&digest.as_slice());
    }

    // fn from_sha2_digest(digest: sha2::Digest<OutputSize=u32>) -> Hash {
    //     panic!("Not implemented");
    // }

    fn from_slice(slice: &[u8]) -> Hash {
        let mut hash: [u8; 32] = [0; 32];

        let mut i = 0;
        for b in slice {
            hash[i] = *b;
            i = i + 1;
        }

        if i != 32 {
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

    fn to_string(&self) -> String {
        return hexstring(&self.bytes);
    }

    fn file(file: &path::Path) -> Result<Hash, io::Error> {
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

fn hexstring(slice: &[u8]) -> String {
    let mut s = String::new();
    for b in slice {
        let byte: [char; 2] = hexchar(*b);
        s.push(byte[0]);
        s.push(byte[1]);
    }
    return s;
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
