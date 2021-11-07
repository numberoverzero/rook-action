use std::env;

fn main() {
    let args = read_all_args();
    write_debug_ln("printing all args");
    for (idx, arg) in args.iter().enumerate() {
        write_debug_ln(format!("${} is >>{}<<", idx, arg));
    }
    write_debug_ln("printing all env vars");
    for (key, value) in env::vars() {
        write_debug_ln(format!("k:{} v:{}", key, value));
    }
    write_debug_ln("done");
}


fn read_all_args() -> Vec<String> {
    Vec::from_iter(env::args())
}

fn write_debug_ln<S: ToString>(x: S) {
    println!("::debug::{}", x.to_string())
}