use hmac::{Hmac, Mac};
use reqwest::header::HeaderValue;
use sha2::Sha256;

macro_rules! unwrap_or_die {
    ( $e: expr, $msg: expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => {
                die!($msg);
            }
        }
    };
}

macro_rules! die {
    ( $arg: expr ) => {
        println!("::error::{}", $arg);
        std::process::exit(1);
    };
}

macro_rules! debug {
    ( $arg: expr ) => {
        println!("::debug::{}", $arg);
    };
}

const ENV_ENDPOINT_NAME: &'static str = "INPUT_ENDPOINT";
const ENV_SECRET_NAME: &'static str = "INPUT_SECRET";
const ENV_BODY_NAME: &'static str = "INPUT_BODY";

const ROOK_DIGEST_HEADER: &'static str = "x-rook-signature-256";
const DIGEST_PREFIX: &'static [u8; 7] = b"sha256=";

struct Input {
    endpoint: String,
    secret: Vec<u8>,
    body: Vec<u8>,
}

fn main() {
    let input = parse_inputs();
    let header = generate_header(&input.body, input.secret);

    let client = reqwest::blocking::Client::new();
    debug!(format!(">>>POST {}", input.endpoint));
    debug!(format!(">>>{}: {:?}", ROOK_DIGEST_HEADER, header));
    debug!(format!(">>>{}b body", input.body.len()));
    let res = match client
        .post(input.endpoint)
        .header(ROOK_DIGEST_HEADER, header)
        .body(input.body)
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            die!(format!("request failed: {}", e));
        }
    };
    let status = res.status();

    println!("<<<{}", status);
    match res.text() {
        Ok(body) => match body.len() {
            0 => {
                debug!("<<<(no body)");
            }
            n => {
                debug!(format!("<<<{}b body", n));
            }
        },
        Err(e) => {
            debug!(format!("<<<error decoding body: {}", e));
        }
    };

    if status.is_client_error() {
        die!("client error");
    }
    if status.is_server_error() {
        die!("server error");
    }
}

fn parse_inputs() -> Input {
    let load = |k| {
        unwrap_or_die!(
            std::env::var(k),
            format!("missing environment variable: {}", k)
        )
    };
    Input {
        endpoint: load(ENV_ENDPOINT_NAME),
        secret: load(ENV_SECRET_NAME).into_bytes(),
        body: load(ENV_BODY_NAME).into_bytes(),
    }
}

fn generate_header(body: &Vec<u8>, secret: Vec<u8>) -> HeaderValue {
    // declared for readability, these will be inlined
    const PREFIX: usize = DIGEST_PREFIX.len(); // "sha256="
    const DIGEST_BYTES: usize = 32;

    let mut bytes = [0u8; PREFIX + 2 * DIGEST_BYTES];
    bytes[..PREFIX].copy_from_slice(DIGEST_PREFIX);

    let mut mac = unwrap_or_die!(
        Hmac::<Sha256>::new_from_slice(&secret),
        "failed to initialize hmac"
    );
    mac.update(&body);
    let digest: [u8; DIGEST_BYTES] = mac.finalize().into_bytes().into();

    // copy N digest bytes into 2*N hex digest bytes
    hex_lower_into(&mut bytes[PREFIX..], &digest);

    unwrap_or_die!(
        HeaderValue::from_bytes(&bytes),
        "attempted to construct invalid header"
    )
}

/// convert each 1 src byte into its 2 byte hex representation and copy them into the output slice
fn hex_lower_into<const N: usize>(dst: &mut [u8], src: &[u8; N]) {
    for i in 0..N {
        dst[i * 2..i * 2 + 2].copy_from_slice(HEX_LOWER[src[i] as usize])
    }
}

static HEX_LOWER: [&[u8; 2]; 256] = [
    b"00", b"01", b"02", b"03", b"04", b"05", b"06", b"07", b"08", b"09", b"0a", b"0b", b"0c",
    b"0d", b"0e", b"0f", b"10", b"11", b"12", b"13", b"14", b"15", b"16", b"17", b"18", b"19",
    b"1a", b"1b", b"1c", b"1d", b"1e", b"1f", b"20", b"21", b"22", b"23", b"24", b"25", b"26",
    b"27", b"28", b"29", b"2a", b"2b", b"2c", b"2d", b"2e", b"2f", b"30", b"31", b"32", b"33",
    b"34", b"35", b"36", b"37", b"38", b"39", b"3a", b"3b", b"3c", b"3d", b"3e", b"3f", b"40",
    b"41", b"42", b"43", b"44", b"45", b"46", b"47", b"48", b"49", b"4a", b"4b", b"4c", b"4d",
    b"4e", b"4f", b"50", b"51", b"52", b"53", b"54", b"55", b"56", b"57", b"58", b"59", b"5a",
    b"5b", b"5c", b"5d", b"5e", b"5f", b"60", b"61", b"62", b"63", b"64", b"65", b"66", b"67",
    b"68", b"69", b"6a", b"6b", b"6c", b"6d", b"6e", b"6f", b"70", b"71", b"72", b"73", b"74",
    b"75", b"76", b"77", b"78", b"79", b"7a", b"7b", b"7c", b"7d", b"7e", b"7f", b"80", b"81",
    b"82", b"83", b"84", b"85", b"86", b"87", b"88", b"89", b"8a", b"8b", b"8c", b"8d", b"8e",
    b"8f", b"90", b"91", b"92", b"93", b"94", b"95", b"96", b"97", b"98", b"99", b"9a", b"9b",
    b"9c", b"9d", b"9e", b"9f", b"a0", b"a1", b"a2", b"a3", b"a4", b"a5", b"a6", b"a7", b"a8",
    b"a9", b"aa", b"ab", b"ac", b"ad", b"ae", b"af", b"b0", b"b1", b"b2", b"b3", b"b4", b"b5",
    b"b6", b"b7", b"b8", b"b9", b"ba", b"bb", b"bc", b"bd", b"be", b"bf", b"c0", b"c1", b"c2",
    b"c3", b"c4", b"c5", b"c6", b"c7", b"c8", b"c9", b"ca", b"cb", b"cc", b"cd", b"ce", b"cf",
    b"d0", b"d1", b"d2", b"d3", b"d4", b"d5", b"d6", b"d7", b"d8", b"d9", b"da", b"db", b"dc",
    b"dd", b"de", b"df", b"e0", b"e1", b"e2", b"e3", b"e4", b"e5", b"e6", b"e7", b"e8", b"e9",
    b"ea", b"eb", b"ec", b"ed", b"ee", b"ef", b"f0", b"f1", b"f2", b"f3", b"f4", b"f5", b"f6",
    b"f7", b"f8", b"f9", b"fa", b"fb", b"fc", b"fd", b"fe", b"ff",
];
