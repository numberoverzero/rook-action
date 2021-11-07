use sha2::{Digest, Sha256};
use std::env;

fn main() {
    let args = read_all_args();
    write_debug_ln("printing all args");
    for (idx, arg) in args.iter().enumerate() {
        write_debug_ln(format!("${} is >>{}<<", idx, arg));
    }
    write_debug_ln("printing all env vars");
    for (key, value) in env::vars() {
        write_debug_ln(format!(
            "k:{} v:{} dig:{:x?}",
            key,
            value,
            hex_sha256(&value)
        ));
    }
    write_debug_ln("done");
}

fn read_all_args() -> Vec<String> {
    Vec::from_iter(env::args())
}

fn write_debug_ln<S: ToString>(x: S) {
    println!("::debug::{}", x.to_string())
}

fn hex_sha256<S: ToString>(x: &S) -> Vec<u8> {
    let mut h = Sha256::new();
    h.update(x.to_string().as_bytes());
    h.finalize().to_vec()
}
